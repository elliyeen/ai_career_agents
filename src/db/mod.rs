use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::path::Path;
use tracing::info;
use uuid::Uuid;

use crate::models::{Application, ApplicationStatus, ApprovalQueueItem, Event, Job, Metric, Resume};

const SCHEMA: &str = include_str!("schema.sql");

// ─────────────────────────────────────────────────────
// Db — primary handle
// ─────────────────────────────────────────────────────

pub struct Db {
    conn: Connection,
}

impl Db {
    /// Open (or create) the database at the given path and apply the schema.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open database at {}", path.display()))?;
        conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA foreign_keys = ON;")
            .context("Failed to set PRAGMAs")?;
        let db = Self { conn };
        db.migrate()?;
        info!("Database ready at {}", path.display());
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        let ddl: String = SCHEMA
            .lines()
            .filter(|l| !l.trim_start().starts_with("PRAGMA"))
            .collect::<Vec<_>>()
            .join("\n");
        self.conn.execute_batch(&ddl).context("Schema migration failed")?;
        info!("Schema applied");
        Ok(())
    }

    // ─────────────────────────────────────────────────
    // Jobs
    // ─────────────────────────────────────────────────

    pub fn insert_job(&self, job: &Job) -> Result<()> {
        self.conn.execute(
            "INSERT INTO jobs
             (id, title, company, url, source, description, location, remote,
              discovered_at, updated_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?9)",
            params![
                job.id, job.title, job.company, job.url, job.source,
                job.description, job.location, job.remote as i32,
                job.discovered_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn url_exists(&self, url: &str) -> Result<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM jobs WHERE url = ?1",
            params![url],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn update_job_qualification(
        &self,
        job_id: &str,
        score: i32,
        qualified: bool,
        reason: Option<&str>,
    ) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE jobs SET score=?1, qualified=?2, qualified_at=?3,
             disqualified_reason=?4, updated_at=?3 WHERE id=?5",
            params![score, qualified as i32, now, reason, job_id],
        )?;
        Ok(())
    }

    pub fn unqualified_jobs(&self) -> Result<Vec<Job>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, company, url, source, description, location,
             remote, discovered_at FROM jobs WHERE qualified IS NULL
             ORDER BY discovered_at ASC",
        )?;
        let jobs = stmt
            .query_map([], |row| {
                let disc: String = row.get(8)?;
                Ok(Job {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    company: row.get(2)?,
                    url: row.get(3)?,
                    source: row.get(4)?,
                    description: row.get(5)?,
                    location: row.get(6)?,
                    remote: row.get::<_, i32>(7)? != 0,
                    discovered_at: chrono::DateTime::parse_from_rfc3339(&disc)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    score: None,
                    qualified: None,
                    qualified_at: None,
                    disqualified_reason: None,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(jobs)
    }

    pub fn qualified_jobs(&self) -> Result<Vec<Job>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, company, url, score, discovered_at
             FROM jobs WHERE qualified=1 ORDER BY score DESC",
        )?;
        let jobs = stmt
            .query_map([], |row| {
                let disc: String = row.get(5)?;
                Ok(Job {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    company: row.get(2)?,
                    url: row.get(3)?,
                    source: String::new(),
                    description: String::new(),
                    location: String::new(),
                    remote: false,
                    discovered_at: chrono::DateTime::parse_from_rfc3339(&disc)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    score: row.get(4)?,
                    qualified: Some(true),
                    qualified_at: None,
                    disqualified_reason: None,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(jobs)
    }

    // ─────────────────────────────────────────────────
    // Resumes
    // ─────────────────────────────────────────────────

    pub fn insert_resume(&self, resume: &Resume) -> Result<()> {
        self.conn.execute(
            "INSERT INTO resumes
             (id, job_id, version, content, format, generated_at, approved, artifact_path)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
            params![
                resume.id, resume.job_id, resume.version, resume.content,
                resume.format, resume.generated_at.to_rfc3339(),
                resume.approved as i32, resume.artifact_path,
            ],
        )?;
        Ok(())
    }

    pub fn approve_resume(&self, resume_id: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE resumes SET approved=1, approved_at=?1 WHERE id=?2",
            params![chrono::Utc::now().to_rfc3339(), resume_id],
        )?;
        Ok(())
    }

    pub fn next_resume_version(&self, job_id: &str) -> Result<i32> {
        let v: Option<i32> = self
            .conn
            .query_row(
                "SELECT MAX(version) FROM resumes WHERE job_id=?1",
                params![job_id],
                |row| row.get(0),
            )
            .ok()
            .flatten();
        Ok(v.unwrap_or(0) + 1)
    }

    // ─────────────────────────────────────────────────
    // Applications
    // ─────────────────────────────────────────────────

    pub fn insert_application(&self, app: &Application) -> Result<()> {
        self.conn.execute(
            "INSERT INTO applications (id, job_id, resume_id, status, notes, created_at)
             VALUES (?1,?2,?3,?4,?5,?6)",
            params![
                app.id, app.job_id, app.resume_id, app.status.to_string(),
                app.notes, app.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn update_application_status(
        &self,
        app_id: &str,
        status: &ApplicationStatus,
    ) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        match status {
            ApplicationStatus::Submitted => {
                self.conn.execute(
                    "UPDATE applications SET status=?1, submitted_at=?2 WHERE id=?3",
                    params![status.to_string(), now, app_id],
                )?;
            }
            ApplicationStatus::Responded
            | ApplicationStatus::Interview
            | ApplicationStatus::Offer
            | ApplicationStatus::Rejected => {
                self.conn.execute(
                    "UPDATE applications SET status=?1, response_at=?2 WHERE id=?3",
                    params![status.to_string(), now, app_id],
                )?;
            }
            _ => {
                self.conn.execute(
                    "UPDATE applications SET status=?1 WHERE id=?2",
                    params![status.to_string(), app_id],
                )?;
            }
        }
        Ok(())
    }

    pub fn applications_by_status(&self, status: &ApplicationStatus) -> Result<Vec<Application>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, job_id, resume_id, status, submitted_at, response_at, notes, created_at
             FROM applications WHERE status=?1",
        )?;
        let apps = stmt
            .query_map(params![status.to_string()], |row| {
                let status_str: String = row.get(3)?;
                Ok(Application {
                    id: row.get(0)?,
                    job_id: row.get(1)?,
                    resume_id: row.get(2)?,
                    status: status_str.parse().unwrap_or(ApplicationStatus::Draft),
                    submitted_at: row
                        .get::<_, Option<String>>(4)?
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                        .map(|dt| dt.with_timezone(&chrono::Utc)),
                    response_at: row
                        .get::<_, Option<String>>(5)?
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                        .map(|dt| dt.with_timezone(&chrono::Utc)),
                    notes: row.get(6)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(
                        &row.get::<_, String>(7)?,
                    )
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(apps)
    }

    // ─────────────────────────────────────────────────
    // Events
    // ─────────────────────────────────────────────────

    pub fn emit_event(&self, event: &Event) -> Result<()> {
        self.conn.execute(
            "INSERT INTO events (id, name, payload, occurred_at, agent)
             VALUES (?1,?2,?3,?4,?5)",
            params![
                event.id, event.name, event.payload.to_string(),
                event.occurred_at.to_rfc3339(), event.agent,
            ],
        )?;
        Ok(())
    }

    // ─────────────────────────────────────────────────
    // Approval queue
    // ─────────────────────────────────────────────────

    pub fn enqueue_approval(&self, item: &ApprovalQueueItem) -> Result<()> {
        self.conn.execute(
            "INSERT INTO approval_queue
             (id, entity_type, entity_id, summary, artifact_path, queued_at)
             VALUES (?1,?2,?3,?4,?5,?6)",
            params![
                item.id, item.entity_type, item.entity_id, item.summary,
                item.artifact_path, item.queued_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn pending_approvals(&self) -> Result<Vec<ApprovalQueueItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, entity_type, entity_id, summary, artifact_path, queued_at
             FROM approval_queue WHERE resolved_at IS NULL ORDER BY queued_at ASC",
        )?;
        let items = stmt
            .query_map([], |row| {
                let queued: String = row.get(5)?;
                Ok(ApprovalQueueItem {
                    id: row.get(0)?,
                    entity_type: row.get(1)?,
                    entity_id: row.get(2)?,
                    summary: row.get(3)?,
                    artifact_path: row.get(4)?,
                    queued_at: chrono::DateTime::parse_from_rfc3339(&queued)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    resolved_at: None,
                    decision: None,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(items)
    }

    pub fn resolve_approval(&self, item_id: &str, decision: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE approval_queue SET resolved_at=?1, decision=?2 WHERE id=?3",
            params![chrono::Utc::now().to_rfc3339(), decision, item_id],
        )?;
        Ok(())
    }

    // ─────────────────────────────────────────────────
    // Metrics
    // ─────────────────────────────────────────────────

    pub fn current_metrics(&self) -> Result<Metric> {
        let mut m = Metric {
            id: Uuid::new_v4().to_string(),
            week_start: String::new(),
            recorded_at: chrono::Utc::now(),
            ..Default::default()
        };

        m.jobs_discovered = self
            .conn
            .query_row("SELECT COUNT(*) FROM jobs", [], |r| r.get(0))?;
        m.jobs_qualified = self
            .conn
            .query_row("SELECT COUNT(*) FROM jobs WHERE qualified=1", [], |r| r.get(0))?;
        m.applications = self.conn.query_row(
            "SELECT COUNT(*) FROM applications WHERE status != 'draft'",
            [],
            |r| r.get(0),
        )?;
        m.responses = self.conn.query_row(
            "SELECT COUNT(*) FROM applications WHERE status IN ('responded','interview','offer','rejected')",
            [],
            |r| r.get(0),
        )?;
        m.interviews = self.conn.query_row(
            "SELECT COUNT(*) FROM applications WHERE status IN ('interview','offer')",
            [],
            |r| r.get(0),
        )?;
        m.offers = self.conn.query_row(
            "SELECT COUNT(*) FROM applications WHERE status='offer'",
            [],
            |r| r.get(0),
        )?;
        m.rejections = self.conn.query_row(
            "SELECT COUNT(*) FROM applications WHERE status='rejected'",
            [],
            |r| r.get(0),
        )?;

        Ok(m)
    }
}

// ─────────────────────────────────────────────────────
// Schema tests
// ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn in_memory_db() -> Db {
        Db::open(":memory:").expect("in-memory DB should always open")
    }

    #[test]
    fn schema_applies_cleanly() {
        in_memory_db();
    }

    #[test]
    fn schema_is_idempotent() {
        let db = in_memory_db();
        // Applying migration a second time must not error
        db.migrate().expect("second migration must be idempotent");
    }

    #[test]
    fn foreign_key_constraint_enforced() {
        let db = in_memory_db();
        let result = db.conn.execute(
            "INSERT INTO resumes (id, job_id, content, generated_at, artifact_path)
             VALUES ('r1', 'nonexistent-job', '# Resume', '2024-01-01T00:00:00Z', '')",
            [],
        );
        assert!(result.is_err(), "FK constraint must reject orphaned resume");
    }

    #[test]
    fn url_deduplication_enforced_by_schema() {
        let db = in_memory_db();
        let now = chrono::Utc::now().to_rfc3339();
        db.conn
            .execute(
                "INSERT INTO jobs (id, url, title, company, source, discovered_at, updated_at)
                 VALUES ('j1','https://example.com/job/1','TPM','Acme','linkedin',?1,?1)",
                params![now],
            )
            .unwrap();
        let result = db.conn.execute(
            "INSERT INTO jobs (id, url, title, company, source, discovered_at, updated_at)
             VALUES ('j2','https://example.com/job/1','TPM','Acme','linkedin',?1,?1)",
            params![now],
        );
        assert!(result.is_err(), "Duplicate URL must be rejected by UNIQUE constraint");
    }
}
