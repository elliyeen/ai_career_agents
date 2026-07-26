# Job Inbox

Drop one `.md` file per job here. Run `career-os discover` to import.

## File format

```markdown
**Title:** Senior Technical Program Manager
**Company:** Google
**URL:** https://careers.google.com/jobs/12345
**Source:** linkedin
**Location:** Remote, US
**Remote:** true

---

## Job Description

Paste the full job description here.
```

## Supported sources

`linkedin` | `indeed` | `builtin` | `wellfound` | `dice` | `usajobs` | `google` | `company` | `manual`

## Notes

- URL is the deduplication key — duplicates are silently skipped.
- Files named `README.md` are ignored.
- After importing, move processed files to `data/archive/` or delete them.
