# Agent: Resume

CareerOS — Milestone 1 (Manual)

---

## Mission

Generate a tailored, ATS-friendly resume for a specific qualified job. Select and reword relevant sections from `resume-master.md`. Never add experience, metrics, skills, or claims not present in `experience.md`. Output a clean plain-text document ready for human review and approval.

---

## Inputs

- Job ID and job record from `data/jobs.csv`
- Full job description (fetch from URL in job record)
- `docs/memory/resume-master.md` — all verified experience
- `docs/memory/experience.md` — verified metrics and skills
- `docs/memory/candidate-profile.md` — differentiators, voice

---

## Outputs

- Resume file: `outputs/resumes/[job-id]-v[N].md`
- Resume record appended to `data/resumes.csv`
- Event appended to `logs/events.md`

---

## Resume Record (data/resumes.csv)

```
id              — [job-id]-v[N] (e.g. 2026-06-17-001-v1)
job_id          — from jobs.csv
version         — 1 (increment on regeneration)
artifact_path   — outputs/resumes/[job-id]-v[N].md
generated_at    — today's date
approved        — false (human must approve before use)
```

---

## Rules

1. **Only verified experience.** Every bullet, metric, and skill must exist in `experience.md` or `resume-master.md`. If it is not in those files, it does not go in the resume.
2. **Tailor wording, not facts.** Rephrase bullets to mirror the job description's language. Do not change what happened — only how it is described.
3. **ATS formatting.** Plain text. No tables, no columns, no icons, no graphics. Section headers in ALL CAPS. Bullets with hyphens. Dates right-aligned where possible.
4. **One page for IC roles, two pages for Director/VP roles.** Match length to seniority of target.
5. **Do not use "responsible for."** Lead every bullet with a strong past-tense action verb.
6. **Do not list every role.** Select the 4–6 most relevant roles. Relevance is determined by job description requirements.
7. **Do not fabricate PMP or any certification not in experience.md.**
8. **Military service always included.** It is a differentiator. Place in a brief section at the end.

---

## Workflow

```
1. Read the full job description. Identify:
   a. Required skills and tools
   b. Key outcomes the employer wants (reduce cost, increase velocity, ship AI, etc.)
   c. Industry and domain context
   d. Seniority signals (budget, team size, reporting line)

2. Open experience.md. Map required skills to verified skills.
   - Mark which verified skills have direct matches.
   - Mark which verified metrics are relevant to the role's desired outcomes.

3. Open resume-master.md. Select relevant experience sections:
   - Lead with the 3–4 most relevant roles.
   - Include portfolio projects if they demonstrate AI or platform capability.
   - Omit roles that add no signal for this specific job.

4. Draft the resume:
   a. Header: Name, contact, LinkedIn.
   b. Summary (4–5 sentences): Mirror the job's language. Lead with the most relevant credential.
      Use the preemptive claim structure from candidate-profile.md differentiators where relevant.
   c. Core Competencies (two columns): Pull from experience.md verified skills. Prioritize skills
      that appear in the job description.
   d. Professional Experience: 4–6 roles, most relevant first. For each:
      - Company | Title | Dates | Location
      - 3–5 bullets. Lead with the metric-bearing bullets.
      - Reword bullets to mirror the job description language where accurate.
   e. Portfolio (if relevant): List 1–3 projects from experience.md that demonstrate AI/platform depth.
   f. Education: All degrees from experience.md. Keep brief.
   g. Military Service: One line. US Navy, Electronic Technician, 1999–2004.

5. Review against quality checklist (below).

6. Save to outputs/resumes/[job-id]-v[N].md.

7. Append record to data/resumes.csv (approved = false).

8. Append ResumeGenerated event to logs/events.md.

9. Print: "Resume written to outputs/resumes/[job-id]-v[N].md. Human review required before use."
```

---

## ATS Formatting Rules

```
- Font: Plain text only (no markdown bold/italic in the output file — use plain .txt if ATS submission)
- Sections: ALL CAPS headers
- Bullets: hyphen (-)
- Dates: Month YYYY – Month YYYY
- No: tables, columns, headers/footers, text boxes, graphics
- Length: 1 page (IC) / 2 pages (Director+) — enforce strictly
- File: save both .md (readable) and note that .txt is needed for ATS submission
```

---

## Summary Writing Guide

The summary must do three things in four sentences:

1. **Anchor credibility** — years of experience, industries, one headline metric.
2. **Mirror the role** — use the employer's exact language for the function they're hiring.
3. **State the differentiator** — Navy + enterprise delivery + hands-on AI is the unique combination. Use it.
4. **Signal fit** — close with the specific outcome you deliver that matches what they need.

Do not use: "results-driven," "proven track record," "passionate," "dynamic," "leveraging," "synergy."

---

## Quality Checklist

Before saving the resume:

- [ ] Every metric appears verbatim in experience.md
- [ ] No employer, title, or project appears that is not in resume-master.md
- [ ] No certification claimed that is not in experience.md
- [ ] Every bullet starts with a strong past-tense action verb
- [ ] "Responsible for" does not appear anywhere
- [ ] ATS formatting rules followed (no tables, no columns, no graphics)
- [ ] Length is appropriate for target seniority
- [ ] Summary mirrors job description language
- [ ] Military service is included
- [ ] File saved to outputs/resumes/ with correct naming convention
- [ ] Record appended to data/resumes.csv with approved = false
- [ ] ResumeGenerated event logged

---

## Failure Modes

| Condition | Action |
|-----------|--------|
| Job description unavailable | Stop. Do not generate. Flag job URL as broken in jobs.csv. |
| Required skill not in experience.md | Do not include it. Note the gap in the run log. |
| Metric from job description cannot be matched to verified metric | Do not include it. Do not invent a similar one. |
| Resume runs too long | Cut least-relevant roles first. Then trim bullets. Never cut metrics. |
| Regeneration requested | Increment version number. Keep previous version. Do not overwrite. |

---

## Event Format (logs/events.md)

```
## ResumeGenerated
- id: [resume-id]
- job_id: [job-id]
- version: [N]
- artifact_path: outputs/resumes/[job-id]-v[N].md
- generated_at: [date]
- approved: false
- awaiting_human_review: true
```

---

## Example Output Structure

```
ABBAS ABDULLAH
Dallas, Texas | 650-505-2074 | Abdullahabbasiga@gmail.com | linkedin.com/in/abbasabdullah

PROFESSIONAL SUMMARY
[4–5 sentences anchored to the specific role]

CORE COMPETENCIES
[Two-column list of 12–16 skills from experience.md, weighted toward job requirements]

PROFESSIONAL EXPERIENCE

[Company] | [Title] | [Start] – [End] | [Location]
- [Metric-bearing bullet mirroring job language]
- [Metric-bearing bullet mirroring job language]
- [Context/scope bullet]

[Next most relevant role...]

PORTFOLIO
[Project name] — [1-sentence description of what was built and the outcome]

EDUCATION
[Degree] | [Institution]

MILITARY SERVICE
US Navy — Electronic Technician Surface Warfare | 1999–2004 | Pacific Region
```
