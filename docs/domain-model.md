# Domain Model

CareerOS — v0.1

---

## Core Entities

### Job

A discovered job posting.

```
id            UUID
title         String
company       String
url           String (unique)
source        String          — site scraped from
description   String
location      String
remote        Boolean
discovered_at DateTime
qualified     Boolean | null  — null = not yet evaluated
qualified_at  DateTime | null
disqualified_reason String | null
```

---

### Resume

A tailored resume version tied to a specific job.

```
id            UUID
job_id        UUID → Job
version       Integer         — increments on regeneration
content       String          — ATS-friendly plain text
format        Enum(txt, pdf)
generated_at  DateTime
approved      Boolean         — human must approve before use
approved_at   DateTime | null
artifact_path String          — path in outputs/
```

Rules:
- Only verified experience may appear.
- No fabricated metrics.
- No fabricated employers.
- No fabricated certifications.

---

### Application

A submission record linking a Job and a Resume.

```
id            UUID
job_id        UUID → Job
resume_id     UUID → Resume
status        Enum(draft, approved, submitted, responded, interview, offer, rejected, withdrawn)
submitted_at  DateTime | null
response_at   DateTime | null
notes         String
```

---

### OutreachMessage

A recruiter message, follow-up, or thank-you note.

```
id            UUID
application_id UUID → Application
type          Enum(initial, follow_up, thank_you)
content       String
draft_at      DateTime
approved      Boolean
approved_at   DateTime | null
sent_at       DateTime | null
artifact_path String
```

---

### InterviewPrep

A preparation artifact for a scheduled interview.

```
id            UUID
application_id UUID → Application
scheduled_at  DateTime
company_research  String
star_stories      String
question_list     String
plan_30_60_90     String
generated_at  DateTime
artifact_path String
```

---

### Event

Append-only event log. Never updated. Never deleted.

```
id            UUID
name          Enum(see event-model.md)
payload       JSON
occurred_at   DateTime
agent         String
```

---

### Metric

Weekly snapshot of pipeline conversion rates.

```
id              UUID
week_start      Date
jobs_discovered Integer
jobs_qualified  Integer
applications    Integer
responses       Integer
interviews      Integer
offers          Integer
rejections      Integer
recorded_at     DateTime
```

---

### Log

Execution audit trail for every agent run.

```
id            UUID
agent         String
started_at    DateTime
duration_ms   Integer
status        Enum(success, failure, skipped)
error         String | null
output_path   String | null
```

---

## Entity Relationships

```
Job
 └── Resume (many, one per tailoring)
 └── Application (one)
      └── OutreachMessage (many: initial, follow-up, thank-you)
      └── InterviewPrep (one per scheduled interview)

Event (independent, references any entity by id in payload)
Metric (independent, weekly aggregate)
Log (independent, per agent execution)
```

---

## Status Lifecycle

### Application.status

```
draft
  → approved (human approves resume)
    → submitted (human confirms send)
      → responded (recruiter replies)
        → interview
          → offer
          → rejected
      → rejected (no response / explicit rejection)
  → withdrawn (candidate withdraws)
```

---

## Integrity Rules

1. A Resume may not be approved if it contains fabricated content.
2. An Application may not be submitted without an approved Resume.
3. An OutreachMessage may not be sent without human approval.
4. Events are immutable once written.
5. Logs are immutable once written.
