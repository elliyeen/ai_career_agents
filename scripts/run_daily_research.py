#!/usr/bin/env python3
"""
run_daily_research.py — Daily job discovery runner.

Orchestrates the discovery loop:
1. Prints today's search queries from docs/config/sources.md
2. Records any manually-added jobs (from stdin or file)
3. Runs score_jobs.py in auto mode
4. Prints a summary of what needs human attention

This is the script you run each morning. The discovery agent
(web scraping) is Phase 2. Today, this script is your intake form.

Usage:
    python scripts/run_daily_research.py [--db data/career.db] [--file jobs.json]

    --file PATH   Import jobs from a JSON file instead of interactive mode
"""
import argparse
import json
import sqlite3
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
from uuid import uuid4

SOURCES_CONFIG = Path("docs/config/sources.md")
DB_DEFAULT = "data/career.db"


def get_conn(db_path: str) -> sqlite3.Connection:
    path = Path(db_path)
    if not path.exists():
        print(f"No database found at {path}. Run `career-os status` to initialize.")
        sys.exit(1)
    return sqlite3.connect(path)


def url_exists(conn: sqlite3.Connection, url: str) -> bool:
    count = conn.execute("SELECT COUNT(*) FROM jobs WHERE url=?", (url,)).fetchone()[0]
    return count > 0


def strip_tracking_params(url: str) -> str:
    """Remove common tracking parameters from job URLs."""
    from urllib.parse import urlparse, urlencode, parse_qs, urlunparse
    parsed = urlparse(url)
    tracking_params = {
        "utm_source", "utm_medium", "utm_campaign", "utm_content", "utm_term",
        "ref", "referrer", "src", "source", "referer",
    }
    params = {k: v for k, v in parse_qs(parsed.query).items() if k not in tracking_params}
    clean_query = urlencode(params, doseq=True)
    clean = parsed._replace(query=clean_query, path=parsed.path.rstrip("/"))
    return urlunparse(clean)


def add_job(conn: sqlite3.Connection, job: dict) -> bool:
    """Insert a job if the URL is not a duplicate. Returns True if inserted."""
    url = strip_tracking_params(job["url"])
    if url_exists(conn, url):
        print(f"  SKIP (duplicate): {job['title']} @ {job['company']}")
        return False

    job_id = str(uuid4())
    conn.execute(
        """INSERT INTO jobs (id, title, company, url, source, description, location,
           remote, discovered_at)
           VALUES (?,?,?,?,?,?,?,?,?)""",
        (
            job_id,
            job["title"],
            job.get("company", "Unknown"),
            url,
            job.get("source", "manual"),
            job.get("description", ""),
            job.get("location", ""),
            1 if job.get("remote", False) else 0,
            datetime.now(timezone.utc).isoformat(),
        ),
    )
    conn.execute(
        "INSERT INTO events (id, name, payload, occurred_at, agent) VALUES (?,?,?,?,?)",
        (
            str(uuid4()),
            "JobFound",
            json.dumps({"job_id": job_id, "title": job["title"], "company": job.get("company")}),
            datetime.now(timezone.utc).isoformat(),
            "run_daily_research",
        ),
    )
    conn.commit()
    print(f"  ADDED: {job['title']} @ {job.get('company')} ({job.get('source', 'manual')})")
    return True


def interactive_intake(conn: sqlite3.Connection) -> int:
    """Prompt the user to paste job info interactively."""
    added = 0
    print("\nEnter jobs found today. Press Ctrl+D when done.\n")
    print("For each job, enter: URL, Title, Company, Location, Remote (y/n)")
    print("Or press Enter to skip a field.\n")

    while True:
        try:
            url = input("URL (or blank to finish): ").strip()
        except (EOFError, KeyboardInterrupt):
            break
        if not url:
            break

        title = input("Title: ").strip() or "Unknown"
        company = input("Company: ").strip() or "Unknown"
        location = input("Location (e.g., Remote, US): ").strip()
        remote_raw = input("Remote? (y/n): ").strip().lower()
        remote = remote_raw.startswith("y")
        description = input("Paste description (optional, Enter to skip): ").strip()

        job = {
            "url": url,
            "title": title,
            "company": company,
            "location": location,
            "remote": remote,
            "description": description,
            "source": "manual",
        }

        if add_job(conn, job):
            added += 1
        print()

    return added


def import_from_file(conn: sqlite3.Connection, file_path: str) -> int:
    """
    Import jobs from a JSON file.
    Expected format: list of job objects with fields matching the jobs table.
    """
    path = Path(file_path)
    if not path.exists():
        print(f"File not found: {path}")
        return 0

    jobs = json.loads(path.read_text())
    if not isinstance(jobs, list):
        jobs = [jobs]

    added = 0
    for job in jobs:
        if add_job(conn, job):
            added += 1
    return added


def print_daily_summary(conn: sqlite3.Connection) -> None:
    total = conn.execute("SELECT COUNT(*) FROM jobs").fetchone()[0]
    unscored = conn.execute("SELECT COUNT(*) FROM jobs WHERE qualified IS NULL").fetchone()[0]
    qualified = conn.execute("SELECT COUNT(*) FROM jobs WHERE qualified=1").fetchone()[0]
    pending = conn.execute(
        "SELECT COUNT(*) FROM approval_queue WHERE resolved_at IS NULL"
    ).fetchone()[0]

    print("\n─── Daily Summary ───────────────────────────────────")
    print(f"  Total jobs in database: {total}")
    print(f"  Awaiting scoring:       {unscored}")
    print(f"  Qualified:              {qualified}")
    print(f"  Pending approvals:      {pending}")
    print()

    if unscored > 0:
        print(f"  Next: python scripts/score_jobs.py --auto")
    if pending > 0:
        print(f"  Next: career-os approvals")
    print()


def main():
    parser = argparse.ArgumentParser(description="Daily job research runner")
    parser.add_argument("--db", default=DB_DEFAULT)
    parser.add_argument("--file", help="JSON file of jobs to import")
    args = parser.parse_args()

    print(f"\n─── CareerOS Daily Research — {datetime.now().strftime('%Y-%m-%d')} ─────")

    if SOURCES_CONFIG.exists():
        print(f"\nSources: {SOURCES_CONFIG}")
        print("Check these today:")
        for line in SOURCES_CONFIG.read_text().splitlines():
            if line.startswith("| ") and "http" in line:
                parts = [p.strip() for p in line.split("|") if p.strip()]
                if len(parts) >= 2:
                    print(f"  - {parts[0]}: {parts[1]}")
    print()

    conn = get_conn(args.db)

    if args.file:
        added = import_from_file(conn, args.file)
    else:
        added = interactive_intake(conn)

    print(f"\nAdded {added} new job(s).")
    print_daily_summary(conn)
    conn.close()


if __name__ == "__main__":
    main()
