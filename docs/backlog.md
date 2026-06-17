# Backlog

CareerOS — v0.1

Status: [ ] pending · [~] in progress · [x] done

---

## Phase 1 — Manual Agents (Markdown-driven, no Rust)

Goal: Build working workflows using Claude + Markdown before writing any code. Validate the process. Validate the output quality.

### Memory Setup
- [ ] Create `docs/memory/resume-master.md` — complete verified experience
- [ ] Create `docs/memory/experience.md` — skills, roles, metrics, verified facts
- [ ] Create `docs/memory/candidate-profile.md` — target roles, preferences, constraints
- [ ] Create `docs/memory/qualification-criteria.md` — scoring rules

### Agent Definitions
- [ ] Write `docs/agents/discovery.md`
- [ ] Write `docs/agents/qualification.md`
- [ ] Write `docs/agents/resume.md`
- [ ] Write `docs/agents/outreach.md`
- [ ] Write `docs/agents/interview-prep.md`
- [ ] Write `docs/agents/weekly-review.md`

### Manual Runs
- [ ] Run discovery agent manually against 2–3 sources
- [ ] Qualify first batch of jobs
- [ ] Generate first tailored resume
- [ ] Draft first outreach message
- [ ] Validate output quality against quality checklist

### Measurement
- [ ] Define `outputs/` folder conventions
- [ ] Set up manual metrics log in `data/metrics.csv`

---

## Phase 2 — Rust Orchestration

Goal: Replace manual Claude runs with a compiled binary. Agents become Rust functions. SQLite stores all state.

### Project Setup
- [ ] Initialize Cargo project (`cargo init`)
- [ ] Add dependencies: tokio, sqlx, serde, clap, tracing
- [ ] Write SQLite schema (`src/db/schema.sql`)
- [ ] Write migrations

### CLI
- [ ] `career-os run discovery`
- [ ] `career-os run qualification`
- [ ] `career-os run resume --job-id <id>`
- [ ] `career-os run outreach --application-id <id>`
- [ ] `career-os run interview-prep --application-id <id>`
- [ ] `career-os run review`
- [ ] `career-os approve resume <id>`
- [ ] `career-os approve outreach <id>`
- [ ] `career-os update application <id> --status <status>`
- [ ] `career-os list jobs`
- [ ] `career-os list applications`

### Agent Implementations
- [ ] Discovery agent (Rust)
- [ ] Qualification agent (Rust)
- [ ] Resume agent (Rust + LLM call)
- [ ] Outreach agent (Rust + LLM call)
- [ ] Interview Prep agent (Rust + LLM call)
- [ ] Weekly Review agent (Rust)

### Event System
- [ ] Event table + writer
- [ ] Event poller per agent
- [ ] Event-triggered agent execution

### Tests
- [ ] Unit tests for qualification scoring
- [ ] Unit tests for deduplication
- [ ] Integration tests for each agent
- [ ] Contract tests for all event payloads
- [ ] Fixture files in `tests/fixtures/`

---

## Phase 3 — Daily Loops

Goal: System runs on a schedule without manual triggering.

- [ ] Tokio-based scheduler
- [ ] Daily discovery run
- [ ] Triggered qualification on `JobFound`
- [ ] Weekly review on Sunday
- [ ] Structured log rotation (`logs/YYYY-MM-DD.jsonl`)
- [ ] Alert on execution failure (local notification or log)

---

## Phase 4 — Dashboard

Goal: Visual interface for reviewing pipeline state without using the CLI.

- [ ] Define dashboard requirements
- [ ] Choose rendering approach (TUI vs web)
- [ ] Pipeline funnel view (jobs → applications → interviews → offers)
- [ ] Weekly metrics chart
- [ ] Application status board
- [ ] Pending approvals queue

---

## Phase 5 — Self-Improving Workflows

Goal: Weekly Review agent identifies weakest stage and proposes agent improvements.

- [ ] Define improvement proposal format
- [ ] Weekly Review agent writes improvement suggestions to `outputs/reviews/`
- [ ] Human reviews suggestions and applies changes to agent Markdown configs
- [ ] Track improvement history in `data/improvements.md`
- [ ] Measure conversion rate change week-over-week

---

## Icebox

Items deferred with no phase assignment.

- [ ] Email integration for tracking recruiter responses
- [ ] Calendar integration for interview scheduling
- [ ] LinkedIn scraping (pending legal/ToS review)
- [ ] Multi-profile support (different resume identities for different role types)
- [ ] Encrypted SQLite storage
