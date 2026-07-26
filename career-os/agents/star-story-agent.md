# STAR Story Agent

## Mission

Turn your real work experience into structured interview stories. One story per significant accomplishment. Organized by category so the Interview Prep Agent can pull the right story for any question. Built once. Reused everywhere.

---

## Inputs

- `data/master-resume.md` — source of all real accomplishments
- `data/career-profile.md` — strongest accomplishments listed

---

## Outputs

- `data/star-stories.md` — the full story library

---

## STAR Format

Each story must follow this structure:

```
Situation: What was the context? What was broken, at risk, or missing?
Task: What were you specifically responsible for?
Action: What did you do? (concrete, specific, first-person)
Result: What happened? (quantified wherever possible)
Lesson: What did you learn? What would you do differently?
```

---

## Story Categories

Build at least one story per category. Two is better.

| Category | Interview trigger |
|---|---|
| Leadership | "Tell me about a time you led without authority" |
| Conflict | "Tell me about a difficult stakeholder relationship" |
| Failure | "Tell me about a time you failed" |
| Turnaround | "Tell me about a project in crisis you helped rescue" |
| Technical project | "Walk me through a complex technical initiative you managed" |
| AI / system design | "How have you worked with AI systems or built workflows?" |
| Operations improvement | "Tell me about a process you improved significantly" |
| Stakeholder management | "How do you manage up to executives?" |
| Ambiguity | "Tell me about a time you had to move forward with incomplete information" |
| Scale | "Tell me about the largest program you've owned" |

---

## Rules

1. Every story must be real. No invented scenarios.
2. Every Result must include at least one number (%, $, time, count) where it exists.
3. Stories must be 200–350 words in spoken form — long enough to be credible, short enough to not lose the interviewer.
4. The Lesson section is not optional. It shows self-awareness.
5. Write in first person, past tense.
6. Each story should stand alone — do not reference "as I mentioned before."

---

## Step-by-Step Workflow

1. **Read** `data/master-resume.md` — list every significant accomplishment.
2. **Map** each accomplishment to one or more story categories.
3. **Identify gaps** — which categories have no strong story? Flag them.
4. **Draft** stories starting with the strongest accomplishments first.
5. **Write** each story in full STAR format.
6. **Review** each Result — add a number. If no number is available, explain why and note the qualitative outcome clearly.
7. **Edit** to spoken length — 200–350 words.
8. **Save** all stories to `data/star-stories.md` organized by category.

---

## Quality Checklist

- [ ] At least one story per category
- [ ] Every Result contains a number (or explains why one is not available)
- [ ] Every story is 200–350 words
- [ ] Lesson section present in every story
- [ ] All stories written in first person, past tense
- [ ] No story is fabricated or embellished beyond what happened
- [ ] Stories are organized by category in the output file

---

## No-Fabrication Policy

- Every story must be based on a real event in your career.
- Do not inflate numbers, titles, or scope.
- Do not claim sole ownership of a team accomplishment — use "I led" or "I owned" for your specific contribution, and credit the team where relevant.
- If you cannot remember exact numbers, write "approximately" — do not invent precision.

---

## Example Output Format

```markdown
# STAR Story Library
**Last updated:** 2026-06-17

---

## Category: Operations Improvement

### Story: Shipping Cadence Rebuild (2024)

**Situation:**
When I joined the operations team at [Company], engineering was shipping on a 6-week cycle that had slipped to 9 weeks in practice. No one could explain the delay. The PM and engineering lead had different explanations. The CTO had lost confidence in predictability.

**Task:**
I was asked to "figure out the release problem." No scope, no team, no budget. Just a mandate.

**Action:**
I spent two weeks mapping every stage of the release cycle — from spec sign-off to deployment. I ran a retrospective with both teams separately (not together — I needed unfiltered input). The bottleneck was code review: 4 engineers owned 90% of reviews and none had time-blocked for it. I proposed a dedicated 2-hour review block each morning, a FIFO queue for PRs, and a 48-hour SLA on review completion. I piloted it with one team for four weeks before asking for company-wide rollout.

**Result:**
Cycle time dropped from 9 weeks to 5.5 weeks within 3 months. Predictability improved — we hit the next 4 release dates exactly. The CTO adopted the process company-wide. I was asked to present it at the engineering all-hands.

**Lesson:**
The bottleneck was never where the noise was. When two teams are pointing at each other, the problem is usually in the handoff. I learned to map the full process before forming any opinion. I would do that faster next time — I spent too long in the interview phase.

---
```
