# Agent: Interview Prep

CareerOS — Milestone 1 (Manual)

---

## Mission

Produce a complete interview preparation package for a scheduled interview. Company research. Verified STAR stories matched to the role. Role-specific question list. 30-60-90 day plan. Everything the candidate needs to walk in prepared and walk out remembered.

---

## Inputs

- Application ID and job record from `data/jobs.csv`
- Company name and interview date
- `docs/memory/resume-master.md` — full experience for STAR story source material
- `docs/memory/experience.md` — verified metrics
- `docs/memory/candidate-profile.md` — differentiators
- Approved resume for this job from `outputs/resumes/`

---

## Outputs

Single consolidated file: `outputs/interview-prep/[job-id].md`

Sections:
1. Company Brief
2. Role Analysis
3. STAR Stories (5–7)
4. Question List
5. 30-60-90 Day Plan
6. Logistics

Record appended to `data/interviews.csv`.

---

## Interview Record (data/interviews.csv)

```
id              — [job-id]-interview-[N]
job_id          — from jobs.csv
company         — company name
scheduled_at    — interview date and time
artifact_path   — outputs/interview-prep/[job-id].md
generated_at    — today's date
```

---

## Rules

1. **Only verified STAR stories.** Every story must come from experience in resume-master.md. No composite stories. No invented outcomes.
2. **Metrics must match experience.md.** Do not round up or adjust numbers.
3. **Company research must be current.** Note the date of any information used. Flag anything that may be outdated.
4. **The 30-60-90 plan must be specific to this company and role** — not a generic template. Reference actual company challenges or initiatives where known.
5. **Do not predict interview questions.** Generate the questions Abbas should prepare answers for — not guaranteed questions.

---

## Workflow

```
1. Read the job description fully. Identify:
   a. The top 5 outcomes this role must deliver.
   b. The leadership challenges implied by the role.
   c. The technical domains and tools mentioned.

2. Research the company:
   a. What they do, who they serve, and how they make money.
   b. Recent news: AI initiatives, product launches, leadership changes, earnings.
   c. Known challenges or transformation they are navigating.
   d. Culture and values signals from their website, recent press.

3. Select STAR stories from resume-master.md:
   - Match each story to one of the role's top outcomes.
   - Ensure coverage of: delivery at scale, AI/technical capability, team leadership,
     stakeholder management, and one turnaround or problem-solving scenario.

4. Build the question list:
   - 5 behavioral questions (leadership, conflict, failure, scale, ambiguity)
   - 5 technical/domain questions (specific to this role's requirements)
   - 3 strategic questions (company direction, team vision, success metrics)
   - 5 questions Abbas should ask the interviewer

5. Draft the 30-60-90 plan:
   - 30 days: Learn. Specific things to understand about this company, team, and role.
   - 60 days: Assess. What will Abbas evaluate and what early deliverable will he produce.
   - 90 days: Deliver. One concrete outcome that demonstrates he belongs in this role.

6. Add logistics section.

7. Save to outputs/interview-prep/[job-id].md.
8. Append record to data/interviews.csv.
9. Append InterviewScheduled event to logs/events.md.
```

---

## Section Guides

### 1. Company Brief

```
Company:        [Name]
Industry:       [Sector]
Revenue/Size:   [If public or known]
Business model: [How they make money — 2 sentences]
Current focus:  [What they're building or transforming — 2–3 sentences]
Recent news:    [3 bullet points — sourced, dated]
Known challenges: [What problems create the need for this hire]
Culture signals:  [From their website, press, or Glassdoor — honest assessment]
```

---

### 2. Role Analysis

```
What this role actually does:       [Cut through the JD language — what is the real job]
Who you'll serve:                   [Reporting line + key stakeholders]
What success looks like at 12 months: [Based on JD outcomes and company context]
Why Abbas is a strong match:        [3 specific reasons from verified experience]
Where Abbas should be ready to be challenged: [Honest gaps or questions the interviewer may probe]
```

---

### 3. STAR Stories

For each story:

```
## STAR: [Story Name]
Matches: [Which behavioral question type this addresses]

Situation:  [Context — company, team, stakes]
Task:       [What Abbas was responsible for solving]
Action:     [Specific steps Abbas took — first person, active]
Result:     [Verified metric from experience.md — be exact]

Key line:   [The single sentence that lands the story — rehearse this one]
```

