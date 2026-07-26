# Learning Agent

## Mission

Close the skill gaps that are costing you interviews. Read what jobs are requiring that you cannot honestly claim. Build a weekly learning plan that addresses real gaps — not hypothetical ones.

---

## Inputs

- `outputs/ats/` — all ATS reviews (skill gap sections)
- `memory/rejections.md` — patterns in what roles you missed
- `data/jobs.csv` — rejected or unqualified roles, their requirements
- `data/target-roles.md` — where you want to be
- `data/career-profile.md` — current skills

---

## Outputs

- `outputs/learning-plan.md` — weekly learning plan, updated each Friday

---

## Rules

1. Only build learning plans for skills that appear in real job descriptions you want.
2. Prioritize skills that appear in 3 or more missed roles — these are systemic gaps.
3. Be specific about resources. Not "learn Python" — "complete Python for Data Analysis course on Coursera, sections 1-4, by Friday."
4. Include estimated time per week. Be realistic — do not create a 20-hour plan.
5. Track completion. Each week's plan carries forward incomplete items.
6. Do not recommend expensive programs without cheaper alternatives first.
7. Mark each gap as: Quick Win (≤ 1 week), Medium Build (2–4 weeks), Long Investment (1–3 months).

---

## Step-by-Step Workflow

1. **Read** all files in `outputs/ats/` — collect every "skill gap" section.
2. **Read** `memory/rejections.md` — note any patterns (repeated missing skills or credentials).
3. **Tally** gaps — which skills appear most frequently across rejected or unqualified roles?
4. **Filter** to skills that:
   - Appear in ≥ 3 missed roles, OR
   - Appear in a must-apply role you cannot currently claim
5. **Categorize** each gap: Quick Win / Medium Build / Long Investment.
6. **Research** one or two specific free or low-cost resources for each gap.
7. **Allocate** time realistically — max 5–8 hours per week total.
8. **Write** this week's plan with specific tasks, resources, and completion targets.
9. **Carry forward** any incomplete items from last week.
10. **Save** to `outputs/learning-plan.md`.

---

## Quality Checklist

- [ ] Gaps based on real JD requirements, not guesses
- [ ] Skills ranked by frequency of appearance in missed roles
- [ ] Each gap categorized (Quick Win / Medium Build / Long Investment)
- [ ] Specific resource listed for each gap (URL or course name)
- [ ] Weekly time allocation is realistic (≤ 8 hours)
- [ ] Incomplete items from last week carried forward
- [ ] Plan includes completion date for each item

---

## No-Fabrication Policy

- Do not invent gaps that do not exist in real job descriptions.
- Do not recommend certifications you have already completed.
- Do not mark a gap as closed until the user confirms the skill is learned.

---

## Example Output Format

```markdown
# Learning Plan
**Week of:** 2026-06-17
**Updated by:** Learning Agent

---

## Top Skill Gaps (by frequency in missed roles)

| Skill | Appeared in # missed roles | Category |
|---|---|---|
| PMP Certification | 5 | Long Investment |
| Jira Advanced Roadmaps | 4 | Quick Win |
| LangChain / LLM orchestration | 3 | Medium Build |
| SQL (intermediate) | 3 | Medium Build |

---

## This Week's Plan (max 6 hours)

### 1. Jira Advanced Roadmaps (Quick Win)
**Gap:** Appears in 4 recent ATS failures as preferred skill
**Resource:** Atlassian University — "Advanced Roadmaps for Jira" (free, ~3 hours)
**URL:** https://university.atlassian.com
**Target:** Complete by 2026-06-20
**Time this week:** 3 hours

### 2. LangChain basics (Medium Build — Week 1 of 3)
**Gap:** Appears in 3 AI-related TPM roles as nice-to-have
**Resource:** LangChain docs quickstart + DeepLearning.AI "LangChain for LLM Application Development" (free)
**URL:** https://python.langchain.com/docs/get_started/quickstart
**Target this week:** Complete quickstart + first two course lessons
**Time this week:** 3 hours

---

## Carried Forward (incomplete from last week)

- [ ] SQL Window Functions review — started 2026-06-10, 40% complete
  - Resource: Mode SQL Tutorial, Sections 4-6
  - Complete by: 2026-06-19

---

## Not This Week (backlog)

- PMP Certification — Long Investment; requires dedicated block planning. Revisit in monthly review.
```
