# Qualification Criteria

CareerOS — v0.1
Used by the Qualification agent to score each discovered job as qualified or disqualified.

---

## Scoring Model

Each job is scored across five dimensions. Total possible score: 100.

A job qualifies if:
- Total score ≥ 60, AND
- No hard disqualifier is triggered.

A job is disqualified if:
- Total score < 60, OR
- Any hard disqualifier is triggered (regardless of score).

---

## Dimension 1 — Title Match (25 points)

Does the job title align with target roles in the candidate profile?

| Match level | Points |
|-------------|--------|
| Exact match (e.g. "Director of Technology", "Senior TPM", "VP of Engineering") | 25 |
| Strong adjacent (e.g. "Head of AI Delivery", "Principal Program Manager", "Chief of Staff") | 20 |
| Moderate adjacent (e.g. "Senior Program Manager", "Platform Lead") | 12 |
| Weak adjacent (e.g. "Project Manager", "Operations Manager" without seniority signal) | 5 |
| No match | 0 |

---

## Dimension 2 — Industry Fit (20 points)

Does the company operate in a preferred or open industry?

| Match level | Points |
|-------------|--------|
| Preferred industry (telecom, AI/ML, enterprise software, aerospace, public safety, gov/civic tech) | 20 |
| Open industry (healthtech, fintech, logistics tech) | 12 |
| Neutral (no strong fit or mis-fit signal) | 5 |
| Avoid industry (pure consumer, non-tech ops) | 0 |

---

## Dimension 3 — Skill Overlap (25 points)

How many of the job's required skills appear in experience.md?

Count required skills that map directly to verified experience. Divide by total required skills. Apply to 25 points.

| Overlap | Points |
|---------|--------|
| ≥ 80% of required skills verified | 25 |
| 60–79% | 18 |
| 40–59% | 10 |
| 20–39% | 4 |
| < 20% | 0 |

Skills to weight most heavily (strong differentiators):
- AI / GenAI / LLM systems
- Agentic workflows
- Cloud migration programs
- Enterprise program management at scale
- Cross-functional team leadership
- Telecommunications / aerospace domain

---

## Dimension 4 — Seniority Signal (20 points)

Is the role at the right level?

| Signal | Points |
|--------|--------|
| Director, VP, Head of, Principal, Lead — clearly senior IC or manager | 20 |
| Senior with meaningful scope indicators (budget, team, program scale) | 14 |
| Senior without scope clarity | 7 |
| Mid-level signals (Manager, no Senior modifier) | 2 |
| Junior signals (Coordinator, Analyst, Associate) | 0 |

---

## Dimension 5 — Location / Remote (10 points)

| Signal | Points |
|--------|--------|
| Fully remote | 10 |
| Dallas, TX (on-site or hybrid) | 10 |
| Texas (non-Dallas, hybrid) | 7 |
| Remote-friendly (US) | 9 |
| Relocation required | 0 — flag for human review |

---

## Hard Disqualifiers

Any of the following disqualifies a job regardless of score:

| Condition | Action |
|-----------|--------|
| Hands-on coding as primary responsibility | Disqualify |
| Requires active PMP certification as stated prerequisite | Flag — do not auto-disqualify; note in record |
| Relocation required, no remote option | Flag — escalate to human |
| Role is below Senior / Staff level by title and description | Disqualify |
| Pure sales quota-carrying role | Disqualify |
| Company has no disclosed funding and < 50 employees (startup risk) | Flag — escalate to human |

---

## Scoring Example

**Job:** Senior Technical Program Manager — AI Platform, Verizon Business, Remote

| Dimension | Score |
|-----------|-------|
| Title Match: "Senior Technical Program Manager" → strong adjacent | 20 |
| Industry: Telecom → preferred | 20 |
| Skill overlap: AI platform, cloud, Agile, stakeholder mgmt → ~85% | 25 |
| Seniority: Senior with program scope | 14 |
| Location: Remote | 10 |
| **Total** | **89 / 100** |
| Hard disqualifiers | None |
| **Verdict** | **Qualified** |

---

## Output Format

The Qualification agent writes the following to the jobs table on evaluation:

```
qualified:            true | false
qualified_at:         timestamp
disqualified_reason:  string (required if false)
score:                integer (0-100)
score_breakdown:      JSON {title, industry, skills, seniority, location}
```

---

## Calibration Note

These criteria are a starting point. The Weekly Review agent will surface patterns — if qualified jobs are not converting to applications, the threshold or dimension weights may need adjustment. Update this file when criteria are recalibrated.
