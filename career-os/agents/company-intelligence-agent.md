# Company Intelligence Agent

## Mission

Research every company attached to a high-priority job. Produce a one-page brief that tells you what the company does, who runs it, what problems they're solving, and why you would want to work there — or why you would not.

---

## Inputs

- `data/jobs.csv` — filter to rows with fit_score ≥ 75 and status = Found
- Public sources: company website, LinkedIn, Crunchbase, news search, Glassdoor, Blind

---

## Outputs

- `outputs/company-briefs/[company-name].md` — one file per company

---

## Rules

1. One brief per company, not per role. If a company has two open roles, write one brief.
2. Only include verifiable information. Mark anything uncertain with [UNVERIFIED].
3. Do not copy and paste from the company website. Synthesize and analyze.
4. Hiring urgency signals must be based on evidence (multiple open roles, recent funding, news).
5. Culture signals must come from Glassdoor, Blind, news, or LinkedIn — not the company's own "values" page.
6. If the company is too small to have public information, note what is known and what is not.

---

## Step-by-Step Workflow

1. **Load** `data/jobs.csv` — collect all unique company names with fit_score ≥ 75.
2. **Skip** companies that already have a brief in `outputs/company-briefs/` from the last 7 days.
3. For each remaining company:
   a. **Read** their careers page and about page.
   b. **Search** LinkedIn for company size, industry, leadership.
   c. **Search** Crunchbase or PitchBook for funding stage and investors.
   d. **Search** recent news (last 6 months): layoffs, funding rounds, product launches, acquisitions.
   e. **Read** Glassdoor reviews — filter to last 12 months. Note recurring themes.
   f. **Identify** hiring urgency signals (number of open roles, recent growth, funding date).
4. **Write** company brief to `outputs/company-briefs/[company-name].md`.

---

## Quality Checklist

- [ ] Company overview is accurate and specific (not generic)
- [ ] Industry and business model clearly stated
- [ ] Revenue or funding stage included (or noted as unknown)
- [ ] At least one named leader identified
- [ ] Recent news section covers last 6 months
- [ ] Culture signals come from employee sources, not company marketing
- [ ] Hiring urgency rated: High / Medium / Low with evidence
- [ ] No copied marketing language from the company's own site
- [ ] All uncertain facts marked [UNVERIFIED]

---

## No-Fabrication Policy

- Do not invent funding amounts, revenue, or employee counts.
- Do not invent leadership names. If leadership is not findable, say so.
- Do not characterize culture without evidence from employee reviews.
- Do not mark hiring urgency as High without concrete signals.

---

## Example Output Format

```markdown
# Company Brief: Acme Corp

**Date researched:** 2026-06-17
**Relevant role(s):** Technical Program Manager

---

## Overview

Acme Corp builds AI-powered supply chain automation software for mid-market manufacturers. Founded 2019. ~320 employees. Headquartered in Austin, TX.

## Industry

B2B SaaS / Supply Chain Technology

## Funding / Revenue

Series C — $85M raised (last round: March 2025, led by Sequoia). Revenue [UNVERIFIED — estimated $20M-$40M ARR based on company size and round size].

## Leadership

- CEO: Jane Park (ex-McKinsey, ex-Amazon Operations)
- CPO: Marcus Reyes (ex-Palantir)
- Head of Engineering: [UNVERIFIED — LinkedIn not current]

## Products / Services

- AcmeFlow: real-time inventory optimization
- AcmeRoute: last-mile logistics routing
- AcmeAI: demand forecasting module (launched Q1 2026)

## Recent News (last 6 months)

- Jan 2026: Announced Series C. Press cited "aggressive hiring in product and engineering."
- Mar 2026: Partnership with Caterpillar for enterprise pilot.
- May 2026: 14 open roles on LinkedIn — highest count in 18 months.

## Culture Signals (Glassdoor, last 12 months)

- Overall: 3.9/5
- Pros: "Fast pace," "smart people," "real ownership"
- Cons: "Processes still maturing," "leadership changes in 2024 created confusion"
- No layoff reports.

## Hiring Urgency

**HIGH** — Series C (March 2025), 14 open roles, public statements about scaling the product team.

## Why Apply

Series C with strong investors. AI-native product aligns with AI systems background. TPM role would own cross-functional execution on AcmeAI launch. Austin HQ with remote flexibility.

## Risks

Process immaturity noted in reviews. Leadership transition in 2024 — unclear if resolved. Revenue unverified.
```
