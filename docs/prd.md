# Product Requirements Document (PRD)

# CareerOS

Version: 0.1

---

# Vision

CareerOS is an agentic operating system that continuously discovers opportunities, evaluates jobs, tailors resumes, drafts outreach, prepares interviews, tracks results, and improves itself over time.

The system should minimize repetitive work while maintaining human approval before any external action.

---

# Objectives

Primary Goal:

Obtain interviews and offers for high-fit positions.

Secondary Goals:

* Maintain a single source of truth.
* Eliminate manual tracking.
* Prevent hallucinations and fabricated experience.
* Continuously improve conversion rates.

---

# Success Metrics

Weekly:

* Jobs discovered
* Qualified jobs
* Applications submitted
* Recruiter responses
* Interview requests
* Offers received

Conversion metrics:

* Jobs → Applications
* Applications → Responses
* Responses → Interviews
* Interviews → Offers

---

# Users

Primary User:

Professional with experience in:

* Program Management
* Technical Program Management
* Operations
* AI Systems
* Process Improvement

---

# Functional Requirements

## Job Discovery

Must:

* Search approved sources.
* Deduplicate jobs.
* Store URLs.
* Store timestamps.

---

## Resume Generation

Must:

* Use only verified experience.
* Tailor wording.
* Preserve truthfulness.
* Produce ATS-friendly output.

---

## Outreach

Must:

* Generate recruiter messages.
* Generate follow-ups.
* Generate thank-you notes.

---

## Interview Preparation

Must:

* Research companies.
* Generate STAR stories.
* Produce question lists.
* Produce 30-60-90 plans.

---

## Metrics

Must:

Track:

* Applications
* Responses
* Interviews
* Offers
* Rejections

---

# Nonfunctional Requirements

Performance:

* Memory efficient
* Single binary deployment
* Fast startup

Reliability:

* Idempotent jobs
* Retry support
* Structured logs

Maintainability:

* Markdown-first
* Human readable
* Git versioned

Security:

* Secrets stored in .env
* No automatic external actions
* Human approval required

---

# Constraints

Never:

* Fabricate experience.
* Apply automatically.
* Send messages automatically.
* Modify external systems without approval.

---

# Architecture

UI:

Future dashboard

Runtime:

Rust

Storage:

SQLite

Configuration:

Markdown

Observability:

Structured logs

Scheduler:

Tokio

State Machine:

LangGraph-style workflow

---

# Milestones

Phase 1

Manual agents

Phase 2

Rust orchestration

Phase 3

Daily loops

Phase 4

Dashboard

Phase 5

Self-improving workflows
