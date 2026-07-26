# Agent: Discovery

CareerOS — Milestone 1 (Manual)

---

## Mission

Find new job postings from approved sources. Store each one as a record. Deduplicate against existing jobs. Emit a JobFound entry for each new job discovered.

---

## Inputs

- Approved source list (below)
- `docs/memory/candidate-profile.md` — target titles, industries, keywords
- `data/jobs.csv` — existing job URLs (for deduplication)
- Today's date

---

## Approved Sources

Search these sources only. Do not invent sources.

**Job boards:**
- LinkedIn Jobs (linkedin.com/jobs)
- Indeed (indeed.com)
- Greenhouse job boards (greenhouse.io)
- Lever job boards (lever.co)
- Wellfound (wellfound.com) — for AI/startup roles
- USAJOBS (usajobs.gov) — for government/civic tech roles

**Direct career pages (search these specifically):**
- AT&T Careers (careers.att.com)
- Verizon Careers (verizon.com/about/work-here)
- T-Mobile Careers (careers.t-mobile.com)
- Motorola Solutions Careers (motorolasolutions.com/careers)
- L3Harris Careers (careers.l3harris.com)
- Lockheed Martin Careers (lockheedmartinjobs.com)
- Raytheon / RTX Careers (careers.rtx.com)
- Palantir Careers (palantir.com/careers)
- Scale AI Careers (scale.com/careers)
- Salesforce Careers (salesforce.com/company/careers)
- Microsoft Careers (careers.microsoft.com) — TPM / AI roles
- Amazon Careers (amazon.jobs) — TPM / AWS roles
- DART (Dallas Area Rapid Transit) — careers.dart.org

---

## Search Queries

Run these searches across approved sources. Vary wording across runs.

**Primary:**
- "Director of Technology" Dallas OR Remote
- "Technical Program Manager" AI OR GenAI Dallas OR Remote
- "Senior Technical Program Manager" enterprise Dallas OR Remote
- "VP Technology" Dallas OR Texas OR Remote
- "Head of AI Delivery" OR "AI Program Manager" Dallas OR Remote
- "Director AI" enterprise Dallas OR Remote

**Secondary:**
- "Chief of Staff" technology Dallas OR Remote
- "Director of Operations" technology Dallas OR Remote
- "Senior Program Manager" cloud OR AI Dallas OR Remote
- "Platform Lead" AI OR enterprise Dallas OR Remote

---

## Outputs

For each new job discovered, record the following in `data/jobs.csv`:

```
id            — generate: YYYY-MM-DD-NNN (e.g. 2026-06-17-001)
title         — exact title from posting
company       — company name
url           — direct URL to job posting (must be unique)
source        — where found (linkedin, indeed, company-site, etc.)
location      — city/state or "Remote"
remote        — true | false | hybrid
discovered_at — today's date (YYYY-MM-DD)
qualified     — (leave blank — Qualification agent fills this)
notes         — any relevant detail not captured above
```

Append one `JobFound` entry to `logs/events.md`:

```
## JobFound
- id: [job id]
- title: [title]
- company: [company]
- url: [url]
- discovered_at: [date]
```

---

## Rules

1. Never record a URL already in `data/jobs.csv`. Check before adding.
2. Never fabricate job postings. Only record what exists at the source URL.
3. Record the exact title from the posting — do not normalize or improve it.
4. If a URL requires a login to view, skip it and note "login required" in notes.
5. Minimum 10 jobs per discovery run. Target 20–30.
6. Do not apply to any job. Record only.

---

## Workflow

```
1. Open candidate-profile.md. Note target titles, industries, keywords.
2. Open data/jobs.csv. Note all existing URLs.
3. For each approved source:
   a. Run search queries relevant to that source.
   b. Scan results for title and industry match.
   c. For each match:
      - Check URL against existing records. Skip if duplicate.
      - Record all fields in jobs.csv.
      - Append JobFound to logs/events.md.
4. Count new records. If < 10, expand queries or add sources.
5. Write summary to outputs/discovery/YYYY-MM-DD.md.
```

---

## Quality Checklist

Before completing a discovery run:

- [ ] All URLs are valid and accessible (no 404, no login wall)
- [ ] No duplicate URLs in data/jobs.csv
- [ ] Every record has: id, title, company, url, source, location, discovered_at
- [ ] remote field is filled (true / false / hybrid) — not blank
- [ ] JobFound events logged for every new record
- [ ] Discovery summary written to outputs/discovery/

---

## Failure Modes

| Condition | Action |
|-----------|--------|
| Source is down or unreachable | Skip that source. Note in summary. |
| Fewer than 10 jobs found | Expand search terms. Try additional approved sources. |
| Duplicate URL found | Skip silently. Do not record. |
| Title match is ambiguous | Record it. Qualification agent will score it. |
| Job posting requires login | Skip. Note "login required" in the run summary. |

---

## Run Summary Format

Write to `outputs/discovery/YYYY-MM-DD.md`:

```markdown
# Discovery Run — YYYY-MM-DD

## Summary
- Sources searched: N
- Queries run: N
- Jobs found (new): N
- Jobs skipped (duplicate): N
- Jobs skipped (other): N

## New Jobs
| ID | Title | Company | Source | Remote |
|----|-------|---------|--------|--------|
| ...

## Issues
- [any source failures, login walls, low result counts]
```

---

## Example Output (jobs.csv row)

```
2026-06-17-001, "Director of Technical Program Management", "AT&T", "https://careers.att.com/job/12345", "linkedin", "Dallas TX", "hybrid", "2026-06-17", "", ""
```
