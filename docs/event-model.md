# Event Model

CareerOS — v0.1

---

## Design Rules

- Events are append-only. Never updated. Never deleted.
- Every event has a unique ID, name, timestamp, source agent, and JSON payload.
- Events are the only inter-agent signal.
- Agents subscribe to event types by polling the `events` table.

---

## Event Schema

```sql
CREATE TABLE events (
  id           TEXT PRIMARY KEY,   -- UUID
  name         TEXT NOT NULL,      -- event name (see below)
  payload      TEXT NOT NULL,      -- JSON
  occurred_at  TEXT NOT NULL,      -- RFC 3339
  agent        TEXT NOT NULL       -- agent that emitted it
);
```

---

## Event Catalog

### JobFound

Emitted when a new job posting is discovered and stored.

```json
{
  "job_id": "uuid",
  "title": "Senior Program Manager",
  "company": "Acme Corp",
  "url": "https://...",
  "source": "linkedin"
}
```

Consumed by: Qualification agent

---

### JobQualified

Emitted when a job passes qualification criteria.

```json
{
  "job_id": "uuid",
  "title": "Senior Program Manager",
  "company": "Acme Corp",
  "score": 0.87
}
```

Consumed by: Resume agent

---

### JobDisqualified

Emitted when a job fails qualification. Informational only.

```json
{
  "job_id": "uuid",
  "reason": "Requires relocation to NYC"
}
```

Consumed by: nothing (logged only)

---

### ResumeGenerated

Emitted when a tailored resume is written and awaiting human approval.

```json
{
  "resume_id": "uuid",
  "job_id": "uuid",
  "version": 1,
  "artifact_path": "outputs/resumes/abc-v1.txt"
}
```

Consumed by: Outreach agent (after human approval)

---

### ResumeApproved

Emitted when a human approves a resume via CLI.

```json
{
  "resume_id": "uuid",
  "job_id": "uuid",
  "approved_by": "human",
  "approved_at": "2026-06-17T10:00:00Z"
}
```

Consumed by: Outreach agent

---

### ApplicationPrepared

Emitted when outreach drafts are ready for human review.

```json
{
  "application_id": "uuid",
  "job_id": "uuid",
  "resume_id": "uuid",
  "outreach_ids": ["uuid", "uuid"]
}
```

Consumed by: nothing (human reviews manually)

---

### OutreachApproved

Emitted when a human approves an outreach message.

```json
{
  "outreach_id": "uuid",
  "application_id": "uuid",
  "type": "initial"
}
```

Consumed by: nothing (human sends manually)

---

### InterviewScheduled

Emitted when a human updates an application status to `interview`.

```json
{
  "application_id": "uuid",
  "job_id": "uuid",
  "company": "Acme Corp",
  "scheduled_at": "2026-06-25T14:00:00Z"
}
```

Consumed by: Interview Prep agent

---

### OfferReceived

Emitted when a human updates an application status to `offer`.

```json
{
  "application_id": "uuid",
  "job_id": "uuid",
  "company": "Acme Corp"
}
```

Consumed by: Retrospective agent

---

### WeeklyReviewGenerated

Emitted after the weekly retrospective agent completes.

```json
{
  "week_start": "2026-06-15",
  "artifact_path": "outputs/reviews/2026-25.md",
  "jobs_discovered": 34,
  "jobs_qualified": 12,
  "applications": 5,
  "responses": 2,
  "interviews": 1,
  "offers": 0
}
```

Consumed by: nothing (human reads the report)

---

## Event Flow Diagram

```
Discovery agent
  → JobFound
      → Qualification agent
          → JobQualified
              → Resume agent
                  → ResumeGenerated
                      [human approval]
                  → ResumeApproved
                      → Outreach agent
                          → ApplicationPrepared
                              [human reviews + sends]
                              [human updates status]
                                  → InterviewScheduled
                                      → Interview Prep agent
                                  → OfferReceived
                                      → Retrospective agent
                                          → WeeklyReviewGenerated
          → JobDisqualified (logged)
```

---

## Retention

Events are never deleted.

They form the complete audit trail of the system's behavior.
