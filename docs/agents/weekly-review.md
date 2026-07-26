# Agent: Weekly Review

CareerOS — Milestone 1 (Manual)

---

## Mission

Every Sunday: measure the pipeline. Calculate conversion rates for each stage. Identify the weakest stage. Produce specific, actionable recommendations. Write the report. This is how the system improves over time.

---

## Inputs

- `data/jobs.csv` — all jobs (discovered, qualified, disqualified)
- `data/applications.csv` — all applications and their current status
- `data/resumes.csv` — resumes generated and approved
- `data/outreach.csv` — outreach drafted, approved, and sent
- `data/interviews.csv` — scheduled interviews
- `logs/events.md` — full event history
- Previous weekly review (from `outputs/reviews/`) for comparison

---

## Outputs

- Report file: `outputs/reviews/YYYY-WW.md` (e.g. `2026-25.md` for week 25)
- Metric record appended to `data/metrics.csv`
- Event appended to `logs/events.md`

---

## Metrics Record (data/metrics.csv)

```
week_start        — Monday of the review week (YYYY-MM-DD)
jobs_discovered   — new jobs added to jobs.csv this week
jobs_qualified    — jobs marked qualified=true this week
jobs_disqualified — jobs marked qualified=false this week
resumes_generated — resumes created this week
resumes_approved  — resumes approved by human this week
applications      — applications submitted this week
responses         — recruiter or employer responses received this week
interviews        — interviews scheduled this week
offers            — offers received this week
rejections        — explicit rejections received this week
recorded_at       — today's date
```

---

## Rules

1. **Count only what happened this week** for weekly totals. Cumulative totals go in a separate section.
2. **Calculate conversion rates honestly.** If the denominator is zero, report zero — not N/A.
3. **Identify the weakest stage specifically** — not "outreach needs work." Name the exact number and what it means.
4. **Recommendations must be actionable.** "Apply to more jobs" is not a recommendation. "Increase qualification threshold to 70 to improve resume signal quality" is.
5. **Compare to prior week.** Every metric shows this week, last week, and direction (↑↓→).
6. **Do not fabricate any metric.** Count only from the data files. If a record is missing, note it.

---

## Workflow

```
1. Set the review window: Monday through Sunday of the past week.

2. Count weekly metrics from data files:
   a. jobs.csv: rows where discovered_at is in the review window → jobs_discovered
   b. jobs.csv: rows where qualified_at is in the review window, qualified=true → jobs_qualified
   c. jobs.csv: rows where qualified_at is in the review window, qualified=false → jobs_disqualified
   d. resumes.csv: rows where generated_at is in review window → resumes_generated
   e. resumes.csv: rows where approved=true and approved_at is in review window → resumes_approved
   f. applications.csv: rows where submitted_at is in review window → applications
   g. applications.csv: rows where response_at is in review window → responses
   h. interviews.csv: rows where scheduled_at is in review window → interviews
   i. applications.csv: rows where status=offer and updated this week → offers
   j. applications.csv: rows where status=rejected and updated this week → rejections

3. Calculate conversion rates:
   - Discovery → Qualification: jobs_qualified / jobs_discovered
   - Qualification → Resume: resumes_generated / jobs_qualified
   - Resume → Application: applications / resumes_approved
   - Application → Response: responses / applications
   - Response → Interview: interviews / responses
   - Interview → Offer: offers / interviews

4. Load prior week metrics from data/metrics.csv for comparison.

5. Identify weakest stage:
   - The stage with the lowest conversion rate is the weakest.
   - If two stages are tied, prioritize the one earlier in the pipeline (fixing earlier stages compounds).

6. Write recommendations (minimum 3, maximum 5):
   - One for the weakest stage.
   - One for the second weakest stage.
   - One systemic observation.

7. Write the full report to outputs/reviews/YYYY-WW.md.

8. Append metric record to data/metrics.csv.

9. Append WeeklyReviewGenerated event to logs/events.md.
```

---

## Conversion Rate Benchmarks

