#!/usr/bin/env python3
"""
weekly_review.py — Generate a weekly pipeline metrics report.

Reads the database, calculates conversion rates for each pipeline
stage, identifies the weakest stage, and writes a Markdown report
to outputs/reviews/YYYY-WW.md.

Usage:
    python scripts/weekly_review.py [--db data/career.db]
"""
import argparse
import sqlite3
from datetime import datetime, timezone, timedelta
from pathlib import Path

# ─────────────────────────────────────────────────────
# Database helpers
# ─────────────────────────────────────────────────────

def fetch_metrics(conn: sqlite3.Connection) -> dict:
    m = {}
    def q(sql): return conn.execute(sql).fetchone()[0]

    m["jobs_discovered"]  = q("SELECT COUNT(*) FROM jobs")
    m["jobs_qualified"]   = q("SELECT COUNT(*) FROM jobs WHERE qualified=1")
    m["resumes_generated"]= q("SELECT COUNT(*) FROM resumes")
    m["resumes_approved"] = q("SELECT COUNT(*) FROM resumes WHERE approved=1")
    m["applications"]     = q("SELECT COUNT(*) FROM applications WHERE status != 'draft'")
    m["responses"]        = q("SELECT COUNT(*) FROM applications WHERE status IN ('responded','interview','offer','rejected')")
    m["interviews"]       = q("SELECT COUNT(*) FROM applications WHERE status IN ('interview','offer')")
    m["offers"]           = q("SELECT COUNT(*) FROM applications WHERE status='offer'")
    m["rejections"]       = q("SELECT COUNT(*) FROM applications WHERE status='rejected'")
    m["pending_approvals"]= q("SELECT COUNT(*) FROM approval_queue WHERE resolved_at IS NULL")

    return m

def conversion(num: int, denom: int) -> str:
    if denom == 0:
        return "—"
    return f"{num/denom*100:.0f}%"

def weakest_stage(m: dict) -> str:
    """Identify the pipeline stage with the lowest conversion rate."""
    rates = {
        "Discovery → Qualification": (m["jobs_qualified"], m["jobs_discovered"]),
        "Qualification → Application": (m["applications"], m["jobs_qualified"]),
        "Application → Response": (m["responses"], m["applications"]),
        "Response → Interview": (m["interviews"], m["responses"]),
        "Interview → Offer": (m["offers"], m["interviews"]),
    }
    worst_stage = None
    worst_rate = 1.0
    for stage, (num, denom) in rates.items():
        if denom > 0:
            rate = num / denom
            if rate < worst_rate:
                worst_rate = rate
                worst_stage = stage
    return worst_stage or "Insufficient data"

# ─────────────────────────────────────────────────────
# Report generation
# ─────────────────────────────────────────────────────

def build_report(m: dict, week_label: str) -> str:
    weak = weakest_stage(m)

    lines = [
        f"# Weekly Review — {week_label}",
        f"",
        f"Generated: {datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M UTC')}",
        f"",
        f"---",
        f"",
        f"## Pipeline Snapshot",
        f"",
        f"| Stage                 | Count | Conversion |",
        f"|----------------------|-------|------------|",
        f"| Jobs discovered       | {m['jobs_discovered']} | — |",
        f"| Jobs qualified        | {m['jobs_qualified']} | {conversion(m['jobs_qualified'], m['jobs_discovered'])} |",
        f"| Resumes generated     | {m['resumes_generated']} | {conversion(m['resumes_generated'], m['jobs_qualified'])} |",
        f"| Resumes approved      | {m['resumes_approved']} | {conversion(m['resumes_approved'], m['resumes_generated'])} |",
        f"| Applications sent     | {m['applications']} | {conversion(m['applications'], m['resumes_approved'])} |",
        f"| Responses received    | {m['responses']} | {conversion(m['responses'], m['applications'])} |",
        f"| Interviews scheduled  | {m['interviews']} | {conversion(m['interviews'], m['responses'])} |",
        f"| Offers received       | {m['offers']} | {conversion(m['offers'], m['interviews'])} |",
        f"| Rejections            | {m['rejections']} | — |",
        f"",
        f"---",
        f"",
        f"## Pending Actions",
        f"",
        f"- Items awaiting approval: **{m['pending_approvals']}**",
        f"",
        f"  Run `career-os approvals` to review.",
        f"",
        f"---",
        f"",
        f"## Weakest Stage",
        f"",
        f"**{weak}**",
        f"",
        f"This is where the pipeline is losing the most candidates.",
        f"",
        f"---",
        f"",
        f"## Recommendations",
        f"",
        f"Based on the weakest stage above:",
        f"",
    ]

    if weak.startswith("Discovery"):
        lines += [
            "- Increase search frequency or add new sources (see docs/config/sources.md).",
            "- Review qualification criteria — may be too strict.",
            "- Add target companies to direct monitoring list.",
        ]
    elif weak.startswith("Qualification"):
        lines += [
            "- Review scoring rubric — is the threshold too high?",
            "- Check skills-inventory.md for gaps blocking qualification.",
            "- Consider broader role keywords in target-roles.md.",
        ]
    elif weak.startswith("Application"):
        lines += [
            "- Review approval queue — are resumes sitting unapproved?",
            "- Reduce time from resume generation to submission.",
            "- Check for stale applications (submitted > 14 days, no response).",
        ]
    elif weak.startswith("Response"):
        lines += [
            "- Review outreach quality — are messages sounding human?",
            "- Audit resume ATS keyword density.",
            "- Check application timing — are you applying within 48 hours of posting?",
            "- Consider sending follow-up messages (5 business days after submission).",
        ]
    elif weak.startswith("Interview"):
        lines += [
            "- Review STAR stories in project-stories.md — do they match target roles?",
            "- Run mock interviews before each scheduled interview.",
            "- Review 30-60-90 plans — are they specific to the company?",
        ]
    else:
        lines += ["- No clear recommendation yet. Need more data."]

    lines += [
        "",
        "---",
        "",
        "## Next Steps",
        "",
        "- [ ] Review pending approvals: `career-os approvals`",
        "- [ ] Score new jobs: `python scripts/score_jobs.py`",
        "- [ ] Check for follow-up opportunities (5+ business days since submission)",
        "",
    ]

    return "\n".join(lines)

# ─────────────────────────────────────────────────────
# Main
# ─────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description="Generate weekly pipeline review")
    parser.add_argument("--db", default="data/career.db")
    args = parser.parse_args()

    db_path = Path(args.db)
    if not db_path.exists():
        print(f"Database not found: {db_path}")
        return

    conn = sqlite3.connect(db_path)
    m = fetch_metrics(conn)
    conn.close()

    today = datetime.now()
    # ISO week label: YYYY-W##
    week_label = today.strftime("%Y-W%V")

    report = build_report(m, week_label)

    out_dir = Path("outputs/reviews")
    out_dir.mkdir(parents=True, exist_ok=True)
    out_path = out_dir / f"{week_label}.md"
    out_path.write_text(report)

    print(report)
    print(f"\nReport saved: {out_path}")


if __name__ == "__main__":
    main()
