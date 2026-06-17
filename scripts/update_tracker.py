#!/usr/bin/env python3
"""
update_tracker.py — Update application status from the command line.

Usage:
    python scripts/update_tracker.py --id <app-id> --status <status>
    python scripts/update_tracker.py --list
    python scripts/update_tracker.py --list --status submitted

Valid statuses:
    draft | approved | submitted | responded | interview | offer | rejected | withdrawn
"""
import argparse
import sqlite3
from datetime import datetime, timezone
from pathlib import Path

VALID_STATUSES = {
    "draft", "approved", "submitted", "responded",
    "interview", "offer", "rejected", "withdrawn"
}

TERMINAL_STATUSES = {"offer", "rejected", "withdrawn"}

STATUS_EMOJI = {
    "draft": "📝",
    "approved": "✅",
    "submitted": "📤",
    "responded": "💬",
    "interview": "🎤",
    "offer": "🎯",
    "rejected": "❌",
    "withdrawn": "↩️",
}


def get_conn(db_path: str) -> sqlite3.Connection:
    path = Path(db_path)
    if not path.exists():
        raise FileNotFoundError(f"Database not found: {path}")
    return sqlite3.connect(path)


def list_applications(conn: sqlite3.Connection, filter_status: str | None = None) -> None:
    sql = """
        SELECT a.id, a.status, a.created_at, j.title, j.company, j.url
        FROM applications a
        JOIN jobs j ON a.job_id = j.id
    """
    params = []
    if filter_status:
        sql += " WHERE a.status = ?"
        params.append(filter_status)
    sql += " ORDER BY a.created_at DESC"

    rows = conn.execute(sql, params).fetchall()
    if not rows:
        print("No applications found.")
        return

    print(f"\n{'ID':<10} {'Status':<12} {'Title':<35} {'Company':<20} {'Date'}")
    print("─" * 90)
    for row in rows:
        app_id, status, created, title, company, url = row
        emoji = STATUS_EMOJI.get(status, "")
        print(f"{app_id[:8]:<10} {emoji} {status:<10} {title[:33]:<35} {company[:18]:<20} {created[:10]}")


def update_status(conn: sqlite3.Connection, app_id: str, new_status: str) -> None:
    if new_status not in VALID_STATUSES:
        raise ValueError(f"Invalid status '{new_status}'. Valid: {', '.join(sorted(VALID_STATUSES))}")

    # Fetch current status
    row = conn.execute(
        "SELECT id, status FROM applications WHERE id LIKE ?", (f"{app_id}%",)
    ).fetchone()
    if not row:
        raise ValueError(f"Application not found: {app_id}")

    full_id, current_status = row
    if current_status == new_status:
        print(f"Application {full_id[:8]} is already '{new_status}'.")
        return

    now = datetime.now(timezone.utc).isoformat()

    # Update status and set timestamp fields
    if new_status == "submitted":
        conn.execute(
            "UPDATE applications SET status=?, submitted_at=? WHERE id=?",
            (new_status, now, full_id)
        )
    elif new_status in ("responded", "interview", "offer", "rejected"):
        conn.execute(
            "UPDATE applications SET status=?, response_at=? WHERE id=?",
            (new_status, now, full_id)
        )
    else:
        conn.execute(
            "UPDATE applications SET status=? WHERE id=?",
            (new_status, full_id)
        )

    conn.commit()
    emoji = STATUS_EMOJI.get(new_status, "")
    print(f"Updated: {full_id[:8]} → {emoji} {new_status}")

    if new_status == "interview":
        print("\n  Next step: generate interview prep")
        print(f"  Run: career-os interview --application-id {full_id}")

    if new_status == "offer":
        print("\n  Congratulations. Record offer details in application notes.")
        print("  Do not accept until you have reviewed: compensation, equity, start date.")


def main():
    parser = argparse.ArgumentParser(description="Update application tracker")
    parser.add_argument("--db", default="data/career.db")
    parser.add_argument("--id", help="Application ID (prefix is fine)")
    parser.add_argument("--status", help="New status or filter status for --list")
    parser.add_argument("--list", action="store_true", help="List applications")
    args = parser.parse_args()

    conn = get_conn(args.db)

    if args.list:
        list_applications(conn, filter_status=args.status)
    elif args.id and args.status:
        update_status(conn, args.id, args.status)
    else:
        parser.print_help()

    conn.close()


if __name__ == "__main__":
    main()
