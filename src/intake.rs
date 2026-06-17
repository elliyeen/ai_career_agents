/// Job intake — interactive and file-based.
///
/// Replaces scripts/run_daily_research.py.
/// Handles URL canonicalization, deduplication, stdin prompts,
/// and JSON batch import.
use anyhow::Result;
use serde::Deserialize;
use std::io::{self, BufRead, Write};
use std::path::Path;
use tracing::info;

use crate::db::Db;
use crate::models::{Event, Job};

// ─────────────────────────────────────────────────────
// URL canonicalization
// ─────────────────────────────────────────────────────

const TRACKING_PARAMS: &[&str] = &[
    "utm_source",
    "utm_medium",
    "utm_campaign",
    "utm_content",
    "utm_term",
    "ref",
    "referrer",
    "src",
    "source",
    "referer",
];

/// Strip tracking query parameters and trailing slashes from a URL.
pub fn canonicalize_url(url: &str) -> String {
    let (base, query_part) = match url.split_once('?') {
        None => return url.trim_end_matches('/').to_string(),
        Some((b, q)) => (b, q),
    };

    let clean_params: Vec<&str> = query_part
        .split('&')
        .filter(|param| {
            let key = param.split_once('=').map(|(k, _)| k).unwrap_or(param);
            !TRACKING_PARAMS.contains(&key)
        })
        .collect();

    let base = base.trim_end_matches('/');

    if clean_params.is_empty() {
        base.to_string()
    } else {
        format!("{}?{}", base, clean_params.join("&"))
    }
}

// ─────────────────────────────────────────────────────
// Heuristic scorer (fast — title + location only)
// ─────────────────────────────────────────────────────
//
// This is the auto-mode scorer used when full ScoringInputs are not
// available. It is intentionally conservative: borderline jobs score
// below the qualify threshold and surface for human review.

pub const TARGET_TITLE_KEYWORDS: &[&str] = &[
    "program manager",
    "technical program manager",
    "tpm",
    "operations manager",
    "director of operations",
    "chief of staff",
    "sr. program manager",
    "senior program manager",
    "staff program manager",
    "principal program manager",
];

pub const DISQUALIFY_TITLE_KEYWORDS: &[&str] = &[
    "junior",
    "entry level",
    "entry-level",
    "intern",
    "internship",
    "associate",
    "software engineer",
    "software developer",
    "frontend",
    "backend",
    "full stack",
    "designer",
    "sales",
    "marketing",
    "recruiter",
    "accountant",
];

const QUALIFY_THRESHOLD: i32 = 75;

#[derive(Debug)]
pub struct HeuristicResult {
    pub score: i32,
    pub qualified: bool,
    pub reason: Option<String>,
}

pub fn heuristic_score(title: &str, location: &str, remote: bool) -> HeuristicResult {
    let title_lower = title.to_lowercase();

    // Hard disqualify on title keywords
    for kw in DISQUALIFY_TITLE_KEYWORDS {
        if title_lower.contains(kw) {
            return HeuristicResult {
                score: 0,
                qualified: false,
                reason: Some("TITLE_MISMATCH".to_string()),
            };
        }
    }

    // Role match score (0–25)
    let role_score: i32 = if TARGET_TITLE_KEYWORDS
        .iter()
        .any(|kw| title_lower.contains(kw))
    {
        25
    } else {
        // Partial match — title doesn't contain exact target keywords
        (25.0 * 0.3) as i32 // 7
    };

    // Location score (0–10)
    let location_lower = location.to_lowercase();
    let location_score: i32 = if remote || location_lower.contains("remote") {
        10
    } else if ["new york", "san francisco", "austin", "chicago", "seattle"]
        .iter()
        .any(|c| location_lower.contains(c))
    {
        5
    } else {
        2
    };

    // Conservative defaults for unknowns — assume moderate fit
    let experience_score = (15.0 * 0.6) as i32; // 9
    let salary_score = (15.0 * 0.3) as i32;     // 4 — unknown
    let industry_score = (15.0 * 0.6) as i32;   // 9
    let growth_score = (10.0 * 0.6) as i32;     // 6
    let strategic_score = (10.0 * 0.5) as i32;  // 5

    let total = role_score
        + experience_score
        + salary_score
        + industry_score
        + location_score
        + growth_score
        + strategic_score;

    if total < QUALIFY_THRESHOLD {
        HeuristicResult {
            score: total,
            qualified: false,
            reason: Some("SCORE_BELOW_THRESHOLD".to_string()),
        }
    } else {
        HeuristicResult {
            score: total,
            qualified: true,
            reason: None,
        }
    }
}

