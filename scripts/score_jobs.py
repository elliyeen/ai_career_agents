#!/usr/bin/env python3
"""
score_jobs.py — Score unqualified jobs in the database.

Reads unqualified jobs from the SQLite database, prompts for scoring
inputs (or reads from a config), applies the scoring rubric, and
writes qualification decisions back to the database.

Usage:
    python scripts/score_jobs.py [--db data/career.db] [--auto]

    --db PATH    Path to SQLite database (default: data/career.db)
    --auto       Non-interactive mode: uses heuristics only (no prompts)

Output:
    - Updates jobs table: score, qualified, disqualified_reason
    - Prints summary to stdout
    - Logs to logs/score_jobs_YYYY-MM-DD.jsonl
"""
import argparse
import json
import logging
import sqlite3
import sys
from datetime import datetime, timezone
from pathlib import Path
from uuid import uuid4

# ─────────────────────────────────────────────────────
# Configuration
# ─────────────────────────────────────────────────────

QUALIFY_THRESHOLD = 75
PRIORITY_THRESHOLD = 90

# Load from docs/config/scoring.md in a real implementation.
# These are the rubric weights.
WEIGHTS = {
    "role": 25,
    "experience": 15,
    "salary": 15,
    "industry": 15,
    "location": 10,
    "growth": 10,
    "strategic": 10,
}

# ─────────────────────────────────────────────────────
# Logging setup
# ─────────────────────────────────────────────────────

log_dir = Path("logs")
log_dir.mkdir(exist_ok=True)

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [score_jobs] %(levelname)s %(message)s",
    handlers=[
        logging.StreamHandler(),
        logging.FileHandler(log_dir / f"score_jobs_{datetime.now().strftime('%Y-%m-%d')}.log"),
    ],
)
log = logging.getLogger(__name__)

# ─────────────────────────────────────────────────────
# Database helpers
# ─────────────────────────────────────────────────────

def get_unqualified_jobs(conn: sqlite3.Connection) -> list[dict]:
    cur = conn.execute(
        "SELECT id, title, company, url, description, location, remote "
        "FROM jobs WHERE qualified IS NULL ORDER BY discovered_at ASC"
    )
    cols = [d[0] for d in cur.description]
    return [dict(zip(cols, row)) for row in cur.fetchall()]


def write_qualification(
    conn: sqlite3.Connection,
    job_id: str,
    score: int,
    qualified: bool,
    reason: str | None,
) -> None:
    conn.execute(
        """UPDATE jobs
           SET score=?, qualified=?, qualified_at=?, disqualified_reason=?
           WHERE id=?""",
        (
            score,
            1 if qualified else 0,
            datetime.now(timezone.utc).isoformat(),
            reason,
            job_id,
        ),
    )
    conn.commit()


def emit_event(conn: sqlite3.Connection, name: str, payload: dict, agent: str) -> None:
    conn.execute(
        "INSERT INTO events (id, name, payload, occurred_at, agent) VALUES (?,?,?,?,?)",
        (str(uuid4()), name, json.dumps(payload), datetime.now(timezone.utc).isoformat(), agent),
    )
    conn.commit()

# ─────────────────────────────────────────────────────
# Heuristic scorer (auto mode — no prompts)
# ─────────────────────────────────────────────────────

# Load target role keywords from docs/memory/target-roles.md in a real implementation.
# Placeholder lists — fill in from your actual target-roles.md.
TARGET_TITLE_KEYWORDS = [
    "program manager", "technical program manager", "tpm",
    "operations manager", "director of operations", "chief of staff",
]
DISQUALIFY_TITLE_KEYWORDS = [
    "junior", "entry level", "intern", "associate",
    "software engineer", "developer", "designer", "sales", "marketing",
]
EXCLUDED_INDUSTRIES = []  # fill from target-companies.md

