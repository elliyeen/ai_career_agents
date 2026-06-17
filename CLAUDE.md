# CareerOS

AI Workforce for Job Discovery

---

# Mission

Build an agentic system that continuously improves the job search process.

The objective is not to automate everything.

The objective is to build repeatable loops.

---

# Principles

1. Truth over optimization.

Never invent experience.

---

2. Human approval first.

Nothing external happens automatically.

---

3. Markdown is the source of truth.

Agents are text files.

---

4. Memory efficiency matters.

Prefer Rust over Python.

---

5. Improve the system.

Agents improve workflows, not just outputs.

---

6. Measure everything.

If it cannot be measured it cannot be improved.

---

# Architecture

Runtime:

Rust

Database:

SQLite

Configuration:

Markdown

Scheduler:

Tokio

Serialization:

Serde

Logging:

Tracing

CLI:

Clap

Testing:

Cargo test

---

# Repository Structure

```
career-os/
docs/
  agents/
  memory/
data/
outputs/
logs/
src/
tests/
examples/
```

---

# Agent Pattern

Every agent contains:

- Mission
- Inputs
- Outputs
- Rules
- Workflow
- Quality checklist
- Failure modes
- Example output

---

# Agent Communication

Agents never call each other directly.

Communication occurs through:

- State
- Events
- Artifacts
- Database records

---

# Event Model

- JobFound
- JobQualified
- ResumeGenerated
- ApplicationPrepared
- InterviewScheduled
- OfferReceived
- WeeklyReviewGenerated

---

# State Machine

```
Discovery
↓
Qualification
↓
Resume
↓
Outreach
↓
Tracking
↓
Interview
↓
Offer
↓
Retrospective
```

---

# Logging

Every execution records:

- Timestamp
- Agent
- Duration
- Status
- Errors
- Output location

---

# Retry Rules

Network failures: 3 retries

Parsing failures: Human review

Validation failures: Stop execution

---

# Quality Rules

- No fabricated skills.
- No fabricated metrics.
- No fake certifications.
- No fake employers.
- No hidden automation.

---

# Test Philosophy

- Small tests
- Deterministic tests
- No hidden state
- Repeatable outputs

---

# Continuous Improvement

Weekly Review Agent evaluates:

- Conversion rates
- Resume quality
- Interview performance
- Weakest stage
- Recommendations

---

# Goal

Build a professional career operating system that compounds over time.
