# Test Job 010 — Expected: DISQUALIFY (Duplicate URL)

**ID:** test-010
**Title:** Senior Technical Program Manager
**Company:** Google
**URL:** https://careers.google.com/jobs/test-001
**Source:** linkedin
**Location:** Remote, US
**Remote:** true
**Posted:** 2026-06-15

---

## Description

Same role as test-001 but found via LinkedIn with tracking params stripped.
URL after stripping: https://careers.google.com/jobs/test-001 (same as test-001)

---

## Scoring Notes

- URL matches test-001 (after stripping UTM params)
- Expected result: DISQUALIFY — DUPLICATE
- Test purpose: verify deduplication fires before any scoring
- Pass condition: `url_exists()` returns true, job is skipped and logged
- This should NOT create a second database row

---

## Dedup Test Variations

Also test these URL variants — all should resolve to the same canonical URL:
- `https://careers.google.com/jobs/test-001?utm_source=linkedin`
- `https://careers.google.com/jobs/test-001?ref=builtin&utm_campaign=spring`
- `https://careers.google.com/jobs/test-001/` (trailing slash)