def heuristic_score(job: dict) -> tuple[int, bool, str | None]:
    """
    Fast heuristic scoring for auto mode.
    Returns (score, qualified, disqualify_reason).
    This is intentionally conservative — borderline jobs surface for human review.
    """
    title_lower = job["title"].lower()

    # Hard disqualify on title keywords
    for kw in DISQUALIFY_TITLE_KEYWORDS:
        if kw in title_lower:
            return 0, False, "TITLE_MISMATCH"

    # Role match score
    role_score = 0
    for kw in TARGET_TITLE_KEYWORDS:
        if kw in title_lower:
            role_score = WEIGHTS["role"]
            break
    else:
        # Partial match — title doesn't contain exact keywords
        role_score = int(WEIGHTS["role"] * 0.3)

    # Location score
    location_lower = job["location"].lower()
    remote = bool(job["remote"])
    if remote or "remote" in location_lower:
        location_score = WEIGHTS["location"]
    elif any(c in location_lower for c in ["new york", "san francisco", "austin", "chicago"]):
        location_score = int(WEIGHTS["location"] * 0.5)
    else:
        location_score = int(WEIGHTS["location"] * 0.25)

    # Conservative defaults for unknowns (salary, industry, experience, growth, strategic)
    # In manual mode, the operator fills these in interactively.
    experience_score = int(WEIGHTS["experience"] * 0.6)  # assume moderate fit
    salary_score = int(WEIGHTS["salary"] * 0.3)           # unknown = low
    industry_score = int(WEIGHTS["industry"] * 0.6)       # assume acceptable
    growth_score = int(WEIGHTS["growth"] * 0.6)
    strategic_score = int(WEIGHTS["strategic"] * 0.5)

    total = (
        role_score + experience_score + salary_score +
        industry_score + location_score + growth_score + strategic_score
    )

    if total < QUALIFY_THRESHOLD:
        return total, False, "SCORE_BELOW_THRESHOLD"
    return total, True, None

# ─────────────────────────────────────────────────────
# Main
# ─────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description="Score unqualified jobs")
    parser.add_argument("--db", default="data/career.db", help="SQLite database path")
    parser.add_argument("--auto", action="store_true", help="Non-interactive heuristic mode")
    args = parser.parse_args()

    db_path = Path(args.db)
    if not db_path.exists():
        log.error(f"Database not found: {db_path}. Run `career-os discover` first.")
        sys.exit(1)

    conn = sqlite3.connect(db_path)
    jobs = get_unqualified_jobs(conn)

    if not jobs:
        print("No unqualified jobs. Nothing to score.")
        return

    print(f"\nScoring {len(jobs)} unqualified job(s)...\n")

    qualified_count = 0
    disqualified_count = 0

    for job in jobs:
        print(f"  [{job['id'][:8]}] {job['title']} @ {job['company']}")

        if args.auto:
            total, qualified, reason = heuristic_score(job)
        else:
            # Interactive mode: print job details, ask for score components
            print(f"    URL: {job['url']}")
            print(f"    Location: {job['location']} | Remote: {bool(job['remote'])}")
            print()
            print("    Enter scores (0 = skip/unknown, or press Enter to use heuristic):")
            try:
                total, qualified, reason = heuristic_score(job)
                print(f"    Heuristic score: {total}")
                override = input("    Override? Enter new score or press Enter to accept: ").strip()
                if override:
                    total = int(override)
                    qualified = total >= QUALIFY_THRESHOLD
                    reason = None if qualified else "SCORE_BELOW_THRESHOLD"
            except (KeyboardInterrupt, EOFError):
                print("\n    Skipping.")
                continue

        write_qualification(conn, job["id"], total, qualified, reason)

        if qualified:
            qualified_count += 1
            status = f"QUALIFIED ({total})"
            emit_event(conn, "JobQualified", {"job_id": job["id"], "score": total}, "score_jobs")
        else:
            disqualified_count += 1
            status = f"DISQUALIFIED — {reason}"

        print(f"    → {status}\n")
        log.info(f"job_id={job['id']} title={job['title']} score={total} qualified={qualified} reason={reason}")

    print(f"Done. Qualified: {qualified_count} | Disqualified: {disqualified_count}")
    print(f"Next step: career-os resume")

    conn.close()


if __name__ == "__main__":
    main()
