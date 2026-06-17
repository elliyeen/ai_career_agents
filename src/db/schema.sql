-- CareerOS SQLite Schema v2
-- State machine: discovery → qualification → resume → outreach → tracking → interview → offer → retrospective
-- Run via db::Db::open() — idempotent (all IF NOT EXISTS)

PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

-- ── jobs ──────────────────────────────────────────────────────────────────────
-- One row per unique posting. URL is the deduplication key.

CREATE TABLE IF NOT EXISTS jobs (
    id                  TEXT    PRIMARY KEY,          -- UUID v4
    url                 TEXT    NOT NULL UNIQUE,
    title               TEXT    NOT NULL,
    company             TEXT    NOT NULL,
    location            TEXT    NOT NULL DEFAULT '',
    remote              INTEGER NOT NULL DEFAULT 0,   -- boolean
    description         TEXT    NOT NULL DEFAULT '',
    source              TEXT    NOT NULL,             -- linkedin | indeed | builtin | wellfound | dice | usajobs | google | company | manual
    score               INTEGER,                      -- 0–100+; NULL = not yet scored
    qualified           INTEGER,                      -- NULL=pending 1=yes 0=no
    qualified_at        TEXT,                         -- ISO-8601
    disqualified_reason TEXT,
    notes               TEXT    NOT NULL DEFAULT '',
    discovered_at       TEXT    NOT NULL,             -- ISO-8601
    updated_at          TEXT    NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_jobs_qualified   ON jobs (qualified);
CREATE INDEX IF NOT EXISTS idx_jobs_score       ON jobs (score);
CREATE INDEX IF NOT EXISTS idx_jobs_company     ON jobs (company);
CREATE INDEX IF NOT EXISTS idx_jobs_discovered  ON jobs (discovered_at);

-- ── resumes ───────────────────────────────────────────────────────────────────
-- Tailored resume generated for a specific job. Never fabricated.

CREATE TABLE IF NOT EXISTS resumes (
    id            TEXT    PRIMARY KEY,
    job_id        TEXT    NOT NULL REFERENCES jobs (id) ON DELETE CASCADE,
    version       INTEGER NOT NULL DEFAULT 1,
    content       TEXT    NOT NULL,                   -- ATS-friendly plain text or markdown
    format        TEXT    NOT NULL DEFAULT 'txt',     -- txt | pdf
    generated_at  TEXT    NOT NULL,
    approved      INTEGER NOT NULL DEFAULT 0,         -- 0=pending 1=approved
    approved_at   TEXT,
    artifact_path TEXT    NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_resumes_job_id ON resumes (job_id);

-- ── applications ──────────────────────────────────────────────────────────────
-- Records each application attempt. One per job (ideally).

CREATE TABLE IF NOT EXISTS applications (
    id            TEXT PRIMARY KEY,
    job_id        TEXT NOT NULL REFERENCES jobs (id) ON DELETE CASCADE,
    resume_id     TEXT REFERENCES resumes (id),
    status        TEXT NOT NULL DEFAULT 'draft'
                       CHECK (status IN (
                           'draft', 'approved', 'submitted', 'responded',
                           'interview', 'offer', 'rejected', 'withdrawn'
                       )),
    submitted_at  TEXT,
    response_at   TEXT,
    notes         TEXT NOT NULL DEFAULT '',
    created_at    TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_applications_status ON applications (status);
CREATE INDEX IF NOT EXISTS idx_applications_job_id ON applications (job_id);

-- ── outreach ──────────────────────────────────────────────────────────────────
-- Recruiter messages, follow-ups, thank-you notes. Sent only on human approval.

CREATE TABLE IF NOT EXISTS outreach (
    id              TEXT PRIMARY KEY,
    application_id  TEXT NOT NULL REFERENCES applications (id) ON DELETE CASCADE,
    message_type    TEXT NOT NULL CHECK (message_type IN ('initial', 'follow_up', 'thank_you')),
    content         TEXT NOT NULL,
    drafted_at      TEXT NOT NULL,
    approved        INTEGER NOT NULL DEFAULT 0,
    approved_at     TEXT,
    sent_at         TEXT,
    artifact_path   TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_outreach_application_id ON outreach (application_id);

-- ── contacts ──────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS contacts (
    id            TEXT PRIMARY KEY,
    name          TEXT NOT NULL,
    title         TEXT NOT NULL DEFAULT '',
    company       TEXT NOT NULL DEFAULT '',
    email         TEXT,
    linkedin_url  TEXT,
    source        TEXT NOT NULL DEFAULT '',
    notes         TEXT NOT NULL DEFAULT '',
    created_at    TEXT NOT NULL
);

-- ── interviews ────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS interviews (
    id                TEXT PRIMARY KEY,
    application_id    TEXT NOT NULL REFERENCES applications (id) ON DELETE CASCADE,
    scheduled_at      TEXT NOT NULL,
    round             INTEGER NOT NULL DEFAULT 1,
    format            TEXT NOT NULL DEFAULT '',
    interviewer       TEXT NOT NULL DEFAULT '',
    company_research  TEXT NOT NULL DEFAULT '',
    star_stories      TEXT NOT NULL DEFAULT '',
    question_list     TEXT NOT NULL DEFAULT '',
    plan_30_60_90     TEXT NOT NULL DEFAULT '',
    generated_at      TEXT,
    artifact_path     TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_interviews_application_id ON interviews (application_id);

-- ── offers ────────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS offers (
    id           TEXT PRIMARY KEY,
    application_id TEXT NOT NULL REFERENCES applications (id) ON DELETE CASCADE,
    base_salary  INTEGER,
    equity       TEXT,
    bonus        TEXT,
    details      TEXT,
    received_at  TEXT NOT NULL,
    deadline_at  TEXT,
    decision     TEXT CHECK (decision IN ('accepted', 'declined', 'negotiating', 'pending'))
);

-- ── events ────────────────────────────────────────────────────────────────────
-- Append-only event log. Never updated. Never deleted.

CREATE TABLE IF NOT EXISTS events (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    payload     TEXT NOT NULL DEFAULT '{}',
    occurred_at TEXT NOT NULL,
    agent       TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_events_name     ON events (name);
CREATE INDEX IF NOT EXISTS idx_events_occurred ON events (occurred_at);

-- ── approval_queue ────────────────────────────────────────────────────────────
-- Items pending human review before any external action.

CREATE TABLE IF NOT EXISTS approval_queue (
    id            TEXT PRIMARY KEY,
    entity_type   TEXT NOT NULL,        -- resume | outreach | application
    entity_id     TEXT NOT NULL,
    summary       TEXT NOT NULL,
    artifact_path TEXT NOT NULL,
    queued_at     TEXT NOT NULL,
    resolved_at   TEXT,
    decision      TEXT                  -- approved | rejected
);

CREATE INDEX IF NOT EXISTS idx_approval_resolved ON approval_queue (resolved_at);

-- ── metrics ───────────────────────────────────────────────────────────────────
-- Weekly pipeline snapshots.

CREATE TABLE IF NOT EXISTS metrics (
    id               TEXT PRIMARY KEY,
    week_start       TEXT NOT NULL UNIQUE,
    jobs_discovered  INTEGER NOT NULL DEFAULT 0,
    jobs_qualified   INTEGER NOT NULL DEFAULT 0,
    applications     INTEGER NOT NULL DEFAULT 0,
    responses        INTEGER NOT NULL DEFAULT 0,
    interviews       INTEGER NOT NULL DEFAULT 0,
    offers           INTEGER NOT NULL DEFAULT 0,
    rejections       INTEGER NOT NULL DEFAULT 0,
    recorded_at      TEXT NOT NULL
);

-- ── logs ──────────────────────────────────────────────────────────────────────
-- Execution audit trail for every agent run.

CREATE TABLE IF NOT EXISTS logs (
    id          TEXT PRIMARY KEY,
    agent       TEXT NOT NULL,
    started_at  TEXT NOT NULL,
    duration_ms INTEGER NOT NULL DEFAULT 0,
    status      TEXT NOT NULL CHECK (status IN ('success', 'failure', 'skipped')),
    error       TEXT,
    output_path TEXT
);
