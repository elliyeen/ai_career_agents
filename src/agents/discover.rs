/// Discover agent — imports job markdown files from data/inbox/ into the database.
///
/// Inbox format (one .md file per job):
///   **Title:** Senior Technical Program Manager
///   **Company:** Google
///   **URL:** https://careers.google.com/jobs/...
///   **Source:** linkedin
///   **Location:** Remote, US
///   **Remote:** true
///
///   ---
///
///   ## Job Description
///   [full description text]
use crate::db::Db;
use crate::models::Job;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

const INBOX_DIR: &str = "data/inbox";

#[derive(Debug)]
pub struct DiscoverResult {
    pub imported: usize,
    pub skipped: usize,
    pub errors: usize,
}

pub fn run(db: &Db) -> Result<DiscoverResult> {
    let inbox = PathBuf::from(INBOX_DIR);

    if !inbox.exists() {
        fs::create_dir_all(&inbox)
            .with_context(|| format!("Failed to create inbox dir: {}", inbox.display()))?;
        println!(
            "Created inbox directory: {}\nAdd job markdown files here, then re-run discover.",
            inbox.display()
        );
        return Ok(DiscoverResult { imported: 0, skipped: 0, errors: 0 });
    }

    let files = collect_md_files(&inbox)?;
    if files.is_empty() {
        println!(
            "Inbox is empty: {}\nAdd .md job files and re-run discover.",
            inbox.display()
        );
        return Ok(DiscoverResult { imported: 0, skipped: 0, errors: 0 });
    }

    let mut result = DiscoverResult { imported: 0, skipped: 0, errors: 0 };

    for path in &files {
        match import_file(db, path) {
            Ok(ImportOutcome::Imported(title, company)) => {
                println!("  + {} @ {}", title, company);
                result.imported += 1;
            }
            Ok(ImportOutcome::Duplicate(url)) => {
                info!("duplicate skipped: {}", url);
                result.skipped += 1;
            }
            Ok(ImportOutcome::ParseError(msg)) => {
                warn!("parse error in {}: {}", path.display(), msg);
                result.errors += 1;
            }
            Err(e) => {
                warn!("error importing {}: {}", path.display(), e);
                result.errors += 1;
            }
        }
    }

    Ok(result)
}

enum ImportOutcome {
    Imported(String, String),
    Duplicate(String),
    ParseError(String),
}

fn import_file(db: &Db, path: &Path) -> Result<ImportOutcome> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Cannot read {}", path.display()))?;

    let parsed = match parse_job_md(&content) {
        Some(p) => p,
        None => return Ok(ImportOutcome::ParseError("missing required fields (URL, Title, Company)".into())),
    };

    if db.url_exists(&parsed.url)? {
        return Ok(ImportOutcome::Duplicate(parsed.url));
    }

    let job = Job::new(
        parsed.title.clone(),
        parsed.company.clone(),
        parsed.url,
        parsed.source,
        parsed.description,
        parsed.location,
        parsed.remote,
    );

    db.insert_job(&job)?;

    // Emit JobFound event
    let event = crate::models::Event::new(
        "JobFound",
        serde_json::json!({ "job_id": job.id, "title": job.title, "company": job.company }),
        "discover",
    );
    db.emit_event(&event)?;

    Ok(ImportOutcome::Imported(parsed.title, parsed.company))
}

struct ParsedJob {
    title: String,
    company: String,
    url: String,
    source: String,
    location: String,
    remote: bool,
    description: String,
}

fn parse_job_md(content: &str) -> Option<ParsedJob> {
    let mut title: Option<String> = None;
    let mut company: Option<String> = None;
    let mut url: Option<String> = None;
    let mut source = String::from("manual");
    let mut location = String::new();
    let mut remote = false;

    // Parse bold front-matter fields: **Key:** Value
    for line in content.lines() {
        let line = line.trim();
        if let Some(val) = extract_field(line, "Title") { title = Some(val); }
        else if let Some(val) = extract_field(line, "Company") { company = Some(val); }
        else if let Some(val) = extract_field(line, "URL") { url = Some(val); }
        else if let Some(val) = extract_field(line, "Source") { source = val; }
        else if let Some(val) = extract_field(line, "Location") { location = val; }
        else if let Some(val) = extract_field(line, "Remote") {
            remote = matches!(val.to_lowercase().as_str(), "true" | "yes" | "1");
        }
    }

    // Description: everything after "## Job Description"
    let description = content
        .find("## Job Description")
        .map(|pos| content[pos + "## Job Description".len()..].trim().to_string())
        .unwrap_or_else(|| {
            // Fallback: text after the first "---" separator
            content
                .find("\n---")
                .map(|pos| content[pos + 4..].trim().to_string())
                .unwrap_or_default()
        });

    Some(ParsedJob {
        title: title?,
        company: company?,
        url: url?,
        source,
        location,
        remote,
        description,
    })
}

/// Extract value from a markdown bold field: `**Key:** Value` or `Key: Value`
fn extract_field(line: &str, key: &str) -> Option<String> {
    // Bold format: **Title:** value
    let bold_prefix = format!("**{}:**", key);
    if let Some(rest) = line.strip_prefix(&bold_prefix) {
        return Some(rest.trim().to_string());
    }
    // Plain format: Title: value
    let plain_prefix = format!("{}:", key);
    if let Some(rest) = line.strip_prefix(&plain_prefix) {
        return Some(rest.trim().to_string());
    }
    None
}

fn collect_md_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "md").unwrap_or(false))
        .filter(|p| p.file_name().map(|n| n != "README.md").unwrap_or(true))
        .collect();
    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
# Test Job

**Title:** Senior Technical Program Manager
**Company:** Acme Corp
**URL:** https://acme.com/jobs/tpm-123
**Source:** linkedin
**Location:** Remote, US
**Remote:** true

---

## Job Description

Lead cross-functional programs across engineering and product.
    "#;

    #[test]
    fn parses_standard_format() {
        let p = parse_job_md(SAMPLE).expect("should parse");
        assert_eq!(p.title, "Senior Technical Program Manager");
        assert_eq!(p.company, "Acme Corp");
        assert_eq!(p.url, "https://acme.com/jobs/tpm-123");
        assert!(p.remote);
        assert!(p.description.contains("cross-functional"));
    }

    #[test]
    fn returns_none_when_url_missing() {
        let content = "**Title:** TPM\n**Company:** Acme\n## Job Description\nDesc";
        assert!(parse_job_md(content).is_none());
    }
}