Use these to contextualize performance. These are industry-approximate benchmarks, not targets.

| Stage | Benchmark |
|-------|-----------|
| Discovery → Qualification | 30–50% (expect more disqualification early; improve over time) |
| Qualification → Resume | 80–100% (every qualified job should get a resume) |
| Resume → Application | 80–100% (every approved resume should become an application) |
| Application → Response | 10–25% (this is the hardest metric to move — expect low early) |
| Response → Interview | 50–80% (if responding, they are likely interested) |
| Interview → Offer | 20–40% (depends heavily on role competition) |

---

## Report Format

Write to `outputs/reviews/YYYY-WW.md`:

```markdown
# Weekly Review — Week [N], [Date Range]

## Pipeline Summary

| Stage | This Week | Last Week | Change |
|-------|-----------|-----------|--------|
| Jobs Discovered | N | N | ↑↓→ |
| Jobs Qualified | N | N | ↑↓→ |
| Resumes Generated | N | N | ↑↓→ |
| Applications Submitted | N | N | ↑↓→ |
| Recruiter Responses | N | N | ↑↓→ |
| Interviews Scheduled | N | N | ↑↓→ |
| Offers | N | N | ↑↓→ |

## Conversion Rates

| Stage | Rate | Benchmark | Status |
|-------|------|-----------|--------|
| Discovery → Qualification | N% | 30–50% | ✓ / ✗ |
| Qualification → Resume | N% | 80–100% | ✓ / ✗ |
| Resume → Application | N% | 80–100% | ✓ / ✗ |
| Application → Response | N% | 10–25% | ✓ / ✗ |
| Response → Interview | N% | 50–80% | ✓ / ✗ |
| Interview → Offer | N% | 20–40% | ✓ / ✗ |

## Weakest Stage

**[Stage name]: [N]% conversion**

[2–3 sentences explaining what this number means and what is likely causing it.]

## Recommendations

1. **[Stage]:** [Specific, actionable change to make next week.]
2. **[Stage]:** [Specific, actionable change.]
3. **[Systemic]:** [One observation about the overall system — agent quality, criteria calibration, source coverage, or timing.]

## Cumulative Totals

| Metric | All Time |
|--------|----------|
| Jobs Discovered | N |
| Applications Submitted | N |
| Responses Received | N |
| Interviews Completed | N |
| Offers Received | N |

## Active Applications

| Job ID | Title | Company | Status | Days Active |
|--------|-------|---------|--------|-------------|
| ...

## Notes

[Anything notable this week that the numbers don't capture: a strong conversation, a promising lead, a rejection worth analyzing, an agent behavior to adjust.]
```

---

## Quality Checklist

Before saving the report:

- [ ] All metrics counted from data files — no estimates
- [ ] Conversion rates calculated correctly (division by zero handled as 0%)
- [ ] Comparison to last week included for every metric
- [ ] Weakest stage identified with specific reasoning
- [ ] Minimum 3 recommendations written — each actionable and specific
- [ ] Cumulative totals accurate
- [ ] Active applications table is current
- [ ] Report saved to outputs/reviews/YYYY-WW.md
- [ ] Metric record appended to data/metrics.csv
- [ ] WeeklyReviewGenerated event logged

---

## Failure Modes

| Condition | Action |
|-----------|--------|
| Data file missing or empty | Note in report. Count as zero. Do not estimate. |
| First week — no prior week to compare | Mark all "Last Week" cells as "—". Skip direction arrows. |
| All conversion rates are zero | Report honestly. This is week one. Recommend: run discovery and qualification before next review. |
| Denominator is zero (e.g. no interviews, so interview→offer rate is undefined) | Report as "—" not 0%. |

---

## Event Format (logs/events.md)

```
## WeeklyReviewGenerated
- week_start: [YYYY-MM-DD]
- artifact_path: outputs/reviews/YYYY-WW.md
- jobs_discovered: N
- jobs_qualified: N
- applications: N
- responses: N
- interviews: N
- offers: N
- weakest_stage: [stage name]
- recorded_at: [date]
```
