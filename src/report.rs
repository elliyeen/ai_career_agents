/// Weekly pipeline review report.
///
/// Replaces scripts/weekly_review.py.
/// Reads the database, computes conversion rates, identifies the
/// weakest stage, and writes a Markdown report to outputs/reviews/.
use anyhow::Result;
use chrono::{Datelike, Utc};
use std::fmt::Write as FmtWrite;
use std::path::Path;

use crate::db::Db;

// ─────────────────────────────────────────────────────
// Metrics snapshot
// ─────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct WeeklyStats {
    pub jobs_discovered: i64,
    pub jobs_qualified: i64,
    pub resumes_generated: i64,
    pub resumes_approved: i64,
    pub applications: i64,
    pub responses: i64,
    pub interviews: i64,
    pub offers: i64,
    pub rejections: i64,
    pub pending_approvals: i64,
}

fn conversion(num: i64, denom: i64) -> String {
    if denom == 0 {
        "—".to_string()
    } else {
        format!("{:.0}%", num as f64 / denom as f64 * 100.0)
    }
}

// ─────────────────────────────────────────────────────
// Weakest stage
// ─────────────────────────────────────────────────────

fn weakest_stage(s: &WeeklyStats) -> &'static str {
    let stages: &[(&str, i64, i64)] = &[
        ("Discovery → Qualification",    s.jobs_qualified,   s.jobs_discovered),
        ("Qualification → Application",  s.applications,     s.jobs_qualified),
        ("Application → Response",       s.responses,        s.applications),
        ("Response → Interview",         s.interviews,       s.responses),
        ("Interview → Offer",            s.offers,           s.interviews),
    ];

    let mut worst: Option<(&str, f64)> = None;
    for &(stage, num, denom) in stages {
        if denom > 0 {
            let rate = num as f64 / denom as f64;
            match worst {
                None => worst = Some((stage, rate)),
                Some((_, wr)) if rate < wr => worst = Some((stage, rate)),
                _ => {}
            }
        }
    }

    worst.map(|(s, _)| s).unwrap_or("Insufficient data")
}

// ─────────────────────────────────────────────────────
// Report builder
// ─────────────────────────────────────────────────────

fn build_report(stats: &WeeklyStats, week_label: &str) -> String {
    let now = Utc::now().format("%Y-%m-%d %H:%M UTC");
    let weak = weakest_stage(stats);
    let mut out = String::new();

    let _ = writeln!(out, "# Weekly Review — {week_label}");
    let _ = writeln!(out);
    let _ = writeln!(out, "Generated: {now}");
    let _ = writeln!(out);
    let _ = writeln!(out, "---");
    let _ = writeln!(out);
    let _ = writeln!(out, "## Pipeline Snapshot");
    let _ = writeln!(out);
    let _ = writeln!(out, "| Stage                 | Count | Conversion |");
    let _ = writeln!(out, "|----------------------|-------|------------|");
    let _ = writeln!(out, "| Jobs discovered       | {} | — |",         stats.jobs_discovered);
    let _ = writeln!(out, "| Jobs qualified        | {} | {} |",        stats.jobs_qualified,   conversion(stats.jobs_qualified,   stats.jobs_discovered));
    let _ = writeln!(out, "| Resumes generated     | {} | {} |",        stats.resumes_generated, conversion(stats.resumes_generated, stats.jobs_qualified));
    let _ = writeln!(out, "| Resumes approved      | {} | {} |",        stats.resumes_approved,  conversion(stats.resumes_approved,  stats.resumes_generated));
    let _ = writeln!(out, "| Applications sent     | {} | {} |",        stats.applications,      conversion(stats.applications,      stats.resumes_approved));
    let _ = writeln!(out, "| Responses received    | {} | {} |",        stats.responses,         conversion(stats.responses,         stats.applications));
    let _ = writeln!(out, "| Interviews scheduled  | {} | {} |",        stats.interviews,        conversion(stats.interviews,        stats.responses));
    let _ = writeln!(out, "| Offers received       | {} | {} |",        stats.offers,            conversion(stats.offers,            stats.interviews));
    let _ = writeln!(out, "| Rejections            | {} | — |",         stats.rejections);
    let _ = writeln!(out);
    let _ = writeln!(out, "---");
    let _ = writeln!(out);
    let _ = writeln!(out, "## Pending Actions");
    let _ = writeln!(out);
    let _ = writeln!(out, "- Items awaiting approval: **{}**", stats.pending_approvals);
    let _ = writeln!(out);
    let _ = writeln!(out, "  Run `career-os approvals` to review.");
    let _ = writeln!(out);
    let _ = writeln!(out, "---");
    let _ = writeln!(out);
    let _ = writeln!(out, "## Weakest Stage");
    let _ = writeln!(out);
    let _ = writeln!(out, "**{weak}**");
    let _ = writeln!(out);
    let _ = writeln!(out, "This is where the pipeline is losing the most candidates.");
    let _ = writeln!(out);
    let _ = writeln!(out, "---");
    let _ = writeln!(out);
    let _ = writeln!(out, "## Recommendations");
    let _ = writeln!(out);
    let _ = writeln!(out, "Based on the weakest stage above:");
    let _ = writeln!(out);

    let recs: &[&str] = match weak {
        w if w.starts_with("Discovery") => &[
            "- Increase search frequency or add new sources (see docs/config/sources.md).",
            "- Review qualification criteria — may be too strict.",
            "- Add target companies to direct monitoring list.",
        ],
        w if w.starts_with("Qualification") => &[
            "- Review scoring rubric — is the threshold too high?",
            "- Check skills-inventory.md for gaps blocking qualification.",
            "- Consider broader role keywords in target-roles.md.",
        ],
        w if w.starts_with("Application") => &[
            "- Review approval queue — are resumes sitting unapproved?",
            "- Reduce time from resume generation to submission.",
            "- Check for stale applications (submitted > 14 days, no response).",
        ],
        w if w.starts_with("Response") => &[
            "- Review outreach quality — are messages sounding human?",
            "- Audit resume ATS keyword density.",
            "- Check application timing — are you applying within 48 hours of posting?",
            "- Consider sending follow-up messages (5 business days after submission).",
        ],
        w if w.starts_with("Interview") => &[
            "- Review STAR stories in project-stories.md — do they match target roles?",
            "- Run mock interviews before each scheduled interview.",
            "- Review 30-60-90 plans — are they specific to the company?",
        ],
        _ => &["- No clear recommendation yet. Need more pipeline data."],
    };

    for rec in recs {
        let _ = writeln!(out, "{rec}");
    }

    let _ = writeln!(out);
    let _ = writeln!(out, "---");
    let _ = writeln!(out);
    let _ = writeln!(out, "## Next Steps");
    let _ = writeln!(out);
    let _ = writeln!(out, "- [ ] Review pending approvals: `career-os approvals`");
    let _ = writeln!(out, "- [ ] Score new jobs: `career-os qualify --auto`");
    let _ = writeln!(out, "- [ ] Check for follow-up opportunities (5+ business days since submission)");
    let _ = writeln!(out);

    out
}

