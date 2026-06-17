/// Qualify agent — interactive terminal loop that scores and qualifies each
/// unscored job against the rubric in docs/config/scoring.md.
///
/// For each job the user enters a score (0–100).
/// Jobs scoring >= QUALIFY_THRESHOLD (75) are marked qualified.
/// Jobs scoring < 60 or explicitly disqualified are marked disqualified.
use crate::db::Db;
use crate::models::{Event, Job};
use crate::scoring;
use anyhow::Result;
use std::io::{self, Write};

pub struct QualifyResult {
    pub processed: usize,
    pub qualified: usize,
    pub disqualified: usize,
    pub skipped: usize,
}

pub fn run(db: &Db, jobs: &[Job]) -> Result<QualifyResult> {
    println!("\nRubric → docs/config/scoring.md");
    println!(
        "Thresholds: {}+ qualify | {}+ priority | <60 disqualify\n",
        scoring::QUALIFY_THRESHOLD,
        scoring::PRIORITY_THRESHOLD
    );

    let mut result = QualifyResult { processed: 0, qualified: 0, disqualified: 0, skipped: 0 };

    for (i, job) in jobs.iter().enumerate() {
        println!(
            "─── Job {}/{}: {} @ {} ───────────────────────",
            i + 1,
            jobs.len(),
            job.title,
            job.company
        );
        println!("  URL:      {}", job.url);
        println!("  Location: {}", if job.location.is_empty() { "—" } else { &job.location });
        println!("  Remote:   {}", if job.remote { "yes" } else { "no" });

        if !job.description.is_empty() {
            let preview: String = job.description.chars().take(240).collect();
            let preview = if job.description.len() > 240 { format!("{}…", preview) } else { preview };
            println!("\n  {}\n", preview);
        }

        let input = prompt("Score [0-100] / [d]isqualify / [s]kip")?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("s") || input.eq_ignore_ascii_case("skip") {
            println!("  → skipped\n");
            result.skipped += 1;
            continue;
        }

        if input.eq_ignore_ascii_case("d") || input.eq_ignore_ascii_case("disqualify") {
            let reason = prompt_with_default(
                "Disqualify reason [SCORE_BELOW_THRESHOLD/TITLE_MISMATCH/EXCLUDED_INDUSTRY/SALARY_BELOW_FLOOR/LOCATION_INCOMPATIBLE/EXPERIENCE_GAP_CRITICAL]",
                "SCORE_BELOW_THRESHOLD",
            )?;
            db.update_job_qualification(&job.id, 0, false, Some(reason.trim()))?;
            emit_qualified_event(db, job, false)?;
            println!("  ✗ disqualified — {}\n", reason.trim());
            result.disqualified += 1;
            result.processed += 1;
            continue;
        }

        match input.parse::<i32>() {
            Ok(score) if (0..=200).contains(&score) => {
                let qualified = score >= scoring::QUALIFY_THRESHOLD;
                let reason = if !qualified { Some("SCORE_BELOW_THRESHOLD") } else { None };

                db.update_job_qualification(&job.id, score, qualified, reason)?;
                emit_qualified_event(db, job, qualified)?;

                let label = if score >= scoring::PRIORITY_THRESHOLD {
                    "QUALIFY + PRIORITY"
                } else if qualified {
                    "QUALIFY"
                } else {
                    "DISQUALIFY"
                };

                println!("  {} score={} → {}\n", if qualified { "✓" } else { "✗" }, score, label);

                if qualified { result.qualified += 1; } else { result.disqualified += 1; }
                result.processed += 1;
            }
            _ => {
                println!("  ! invalid input — skipped\n");
                result.skipped += 1;
            }
        }
    }

    Ok(result)
}

fn emit_qualified_event(db: &Db, job: &Job, qualified: bool) -> Result<()> {
    let event = Event::new(
        "JobQualified",
        serde_json::json!({ "job_id": job.id, "qualified": qualified }),
        "qualify",
    );
    db.emit_event(&event)?;
    Ok(())
}

fn prompt(label: &str) -> Result<String> {
    print!("  {}: ", label);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn prompt_with_default(label: &str, default: &str) -> Result<String> {
    print!("  {} [{}]: ", label, default);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let trimmed = buf.trim().to_string();
    if trimmed.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(trimmed)
    }
}