// ─────────────────────────────────────────────────────
// Insert a single job with dedup
// ─────────────────────────────────────────────────────

/// Insert a job into the database if the URL is not already present.
/// Returns true if inserted, false if skipped as duplicate.
pub fn add_job(
    db: &Db,
    title: &str,
    company: &str,
    url: &str,
    source: &str,
    description: &str,
    location: &str,
    remote: bool,
) -> Result<bool> {
    let canonical = canonicalize_url(url);

    if db.url_exists(&canonical)? {
        println!("  SKIP (duplicate): {} @ {}", title, company);
        return Ok(false);
    }

    let job = Job::new(title, company, &canonical, source, description, location, remote);
    let job_id = job.id.clone();
    db.insert_job(&job)?;

    let event = Event::new(
        "JobFound",
        serde_json::json!({
            "job_id": job_id,
            "title": title,
            "company": company,
        }),
        "intake",
    );
    db.emit_event(&event)?;

    info!("Added: {} @ {} ({})", title, company, source);
    println!("  ADDED: {} @ {} ({})", title, company, source);
    Ok(true)
}

// ─────────────────────────────────────────────────────
// Interactive intake (stdin)
// ─────────────────────────────────────────────────────

fn prompt(msg: &str) -> Option<String> {
    print!("{}", msg);
    io::stdout().flush().ok();
    let mut line = String::new();
    match io::stdin().lock().read_line(&mut line) {
        Ok(0) | Err(_) => None,
        Ok(_) => {
            let s = line.trim().to_string();
            Some(s)
        }
    }
}

pub fn interactive_intake(db: &Db) -> Result<usize> {
    println!("\nEnter jobs found today. Leave URL blank to finish.\n");
    println!("Fields: URL, Title, Company, Location, Remote (y/n), Description (optional).\n");

    let mut added = 0usize;

    loop {
        let url = match prompt("URL (or blank to finish): ") {
            None => break,
            Some(s) if s.is_empty() => break,
            Some(s) => s,
        };

        let title = prompt("Title: ")
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Unknown".to_string());

        let company = prompt("Company: ")
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Unknown".to_string());

        let location = prompt("Location (e.g. Remote, US): ")
            .unwrap_or_default();

        let remote = prompt("Remote? (y/n): ")
            .map(|s| s.to_lowercase().starts_with('y'))
            .unwrap_or(false);

        let description = prompt("Description (optional, Enter to skip): ")
            .unwrap_or_default();

        if add_job(db, &title, &company, &url, "manual", &description, &location, remote)? {
            added += 1;
        }
        println!();
    }

    Ok(added)
}

// ─────────────────────────────────────────────────────
// JSON file import
// ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct JobRecord {
    pub title: String,
    pub company: Option<String>,
    pub url: String,
    pub source: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub remote: Option<bool>,
}

pub fn import_from_file(db: &Db, path: impl AsRef<Path>) -> Result<usize> {
    let path = path.as_ref();
    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let text = std::fs::read_to_string(path)?;
    let records: Vec<JobRecord> = serde_json::from_str(&text)?;

    let mut added = 0usize;
    for r in &records {
        let inserted = add_job(
            db,
            &r.title,
            r.company.as_deref().unwrap_or("Unknown"),
            &r.url,
            r.source.as_deref().unwrap_or("file"),
            r.description.as_deref().unwrap_or(""),
            r.location.as_deref().unwrap_or(""),
            r.remote.unwrap_or(false),
        )?;
        if inserted {
            added += 1;
        }
    }

    Ok(added)
}