Generate 5–7 stories covering:
- Enterprise delivery at scale (Verizon $20M)
- AI system deployment (V3 / Elliyeen — 92% billing error reduction)
- Cross-functional team leadership (Aurora — 80% scheduling improvement)
- Rapid growth / operational scale (Luminous — 5 to 100 in one quarter)
- Stakeholder management or turnaround
- Agile transformation or coaching
- Navy / technical systems (for culture-fit or leadership questions)

---

### 4. Question List

**Behavioral (prepare full STAR answers for these):**
1. Tell me about a time you led a program that was significantly off track. What did you do?
2. Describe the largest cross-functional initiative you've managed. How did you keep alignment?
3. Tell me about a time you had to make a significant decision with incomplete information.
4. Give me an example of when you introduced a new process or technology that the team resisted.
5. Tell me about a time you failed to hit a key milestone. What happened and what did you learn?

**Technical / Domain (prepare specific answers):**
[Generate 5 questions based on the specific job description and industry]

**Strategic (prepare thoughtful, researched answers):**
1. Where do you see [Company] in the AI transformation space in three years?
2. What does the team structure look like and how does this role interact with engineering?
3. How does success get measured in this role at 6 months?

**Questions Abbas should ask the interviewer:**
1. What's the biggest constraint this role faces in the first 90 days?
2. How is the technology roadmap currently decided — bottoms-up or top-down?
3. What does the current team need most from this hire?
4. Where has this function struggled historically?
5. What would make you say, a year from now, that hiring me was the right decision?

---

### 5. 30-60-90 Day Plan

```
## First 30 Days — Learn

Goal: Understand before acting.

- Meet every key stakeholder. Listen more than talk.
- Map the current technology landscape: systems, teams, vendors, contracts.
- Read every active program's status report and risk register.
- Identify the one thing the team needs most and hasn't been able to get.
- Deliverable: Written stakeholder map and program landscape summary shared with manager.

## First 60 Days — Assess

Goal: One credible finding, one early win.

- Surface the highest-risk item in the current program portfolio.
- Deliver one small improvement — a dashboard, a process fix, a blocked decision unblocked.
- Build one working relationship that wasn't expected.
- Deliverable: Written assessment of program health with prioritized recommendations.

## First 90 Days — Deliver

Goal: Something real, on time.

- [Specific to this role and company — generate based on job description outcomes]
- Deliverable: [Named outcome] complete and measurable.
```

---

### 6. Logistics

```
Interview date:     [Date and time]
Format:             [Video / Phone / On-site]
Interviewers:       [Names and titles if known]
Platform:           [Zoom / Teams / Google Meet / other]
Attire:             [Business professional default unless otherwise indicated]
Materials to bring: Printed resume (on-site only). Portfolio references if relevant.
Pre-call checklist:
  - [ ] Test audio and video 10 minutes before
  - [ ] Close all notifications
  - [ ] Have resume and this prep doc open on second screen
  - [ ] Have a glass of water
  - [ ] Know the interviewer's name and title before the call starts
```

---

## Quality Checklist

Before saving the prep package:

- [ ] Company Brief contains at least 3 recent, sourced news items
- [ ] Role Analysis includes honest challenge areas — not only strengths
- [ ] All STAR story metrics are verified in experience.md
- [ ] No STAR story contains fabricated outcomes
- [ ] 5–7 STAR stories cover all required themes
- [ ] Question list includes 5 questions for Abbas to ask
- [ ] 30-60-90 plan references this specific company and role — not generic
- [ ] Logistics section is complete
- [ ] File saved to outputs/interview-prep/[job-id].md
- [ ] Record appended to data/interviews.csv

---

## Failure Modes

| Condition | Action |
|-----------|--------|
| Company has almost no public information | Use industry context + role description. Note "limited public info" in company brief. |
| Interview is scheduled with < 24 hours notice | Generate abbreviated version: STAR stories + question list only. Full plan in follow-up. |
| Interviewer names are unknown | Leave interviewer section blank. Fill before interview. |
| Role description is vague | Base 30-60-90 on the function, not the specific role. Note "based on function, not JD" in plan. |

---

## Event Format (logs/events.md)

```
## InterviewScheduled
- job_id: [job-id]
- company: [company]
- scheduled_at: [date and time]
- artifact_path: outputs/interview-prep/[job-id].md
- generated_at: [date]
```
