# Architecture

CareerOS — v0.1

---

## Overview

CareerOS is a single-binary, agent-based pipeline that runs locally.

Agents are Markdown-defined workflows executed by a Rust runtime.

No agent calls another agent directly. All coordination happens through shared state, events, and database records.

---

## Runtime

```
Language:       Rust
Async runtime:  Tokio
CLI:            Clap
Serialization:  Serde + serde_json
Logging:        Tracing + tracing-subscriber
Storage:        SQLite via sqlx
Config:         Markdown files in docs/agents/
```

---

## Binary Layout

```
career-os (single binary)
├── cli        — command parsing (clap)
├── agents     — agent runner + registry
├── db         — SQLite schema + queries
├── events     — event bus (in-process)
├── scheduler  — Tokio task scheduler
└── outputs    — file writers (resume, outreach, reports)
```

---

## Agent Execution Model

Each agent is a discrete, stateless unit.

```
Input:   Database records + Markdown config
Process: Rust function (deterministic where possible)
Output:  Database record + artifact file in outputs/
Event:   Written to events table on completion
```

Agents never share memory. They read from the database. They write to the database. They emit events.

---

## Communication

Agents communicate through four channels only:

| Channel         | Used for                                      |
|-----------------|-----------------------------------------------|
| State (SQLite)  | Persistent job, application, resume records   |
| Events (SQLite) | Signals that trigger downstream agents        |
| Artifacts       | Files written to outputs/ (resumes, messages) |
| DB records      | Logs, metrics, audit trail                    |

---

## Storage Schema (high-level)

```
jobs             — discovered job postings
applications     — job + resume + status
resumes          — tailored resume versions
outreach         — recruiter messages, follow-ups
interviews       — scheduled, prep artifacts
events           — append-only event log
metrics          — weekly conversion snapshots
logs             — execution audit trail
```

Full schema: see domain-model.md

---

## Scheduler

Tokio-based scheduler runs agents on a defined cadence.

| Agent               | Cadence        |
|---------------------|----------------|
| Discovery           | Daily          |
| Qualification       | On: JobFound   |
| Resume              | On: JobQualified |
| Outreach            | On: ResumeGenerated |
| Weekly Review       | Weekly (Sunday) |

All schedules are configurable. No agent runs automatically against external systems without human approval.

---

## Human Approval Gate

Nothing external happens automatically.

Before any outreach message or application is submitted:

1. Agent writes draft to `outputs/`
2. Event emitted: `ApplicationPrepared` or `OutreachDrafted`
3. System pauses. Human reviews file.
4. Human confirms via CLI: `career-os approve <id>`
5. Agent marks record as approved. Execution resumes.

---

## Logging

Every agent execution writes a structured log entry.

```
timestamp   — RFC 3339
agent       — agent name
duration_ms — execution time
status      — success | failure | skipped
error       — error message if failed
output_path — path to artifact if produced
```

Logs are written to: `logs/YYYY-MM-DD.jsonl`

---

## Error Handling

| Error type        | Behavior                          |
|-------------------|-----------------------------------|
| Network failure   | Retry up to 3 times, then log     |
| Parse failure     | Log + flag for human review       |
| Validation failure| Stop execution, do not write output |
| DB failure        | Panic with structured error       |

---

## Deployment

Phase 1: Local binary, run manually.

Phase 2: Scheduled via system cron or launchd.

Phase 3: Dashboard (future).

No cloud infrastructure required. SQLite file is the entire system state.

---

## Non-goals

- No web server in Phase 1–2.
- No multi-user support.
- No cloud sync.
- No automatic external actions of any kind.
