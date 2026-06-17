# Testing Strategy

CareerOS — v0.1

---

## Philosophy

- Small tests. One behavior per test.
- Deterministic. No random data. No time-dependent assertions.
- No hidden state. Each test sets up and tears down its own state.
- Repeatable. The same test must produce the same result every time.

---

## Test Types

### Unit Tests

Location: `tests/unit/` and inline `#[cfg(test)]` blocks in `src/`

Scope: Individual functions and pure logic.

Examples:
- Job deduplication logic
- Qualification scoring function
- Event payload serialization
- Application status transition validator

Rules:
- No database calls.
- No file I/O.
- No network calls.
- Fast. Every unit test must complete in < 10ms.

---

### Integration Tests

Location: `tests/integration/`

Scope: Agent workflows against an in-memory or temporary SQLite database.

Examples:
- Discovery agent writes correct records to DB
- Qualification agent emits `JobQualified` for matching jobs
- Resume agent produces artifact file with correct content
- Event table is append-only (no updates observed)

Rules:
- Use a fresh SQLite database per test (`:memory:` or temp file).
- No network calls. Mock all external sources.
- Assert both DB state and emitted events.

---

### Contract Tests

Location: `tests/contracts/`

Scope: Event payload shapes and agent I/O contracts.

Examples:
- `JobFound` payload matches defined schema
- `ResumeGenerated` payload includes required fields
- Resume output file matches ATS-format spec

Rules:
- Validate against JSON schema defined in `docs/api-contracts/`.
- Fail fast on missing fields.

---

### Snapshot Tests

Location: `tests/snapshots/`

Scope: Agent text outputs (resumes, outreach drafts, interview prep).

Process:
1. Run agent against fixture input.
2. Compare output to stored snapshot file.
3. Fail if output changes unexpectedly.
4. Update snapshot deliberately with `cargo test -- --update-snapshots`.

Rules:
- Snapshots stored in `tests/snapshots/`.
- Never auto-update. Require explicit flag.

---

## Test Data

Location: `tests/fixtures/`

All test inputs are static fixture files.

```
tests/fixtures/
├── jobs/
│   ├── qualified-job.json
│   ├── disqualified-job.json
│   └── duplicate-job.json
├── resumes/
│   └── master-resume.md
├── events/
│   └── job-found-payload.json
└── configs/
    └── qualification-criteria.md
```

Rules:
- No generated test data.
- No random seeds.
- Fixtures are checked into git.

---

## What Is Not Tested

- External job sources (mocked in integration tests)
- Human approval flow (manual process by design)
- File system paths (tested via artifact path assertions only)

---

## Running Tests

```bash
# All tests
cargo test

# Unit only
cargo test --lib

# Integration only
cargo test --test integration

# Specific agent
cargo test discovery

# With output
cargo test -- --nocapture
```

---

## CI Policy (Phase 2+)

- All tests must pass before merge.
- Coverage report generated on each run.
- Snapshot diffs surfaced in PR review.
- No test marked `#[ignore]` without a linked issue.

---

## Quality Gates

A build is not shippable if:

- Any test is failing.
- Any snapshot has an uncommitted diff.
- Any contract test reports a schema violation.
