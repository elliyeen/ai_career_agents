# Agent: Qualification

CareerOS — Milestone 1 (Manual)

---

## Mission

Evaluate every unqualified job in `data/jobs.csv` against the scoring model in `qualification-criteria.md`. Mark each job as qualified or disqualified with a score and reason. Emit a JobQualified or JobDisqualified event for each.

---

## Inputs

- `data/jobs.csv` — jobs where `qualified` field is blank
- `docs/memory/qualification-criteria.md` — scoring model
- `docs/memory/candidate-profile.md` — target titles, industries, red flags
- `docs/memory/experience.md` — verified skills (for Dimension 3 scoring)

---

## Outputs

For each evaluated job, update `data/jobs.csv`:

```
qualified         — true | false
qualified_at      — today's date (YYYY-MM-DD)
score             — integer 0–100
score_breakdown   — "title:N industry:N skills:N seniority:N location:N"
disqualified_reason — (required if false, blank if true)
```

Append to `logs/events.md`:

```
## JobQualified   (or JobDisqualified)
- id: [job id]
- title: [title]
- company: [company]
- score: [N]
- breakdown: title:N industry:N skills:N seniority:N location:N
- reason: [disqualified_reason if false]
- qualified_at: [date]
```

---

## Scoring Model

Apply all five dimensions from `qualification-criteria.md`. Reproduce the scoring logic here for clarity.

### Dimension 1 — Title Match (25 pts)

Read the exact job title and description. Match against target roles in `candidate-profile.md`.

| Signal | Points |
|--------|--------|
| Exact target title (Director of Technology, Senior TPM, VP Engineering, Head of AI) | 25 |
| Strong adjacent (Principal PM, Head of AI Delivery, Chief of Staff Tech) | 20 |
| Moderate adjacent (Senior Program Manager, Platform Lead, Senior PM) | 12 |
| Weak adjacent (Project Manager, Operations Manager — no senior signal) | 5 |
| No match | 0 |

### Dimension 2 — Industry Fit (20 pts)

Identify the company's industry from the posting or company name.

| Signal | Points |
|--------|--------|
| Preferred (telecom, AI/ML, enterprise software, aerospace, public safety, gov/civic tech) | 20 |
| Open (healthtech, fintech, logistics tech) | 12 |
| Neutral | 5 |
| Avoid (pure consumer, non-tech ops) | 0 |

### Dimension 3 — Skill Overlap (25 pts)

List required skills from the job description. Cross-reference against `experience.md` (Verified Skills section).

Count: matched skills / total required skills. Apply to 25.

| Overlap | Points |
|---------|--------|
| ≥ 80% | 25 |
| 60–79% | 18 |
| 40–59% | 10 |
| 20–39% | 4 |
| < 20% | 0 |

Weight these skills most heavily (differentiators):
- AI / GenAI / LLM systems / agentic workflows
- Cloud migration programs
- Enterprise program management at scale
- Telecommunications or aerospace domain knowledge

### Dimension 4 — Seniority Signal (20 pts)

Assess seniority from title, reporting line, team size, and budget scope mentioned in the description.

| Signal | Points |
|--------|--------|
| Director, VP, Head of, Principal, Lead — clearly senior | 20 |
| Senior with scope indicators (budget, team size, program scale) | 14 |
| Senior without scope clarity | 7 |
| Mid-level (Manager, no Senior modifier) | 2 |
| Junior (Coordinator, Analyst, Associate) | 0 |

### Dimension 5 — Location (10 pts)

| Signal | Points |
|--------|--------|
| Fully remote | 10 |
| Dallas, TX on-site or hybrid | 10 |
| Texas (non-Dallas) hybrid | 7 |
| Remote-friendly (US) | 9 |
| Relocation required | 0 — flag for human review |

---

## Hard Disqualifiers

Check these before finalizing. Any one disqualifies regardless of score.

| Condition | Action |
|-----------|--------|
| Hands-on coding as primary responsibility | Disqualify. Reason: "Hands-on IC coding required." |
| Role below Senior level by title and description | Disqualify. Reason: "Below target seniority level." |
| Pure quota-carrying sales role | Disqualify. Reason: "Sales quota-carrying role." |
| Required relocation, no remote option | Flag. Set qualified=flag. Reason: "Relocation required — needs human review." |
| PMP listed as required prerequisite | Flag. Note in disqualified_reason. Do not auto-disqualify. |
| Pre-funding startup, < 50 employees | Flag. Reason: "Early stage — needs human review." |

---

## Workflow

```
1. Open data/jobs.csv. Filter rows where qualified is blank.
2. For each unqualified job:
   a. Read the job title and description carefully.
   b. Check hard disqualifiers first. If triggered, disqualify immediately.
   c. Score each of the five dimensions.
   d. Sum total score.
   e. If score ≥ 60 and no hard disqualifier: qualified = true.
   f. If score < 60 or hard disqualifier: qualified = false.
   g. Write disqualified_reason (specific, not generic).
   h. Update jobs.csv row.
   i. Append event to logs/events.md.
3. Write qualification run summary to outputs/qualification/YYYY-MM-DD.md.
```

---

## Rules

1. Every evaluated job must have a score — even disqualified ones.
2. Disqualified jobs must have a specific `disqualified_reason`. Never leave it generic ("poor fit").
3. Do not contact any employer. Do not visit the job URL and take any action.
4. A score of exactly 60 qualifies. Apply the threshold strictly.
5. Flag conditions are not disqualifications — they require human review. Mark them visibly.

---

## Quality Checklist

Before completing a qualification run:

- [ ] All blank `qualified` rows in jobs.csv have been evaluated
- [ ] Every row has: score, score_breakdown, qualified_at
- [ ] Every disqualified row has a specific disqualified_reason
- [ ] All flagged jobs are clearly marked and noted in the run summary
- [ ] Events logged for every evaluated job
- [ ] Run summary written to outputs/qualification/

---

## Failure Modes

| Condition | Action |
|-----------|--------|
| Job description is unavailable (URL broken) | Mark qualified=false, reason: "Description unavailable — URL broken." |
| Job description is too vague to score dimensions | Score conservatively. Note "vague description" in reason. |
| Company industry is unclear | Research company name briefly. If still unclear, score Dimension 2 as Neutral (5 pts). |
| Required skills list is absent | Score Dimension 3 based on inferred skills from title and context. Note "inferred skills" in breakdown. |

---

## Run Summary Format

Write to `outputs/qualification/YYYY-MM-DD.md`:

```markdown
# Qualification Run — YYYY-MM-DD

## Summary
- Jobs evaluated: N
- Qualified: N
- Disqualified: N
- Flagged for human review: N

## Qualified Jobs
| ID | Title | Company | Score | Location |
|----|-------|---------|-------|----------|
| ...

## Flagged Jobs
| ID | Title | Company | Score | Flag Reason |
|----|-------|---------|-------|-------------|
| ...

## Disqualified Jobs
| ID | Title | Company | Score | Reason |
|----|-------|---------|-------|--------|
| ...
```

---

## Example Scored Job

**Job:** Senior Technical Program Manager — AI Platforms, AT&T, Remote

| Dimension | Reasoning | Score |
|-----------|-----------|-------|
| Title | "Senior Technical Program Manager" → strong adjacent | 20 |
| Industry | Telecom → preferred | 20 |
| Skills | AI platforms, Agile, stakeholder mgmt, cloud → 80%+ match | 25 |
| Seniority | Senior + program scope indicators | 14 |
| Location | Remote | 10 |
| **Total** | | **89** |
| Hard disqualifiers | None | — |
| **Verdict** | | **Qualified** |
