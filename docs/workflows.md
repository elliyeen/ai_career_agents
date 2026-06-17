# Workflows

CareerOS — v0.1

---

## Pipeline Overview

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

Each stage is a discrete agent. Agents do not call each other. Each stage reads the database, does its work, writes output, and emits an event.

---

## Stage 1 — Discovery

**Trigger:** Scheduled daily, or `career-os run discovery`

**Mission:** Find new job postings from approved sources. Store them. Deduplicate.

**Inputs:**
- Approved source list (config)
- Existing job URLs in database (for dedup)

**Process:**
1. Fetch job listings from each approved source.
2. Parse: title, company, URL, location, remote flag, description.
3. Check URL against existing jobs table.
4. Skip duplicates.
5. Insert new jobs with `qualified = null`.
6. Emit `JobFound` for each new job.

**Outputs:**
- New rows in `jobs` table.
- `JobFound` events.

**Quality rules:**
- URL must be valid and unique.
- Title and company must be non-empty.
- No fabricated postings.

**Failure modes:**
- Source unreachable: retry 3×, then log and skip source.
- Parse failure: log, flag for human review, skip record.

---

## Stage 2 — Qualification

**Trigger:** `JobFound` event, or `career-os run qualification`

**Mission:** Evaluate each unqualified job against the candidate profile. Mark as qualified or disqualified with reason.

**Inputs:**
- Jobs where `qualified = null`
- Candidate profile (config: skills, experience areas, preferences)

**Process:**
1. Load unqualified jobs.
2. For each job, score against: title match, skill overlap, location/remote preference, seniority level.
3. Mark `qualified = true` or `qualified = false` with `disqualified_reason`.
4. Emit `JobQualified` for qualified jobs.

**Outputs:**
- Updated `qualified` field on job records.
- `JobQualified` events.

**Quality rules:**
- Qualification criteria must be defined in config.
- Disqualification reason must be recorded — no silent drops.

**Failure modes:**
- No criteria configured: stop execution, warn user.

---

## Stage 3 — Resume

**Trigger:** `JobQualified` event, or `career-os run resume --job-id <id>`

**Mission:** Generate a tailored, ATS-friendly resume for a qualified job.

**Inputs:**
- Job record (title, description, requirements)
- Master resume (docs/memory/resume-master.md)
- Verified experience (docs/memory/experience.md)

**Process:**
1. Load job description.
2. Load master resume.
3. Identify relevant experience sections.
4. Tailor wording to match job language (without fabricating).
5. Write ATS-friendly plain text output.
6. Save to `outputs/resumes/`.
7. Insert Resume record with `approved = false`.
8. Emit `ResumeGenerated`.

**Outputs:**
- File: `outputs/resumes/{job_id}-v{n}.txt`
- Resume record in database.
- `ResumeGenerated` event.

**Quality rules:**
- Only verified experience. No invented metrics. No invented employers.
- Human must review and approve before Resume can be used.

**Human approval step:**
```
career-os review resume <resume-id>   # opens file
career-os approve resume <resume-id>  # marks approved
```

**Failure modes:**
- Master resume missing: stop execution.
- Validation failure (fabrication detected): stop, do not write output.

---

## Stage 4 — Outreach

**Trigger:** `ResumeGenerated` event (after human approval), or `career-os run outreach --application-id <id>`

**Mission:** Draft recruiter message, follow-up, and thank-you note.

**Inputs:**
- Application record
- Job record
- Resume (approved)
- Candidate profile

**Process:**
1. Draft initial recruiter message.
2. Draft follow-up (scheduled +5 business days).
3. Save drafts to `outputs/outreach/`.
4. Insert OutreachMessage records with `approved = false`.
5. Emit `ApplicationPrepared`.

**Outputs:**
- Files in `outputs/outreach/`
- OutreachMessage records.
- `ApplicationPrepared` event.

**Human approval step:**
```
career-os review outreach <message-id>
career-os approve outreach <message-id>
```

**Failure modes:**
- No approved resume: stop. Do not draft outreach.

---

## Stage 5 — Tracking

**Trigger:** Ongoing. Updated manually or via `career-os update application <id> --status <status>`

**Mission:** Maintain accurate application status. No automation. Human-driven.

**Inputs:** Human input via CLI.

**Process:**
1. Human receives response.
2. Human updates application status.
3. System records timestamp.
4. If `interview` status: trigger Interview Prep agent.

**Outputs:**
- Updated Application record.
- `InterviewScheduled` event (if applicable).

---

## Stage 6 — Interview

**Trigger:** `InterviewScheduled` event, or `career-os run interview-prep --application-id <id>`

**Mission:** Produce a complete interview preparation artifact.

**Inputs:**
- Job record
- Company name
- Application record

**Process:**
1. Research company (from approved sources).
2. Generate STAR stories from verified experience.
3. Generate question list (role-specific + behavioral).
4. Generate 30-60-90 day plan.
5. Write to `outputs/interview-prep/`.
6. Insert InterviewPrep record.

**Outputs:**
- File: `outputs/interview-prep/{application-id}.md`
- InterviewPrep record.

---

## Stage 7 — Offer

**Trigger:** Human updates application status to `offer`.

**Mission:** Record offer details. Support decision-making.

**Inputs:** Human input (compensation, title, start date, notes).

**Process:**
1. Record offer in application notes.
2. Emit `OfferReceived`.
3. Flag for retrospective.

**Outputs:**
- Updated Application record.
- `OfferReceived` event.

---

## Stage 8 — Retrospective

**Trigger:** Weekly (Sunday), or `career-os run review`

**Mission:** Measure pipeline performance. Identify weakest stage. Produce recommendations.

**Inputs:**
- Weekly metrics snapshot
- Application status history
- Event log

**Process:**
1. Calculate conversion rates for each stage.
2. Identify the stage with lowest conversion.
3. Generate written recommendations.
4. Write to `outputs/reviews/`.
5. Emit `WeeklyReviewGenerated`.

**Outputs:**
- File: `outputs/reviews/YYYY-WW.md`
- Metric record.
- `WeeklyReviewGenerated` event.
