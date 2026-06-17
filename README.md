# CareerOS

An agentic job search system built in Rust. Discovers jobs, scores them against your profile, generates tailored resumes, drafts outreach, and tracks your pipeline — with human approval required before anything external happens.

---

## What It Does

```
Discovery → Qualification → Resume → Outreach → Tracking → Interview → Offer → Review
```

Each stage is a discrete agent. Agents communicate through a SQLite database and an event log. Nothing touches the outside world without explicit human approval.

---

## Principles

1. **Truth over optimization.** Never invent experience.
2. **Human approval first.** Nothing external happens automatically.
3. **Markdown is the source of truth.** Agents are text files.
4. **Measure everything.** If it cannot be measured, it cannot be improved.

---

## Prerequisites

- [Rust](https://rustup.rs/) (2024 edition)
- SQLite (bundled via `rusqlite`)

---

## Setup

```bash
git clone git@github.com:elliyeen/ai_career_agents.git
cd ai_career_agents
cargo build --release
```

The binary is at `target/release/career-os`.

For convenience, add it to your path:

```bash
export PATH="$PATH:$(pwd)/target/release"
```

---

## First Run: Fill In Your Data

Before using any agent, fill in the source-of-truth files in `docs/memory/`. These are the only source agents are allowed to draw from.

| File | What to fill in |
|------|----------------|
| `docs/memory/master-resume.md` | Your complete, verified work history |
| `docs/memory/skills-inventory.md` | Every skill with proficiency level and evidence |
| `docs/memory/career-profile.md` | Target titles, preferences, availability |
| `docs/memory/target-roles.md` | Exact role patterns you want |
| `docs/memory/target-companies.md` | Companies by tier, with career page URLs |
| `docs/memory/salary-requirements.md` | Floor, target, tradeoffs |
| `docs/memory/work-history.md` | Verified employment history for reference checks |
| `docs/memory/project-stories.md` | STAR-format stories from real projects |

Also configure:

- `docs/config/sources.md` — where the discovery agent searches
- `docs/config/scoring.md` — qualification rubric (weights and thresholds)

---

## CLI Reference

```bash
career-os status                          # Pipeline overview
career-os intake                          # Interactive: add jobs found today
career-os intake --file jobs.json         # Batch import jobs from JSON file
career-os discover                        # Find new jobs (Phase 2)
career-os qualify                         # Score unqualified jobs (interactive)
career-os qualify --auto                  # Score unqualified jobs (heuristic, no prompts)
career-os list                            # List all applications
career-os list --status submitted         # Filter by status
career-os resume                          # List jobs needing resumes
career-os resume --job-id <id>            # Generate resume for a job
career-os outreach --application-id <id> # Draft recruiter messages
career-os update --id <id> --status <s>  # Update application status
career-os interview --application-id <id># Generate interview prep
career-os review                          # Weekly metrics report → outputs/reviews/
career-os approvals                       # List pending human approvals
career-os approve <queue-id>             # Approve a resume or message
career-os reject <queue-id>              # Reject a resume or message
```

### Application Statuses

```
draft → approved → submitted → responded → interview → offer
                                         → rejected
      → withdrawn
```

---

## Daily Workflow

```bash
# Morning: add jobs found today, then score them
career-os intake
career-os qualify --auto

# Review anything that needs approval
career-os approvals
career-os approve <queue-id>

# Update statuses as responses arrive
career-os update --id <app-id> --status responded
career-os list --status submitted

# End of week
career-os review     # saves report to outputs/reviews/YYYY-WNN.md
```

---

## Scoring Rubric

Jobs are scored 0–100+ against your profile. Only jobs scoring 75+ proceed to resume generation. Jobs scoring 90+ are prioritized.

| Category | Weight |
|----------|--------|
| Role match | 25 |
| Experience match | 15 |
| Salary match | 15 |
| Industry match | 15 |
| Location match | 10 |
| Growth potential | 10 |
| Strategic value | 10 |

Bonuses: Tier 1 company (+10), referral (+10), recruiter outreach (+5), Tier 2 company (+5).

Hard disqualifications (score = 0, regardless of other factors):
- `EXCLUDED_COMPANY`
- `EXCLUDED_INDUSTRY`
- `LOCATION_INCOMPATIBLE`
- `SALARY_BELOW_FLOOR`
- `EXPERIENCE_GAP_CRITICAL` — 2+ hard missing required skills

Full rubric: `docs/config/scoring.md`

---

## Test Dataset

Ten sample job descriptions in `data/test-jobs/` cover the full range of expected outcomes:

| Job | Expected result | Tests |
|-----|----------------|-------|
| 001 — Senior TPM @ Google | QUALIFY + PRIORITY | Tier 1 bonus, exact title |
| 002 — Junior PM | DISQUALIFY | Title keyword exclusion |
| 003 — Staff TPM @ Meta | QUALIFY + PRIORITY | AI domain, remote |
| 004 — Director of Ops, Healthcare | WATCHLIST | Unknown salary, non-preferred city |
| 005 — Principal SWE @ Stripe | DISQUALIFY | EXPERIENCE_GAP_CRITICAL |
| 006 — GS-14 Program Manager | QUALIFY | Federal, clearance flag |
| 007 — Excluded Industry Co | DISQUALIFY | EXCLUDED_INDUSTRY fires first |
| 008 — Sr Ops PM @ Stripe (referral) | QUALIFY + PRIORITY | Referral bonus |
| 009 — Stale posting | DISQUALIFY | STALE_POSTING (>30 days) |
| 010 — Duplicate URL | DISQUALIFY | DUPLICATE (dedup check) |

---

## Running Tests

```bash
cargo test
```

46 tests across six suites:

- `tests/test_scoring.rs` — 13 tests for the scoring rubric
- `tests/test_no_fabrication.rs` — 4 tests verifying no skill gaps are papered over
- `tests/test_tracker.rs` — 11 tests for the application state machine and approval queue
- `src/db/mod.rs` (unit) — 4 tests for schema integrity and FK constraints
- `src/intake.rs` (unit) — 9 tests for URL canonicalization, heuristic scoring, title filtering
- `src/report.rs` (unit) — 5 tests for report generation, weakest stage detection, recommendations

---

## Project Structure

```
career-os/
├── src/
│   ├── lib.rs            # Public module exports
│   ├── main.rs           # CLI (clap)
│   ├── models.rs         # Domain types: Job, Resume, Application, Event...
│   ├── scoring.rs        # Job scoring engine
│   ├── approval.rs       # Human approval gate
│   ├── intake.rs         # Job intake: URL canonicalization, dedup, JSON import
│   ├── report.rs         # Weekly metrics report builder
│   └── db/
│       ├── mod.rs        # Database queries (rusqlite)
│       └── schema.sql    # SQLite schema
├── tests/
│   ├── test_scoring.rs
│   ├── test_no_fabrication.rs
│   └── test_tracker.rs
├── docs/
│   ├── memory/           # Source-of-truth files (fill these in)
│   ├── config/           # Scoring rubric, approved sources
│   ├── architecture.md
│   ├── domain-model.md
│   ├── workflows.md
│   └── prd.md
├── data/
│   └── test-jobs/        # 10 sample job descriptions for testing
└── outputs/              # Generated resumes, outreach, interview prep, reviews
    ├── resumes/
    ├── outreach/
    ├── interview-prep/
    └── reviews/
```

---

## Architecture

- **Runtime:** Rust + Tokio
- **CLI:** Clap
- **Database:** SQLite via rusqlite (bundled)
- **Serialization:** Serde + serde_json
- **Logging:** Tracing + tracing-subscriber
- **Config:** Markdown files in `docs/`
- **Deployment:** Single binary, runs locally

Agents do not call each other directly. All coordination flows through:

| Channel | Used for |
|---------|---------|
| SQLite state | Persistent records |
| Event log | Signals between stages |
| `outputs/` files | Resumes, messages, reports |

---

## Development Phases

| Phase | Status | Description |
|-------|--------|-------------|
| 1 | In progress | Manual agents, human-driven intake |
| 2 | Planned | Rust orchestration, automated discovery |
| 3 | Planned | Daily loops via launchd/cron |
| 4 | Planned | Dashboard |
| 5 | Planned | Self-improving workflows |

---

## Quality Rules

The system enforces these at every stage. Violations halt execution.

- No fabricated skills
- No fabricated metrics
- No fabricated employers
- No fabricated certifications
- No automatic external actions
- No application submission without human approval
- No outreach without human approval

---

## License

MIT
