# Job Scoring Rubric

Every qualified job receives a score from 0–100.
Only jobs scoring 75+ proceed to resume generation.

---

## Score Breakdown

| Category           | Weight | Scoring guide |
|-------------------|--------|---------------|
| Role match         | 0–25   | See below |
| Experience match   | 0–15   | See below |
| Salary match       | 0–15   | See below |
| Industry match     | 0–15   | See below |
| Location match     | 0–10   | See below |
| Growth potential   | 0–10   | See below |
| Strategic value    | 0–10   | See below |
| **Total**          | **100**|  |

---

## Role Match (0–25)

Does the job title and description match your target roles?

| Score | Condition |
|-------|-----------|
| 23–25 | Exact title match to Tier 1 target role |
| 18–22 | Close title match or strong description alignment |
| 12–17 | Acceptable alternative role with right scope |
| 6–11  | Stretch role — significant gap in title or scope |
| 0–5   | Weak title match — barely relevant |

---

## Experience Match (0–15)

Does your verified experience satisfy the job requirements?

| Score | Condition |
|-------|-----------|
| 13–15 | You meet 90%+ of stated requirements with verified experience |
| 9–12  | You meet 70–89% of requirements |
| 5–8   | You meet 50–69% of requirements |
| 0–4   | You meet < 50% of requirements |

**Hard rule:** If the job requires a skill listed as a gap in skills-inventory.md,
deduct 5 points per missing required skill (not "nice to have").

---

## Salary Match (0–15)

Does the posted or estimated compensation meet your requirements?

| Score | Condition |
|-------|-----------|
| 13–15 | Posted range is at or above your target |
| 9–12  | Posted range overlaps your target range |
| 5–8   | Posted range is below target but above floor |
| 0–4   | Unknown salary (no posting) |
| Auto-disqualify | Posted max is below your hard floor |

---

## Industry Match (0–15)

Does the company operate in an industry you prefer or accept?

| Score | Condition |
|-------|-----------|
| 13–15 | Preferred industry |
| 9–12  | Acceptable industry |
| 4–8   | Neutral — no preference |
| 0–3   | Industry you would tolerate but not prefer |
| Auto-disqualify | Excluded industry (from target-companies.md) |

---

## Location Match (0–10)

Is the work arrangement compatible with your preferences?

| Score | Condition |
|-------|-----------|
| 9–10  | Fully remote, US |
| 7–8   | Hybrid with 1–2 days in preferred city |
| 5–6   | Hybrid with 3+ days or non-preferred city |
| 3–4   | In-office in a city you would consider |
| 0–2   | In-office in a city you would not relocate to |
| Auto-disqualify | In-office only in excluded location |

---

## Growth Potential (0–10)

What is the likely trajectory from this role?

| Score | Condition |
|-------|-----------|
| 9–10  | Clear path to next level; company is scaling |
| 7–8   | Reasonable growth opportunity |
| 4–6   | Unclear or lateral |
| 0–3   | Known dead-end or declining company |

Signals to evaluate:
- Company growth rate (headcount, revenue if public)
- Stage (Series A growing > stagnant Series D)
- Role scope (building > maintaining)
- Public statements about team growth in JD

---

## Strategic Value (0–10)

How much does this role advance your career story?

| Score | Condition |
|-------|-----------|
| 9–10  | Adds a brand name, skill, or scope you currently lack |
| 7–8   | Solidifies an existing strength |
| 4–6   | Lateral — same level, same domain |
| 0–3   | Backward step or misaligned with career direction |

---

## Bonus Points (not part of base score — added after)

| Condition                              | Bonus |
|---------------------------------------|-------|
| Company is on Tier 1 target list       | +10   |
| Company is on Tier 2 target list       | +5    |
| Recruiter reached out proactively      | +5    |
| Referral from known contact            | +10   |

Bonus points can push a score above 100. That is acceptable.

---

## Thresholds

| Score    | Action |
|----------|--------|
| 90+      | Prioritize — run resume immediately |
| 75–89    | Qualify — add to resume queue |
| 60–74    | Watchlist — revisit if queue is low |
| Below 60 | Disqualify — record reason, do not proceed |

---

## Disqualification Reasons (use these exact strings in the database)

- `TITLE_MISMATCH` — Title does not match any target role
- `EXCLUDED_INDUSTRY` — Industry is on the blocklist
- `EXCLUDED_COMPANY` — Company is on the blocklist
- `SALARY_BELOW_FLOOR` — Compensation cannot meet minimum requirement
- `LOCATION_INCOMPATIBLE` — In-office in excluded location
- `EXPERIENCE_GAP_CRITICAL` — Missing 2+ required skills with no path to close gap
- `SCORE_BELOW_THRESHOLD` — Total score < 60
- `DUPLICATE` — URL already exists in database
- `STALE_POSTING` — Job posted > 30 days ago (configurable)
