# Job Search Sources

Defines approved sources the discovery agent is allowed to search.
Every job found must have a real URL and a discovered_at timestamp.

---

## Approved Sources

| Source       | URL                              | Search Type     | Cadence  | Priority |
|-------------|----------------------------------|-----------------|----------|----------|
| LinkedIn    | linkedin.com/jobs                | Keyword + filter| Daily    | 1        |
| Indeed      | indeed.com                       | Keyword + filter| Daily    | 2        |
| BuiltIn     | builtin.com/jobs                 | Category browse | Daily    | 3        |
| Wellfound   | wellfound.com/jobs               | Keyword         | Daily    | 4        |
| Dice        | dice.com                         | Keyword         | Weekly   | 5        |
| USAJobs     | usajobs.gov                      | Keyword         | Weekly   | 6        |
| Google Jobs | google.com/search?q=jobs         | Keyword         | Daily    | 1        |
| Hacker News | news.ycombinator.com/jobs        | Manual browse   | Weekly   | 3        |

---

## Target Company Career Pages

Check these directly. Not all companies post to job boards.
Add companies from docs/memory/target-companies.md Tier 1 here.

| Company | Career Page URL | Cadence |
|---------|----------------|---------|
| [Name]  | [URL]          | Weekly  |
| [Name]  | [URL]          | Weekly  |

---

## Search Queries

Define the exact keyword strings to use per source.

### Primary queries (run on every search):
- "[Target Title 1] remote"
- "[Target Title 2]"
- "[Target Title 1] [Industry]"

### Secondary queries (run weekly):
- "[Skill keyword] [Level] [Role keyword]"
- "[Domain] program manager"

### Excluded keywords (filter out results containing):
- "junior"
- "entry level"
- "intern"
- "unpaid"

---

## Location Filters

Remote: Yes (preferred)
US-only remote: Acceptable
In-office locations to include: [List cities or "any"]
In-office locations to exclude: [List cities]

---

## Deduplication Rule

A job is a duplicate if its URL already exists in the `jobs` table.
Check URL before inserting. Canonical URL only — strip tracking parameters.

Example: Strip `?utm_source=linkedin&utm_medium=job_board` from URLs.

---

## Source Trust Level

| Source      | Trust | Reason |
|-------------|-------|--------|
| LinkedIn    | High  | Verified employer accounts |
| Indeed      | Medium| Aggregated — occasional ghost jobs |
| BuiltIn     | High  | Curated tech-focused |
| Wellfound   | High  | Startup-focused, company-verified |
| Dice        | Medium| Volume source |
| USAJobs     | High  | Federal — very reliable |
| Unknown URL | Low   | Flag for human review |