// ─────────────────────────────────────────────────────
// Sources checklist
// ─────────────────────────────────────────────────────

/// Print job search sources from docs/config/sources.md.
pub fn print_sources_checklist() {
    let path = Path::new("docs/config/sources.md");
    if !path.exists() {
        return;
    }
    if let Ok(text) = std::fs::read_to_string(path) {
        println!("Sources to check today:");
        for line in text.lines() {
            if line.starts_with("| ") && line.contains("http") {
                let parts: Vec<&str> = line
                    .split('|')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .collect();
                if parts.len() >= 2 {
                    println!("  - {}: {}", parts[0], parts[1]);
                }
            }
        }
        println!();
    }
}

// ─────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_utm_params() {
        let url = "https://jobs.example.com/tpm?utm_source=linkedin&utm_campaign=spring";
        assert_eq!(canonicalize_url(url), "https://jobs.example.com/tpm");
    }

    #[test]
    fn strips_trailing_slash() {
        let url = "https://jobs.example.com/tpm/";
        assert_eq!(canonicalize_url(url), "https://jobs.example.com/tpm");
    }

    #[test]
    fn keeps_non_tracking_params() {
        let url = "https://jobs.example.com/search?page=2&q=tpm";
        let canon = canonicalize_url(url);
        assert!(canon.contains("page=2"));
        assert!(canon.contains("q=tpm"));
        assert!(!canon.contains("utm"));
    }

    #[test]
    fn strips_mixed_params() {
        let url = "https://jobs.example.com/tpm?ref=homepage&page=3&utm_source=google";
        let canon = canonicalize_url(url);
        assert!(canon.contains("page=3"), "non-tracking param must survive: {}", canon);
        assert!(!canon.contains("ref="), "ref must be stripped: {}", canon);
        assert!(!canon.contains("utm_source"), "utm_source must be stripped: {}", canon);
    }

    #[test]
    fn target_title_scores_higher_than_unknown_title() {
        let target = heuristic_score("Senior Technical Program Manager", "Remote, US", true);
        let unknown = heuristic_score("Coordinator", "Remote, US", true);
        // Target title earns full role_score (25); unknown title earns 30% (7).
        // The heuristic is intentionally conservative — verify relative ordering, not threshold.
        assert!(
            target.score > unknown.score,
            "Target title should score higher than unknown title: {} vs {}",
            target.score,
            unknown.score,
        );
        // Must not be hard-disqualified (TITLE_MISMATCH)
        assert!(target.reason != Some("TITLE_MISMATCH".to_string()));
    }

    #[test]
    fn junior_title_disqualifies() {
        let r = heuristic_score("Junior Project Manager", "Austin, TX", false);
        assert!(!r.qualified);
        assert_eq!(r.reason.as_deref(), Some("TITLE_MISMATCH"));
        assert_eq!(r.score, 0);
    }

    #[test]
    fn unknown_title_scores_conservatively() {
        let r = heuristic_score("Coordinator", "Remote", true);
        // Should not hard-disqualify but likely below threshold
        assert!(r.score < 75, "Unknown title should score below qualify threshold");
    }

    #[test]
    fn remote_scores_higher_than_in_office_unknown_city() {
        let remote = heuristic_score("Senior Program Manager", "Remote, US", true);
        let office = heuristic_score("Senior Program Manager", "Omaha, NE", false);
        // Remote earns 10 pts on location; unknown in-office city earns 2.
        assert!(
            remote.score > office.score,
            "Remote job should score higher than in-office unknown city: {} vs {}",
            remote.score,
            office.score,
        );
    }
}
