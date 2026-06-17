# Test Job 009 — Expected: DISQUALIFY (Stale posting > 30 days)

**ID:** test-009
**Title:** Senior Technical Program Manager
**Company:** Acme Corp
**URL:** https://acmecorp.com/jobs/tpm-senior
**Source:** indeed
**Location:** Remote, US
**Remote:** true
**Posted:** 2026-04-10
**Days since posted:** 68

---

## Job Description

Acme Corp is looking for a Senior TPM to drive platform modernization.

**Responsibilities:**
- Lead platform engineering programs
- Stakeholder alignment across product and engineering

**Required qualifications:**
- 5+ years TPM experience
- Experience with cloud infrastructure

**Compensation:** $150,000 – $190,000

---

## Scoring Notes

- Title: exact match
- Posted 68 days ago → exceeds 30-day stale threshold
- Expected result: DISQUALIFY — STALE_POSTING
- Test purpose: verify discovery agent respects recency filter
- Pass condition: job is not added to the pipeline (or is immediately disqualified)