// ─────────────────────────────────────────────────────
// Public entry point
// ─────────────────────────────────────────────────────

/// Run the weekly review: compute stats, print report, save to outputs/reviews/.
pub fn run_review(db: &Db) -> Result<()> {
    let stats = db.weekly_stats()?;

    let today = Utc::now();
    let iso = today.iso_week();
    let week_label = format!("{}-W{:02}", iso.year(), iso.week());

    let report = build_report(&stats, &week_label);

    print!("{}", report);

    let out_dir = Path::new("outputs/reviews");
    std::fs::create_dir_all(out_dir)?;
    let out_path = out_dir.join(format!("{week_label}.md"));
    std::fs::write(&out_path, &report)?;
    println!("Report saved: {}", out_path.display());

    Ok(())
}

// ─────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn full_stats() -> WeeklyStats {
        WeeklyStats {
            jobs_discovered: 20,
            jobs_qualified: 10,
            resumes_generated: 8,
            resumes_approved: 6,
            applications: 5,
            responses: 3,
            interviews: 2,
            offers: 1,
            rejections: 2,
            pending_approvals: 1,
        }
    }

    #[test]
    fn conversion_formats_correctly() {
        assert_eq!(conversion(1, 4), "25%");
        assert_eq!(conversion(0, 0), "—");
        assert_eq!(conversion(3, 3), "100%");
    }

    #[test]
    fn weakest_stage_finds_correct_stage() {
        let mut s = full_stats();
        // Make Application → Response the worst (0 responses out of 5 applications)
        s.responses = 0;
        assert_eq!(weakest_stage(&s), "Application → Response");
    }

    #[test]
    fn weakest_stage_returns_insufficient_data_when_empty() {
        let s = WeeklyStats::default();
        assert_eq!(weakest_stage(&s), "Insufficient data");
    }

    #[test]
    fn report_contains_all_sections() {
        let s = full_stats();
        let report = build_report(&s, "2026-W25");
        assert!(report.contains("# Weekly Review — 2026-W25"));
        assert!(report.contains("## Pipeline Snapshot"));
        assert!(report.contains("## Weakest Stage"));
        assert!(report.contains("## Recommendations"));
        assert!(report.contains("## Next Steps"));
        assert!(report.contains("career-os approvals"));
    }

    #[test]
    fn report_recommendation_matches_weakest_stage() {
        let mut s = full_stats();
        // offers=0, interviews=2 → Interview → Offer rate = 0/2 = 0%, worst stage
        s.offers = 0;
        let report = build_report(&s, "2026-W25");
        assert!(
            report.contains("STAR stories"),
            "Report should recommend reviewing STAR stories when Interview → Offer is weakest"
        );
    }

    #[test]
    fn report_references_rust_command_not_python() {
        let s = full_stats();
        let report = build_report(&s, "2026-W25");
        assert!(
            !report.contains("python scripts/"),
            "Report must not reference Python scripts: {}",
            report
        );
        assert!(
            report.contains("career-os qualify"),
            "Report must reference the Rust CLI command"
        );
    }
}
