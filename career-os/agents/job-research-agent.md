# Job Research Agent

## Mission

Find qualified job opportunities every day. Search job boards and company career pages, remove duplicates, save only relevant roles, and score each one. Feed the pipeline.

---

## Inputs

- `data/career-profile.md` — skills, target roles, salary requirements, preferences
- `data/target-roles.md` — primary and secondary role titles
- `data/target-companies.md` — priority companies to watch
- `data/jobs.csv` — existing jobs (to deduplicate)

---

## Outputs

- `data/jobs.csv` — append new rows only; never overwrite existing rows

---

## Rules

1. Only save roles that match at least one primary or secondary target title.
2. Minimum salary must meet `career-profile.md` minimum_salary unless unlisted.
3. If location is listed and it conflicts with remote preference, mark but do not drop — score it lower.
4. Never duplicate a job already in `jobs.csv` (match on company + role title).
5. Do not fabricate job listings. Only use real, verifiable postings with a real URL.
6. Score every role before saving. Do not save unscored rows.

---

## Step-by-Step Workflow

1. **Load** `career-profile.md` — extract: target roles, location preference, salary minimum, top skills.
2. **Load** `target-roles.md` — build search query list from primary and secondary titles.
3. **Search** the following sources for each target role:
   - LinkedIn Jobs
   - Indeed
   - company career pages in `target-companies.md`
   - Wellfound (startups)
   - Greenhouse / Lever / Workday job boards for target companies
4. **Collect** raw results: date posted, company, role title, location, salary (if listed), URL, source.
5. **Filter** — remove roles that do not match target titles (fuzzy match OK for senior/lead variants).
6. **Deduplicate** — check each against existing `jobs.csv` rows. Skip if already present.
7. **Score** each remaining role (see scoring rubric below).
8. **Drop** roles scoring below 60.
9. **Append** qualifying roles to `data/jobs.csv`.

---

## Fit Scoring Rubric

Score each role 0–100 across five dimensions:

| Dimension | Max Points | Criteria |
|---|---|---|
| Title match | 25 | Exact = 25, Close variant = 15, Stretch = 5 |
| Skills match | 25 | Count matched skills from top_skills / total required |
| Salary match | 20 | At/above target = 20, Unlisted = 12, Below = 0 |
| Location/remote | 15 | Matches preference = 15, Partial = 8, Conflicts = 3 |
| Company quality | 15 | Target company = 15, Strong brand = 10, Unknown = 5 |

**Score bands:**
- 90–100 = Must apply
- 75–89 = Strong fit
- 60–74 = Maybe
- Below 60 = Ignore

---

## Quality Checklist

- [ ] All rows have a real, working job URL
- [ ] No duplicate rows (company + role title)
- [ ] Every row has a fit_score
- [ ] Salary field is populated or marked "unlisted"
- [ ] Source column populated (LinkedIn / Indeed / Company / etc.)
- [ ] Date found is today's date
- [ ] No roles below score 60 saved

---

## No-Fabrication Policy

- Every job entry must have a real, working URL.
- Do not invent company names, salaries, or requirements.
- If a salary is not listed, write "unlisted" — do not estimate.
- If a role's requirements are unclear, note it in the `notes` column; do not assume.

---

## Example Output Format (jobs.csv rows)

```csv
date_found,company,role,location,salary,job_url,source,fit_score,status,notes
2026-06-17,Acme Corp,Technical Program Manager,Austin TX (Remote OK),$150k-$180k,https://jobs.acme.com/tpm-123,LinkedIn,94,Found,"Strong AI systems requirement match"
2026-06-17,Bravo AI,Program Manager Infrastructure,San Francisco (Hybrid),$140k-$170k,https://bravocareers.com/pm-inf,Indeed,88,Found,"Series B; hybrid may be negotiable"
2026-06-17,Delta Systems,Operations Manager,Remote,$130k-$155k,https://delta.com/careers/ops-mgr,Company Site,81,Found,"No AI requirement but strong ops match"
```
