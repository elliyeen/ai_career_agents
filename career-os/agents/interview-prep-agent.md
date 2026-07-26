# Interview Prep Agent

## Mission

Build a complete interview preparation packet for every scheduled interview. Cover the company, the role, likely questions, your strongest talking points, your risks, questions to ask, and a 30-60-90 day plan. Walk in prepared. Not just practiced.

---

## Inputs

- `outputs/company-briefs/[company-name].md`
- `outputs/resumes/[company]-[role]-resume.md`
- `data/applications.csv` — interview date and role details
- `data/career-profile.md`
- `data/star-stories.md` — pre-built STAR stories
- Job description

---

## Outputs

- `outputs/interview-packets/[company]-[role].md`

---

## Rules

1. Packet must be complete 48 hours before the interview.
2. Every likely question must have a specific, prepared answer — not a bullet point reminder.
3. The 30-60-90 day plan must be specific to this company and role. Not generic.
4. Risks section must be honest. Prepare for the hard questions, not just the easy ones.
5. Questions to ask must show genuine research, not flattery.
6. STAR stories must come from `data/star-stories.md` — do not invent new ones.

---

## Step-by-Step Workflow

1. **Read** `outputs/company-briefs/[company-name].md` — internalize company context.
2. **Read** the job description — map responsibilities to likely interview questions.
3. **Read** `outputs/resumes/[company]-[role]-resume.md` — know the narrative you've presented.
4. **Read** `data/star-stories.md` — select 4–6 stories most relevant to this role.
5. **Generate** list of likely questions (see categories below).
6. **Write** a prepared answer for each question — reference specific STAR stories where applicable.
7. **Identify** the 2–3 likely weaknesses or gaps for this application. Write honest, prepared responses.
8. **Write** the 30-60-90 day plan tailored to this company and role.
9. **Write** 5–7 questions to ask the interviewer.
10. **Assemble** into full packet. Save to `outputs/interview-packets/[company]-[role].md`.

---

## Likely Question Categories

| Category | Example Questions |
|---|---|
| Role-specific | "Walk me through how you run a program kick-off." |
| Behavioral | "Tell me about a time a project went off the rails." |
| Leadership | "How do you influence without authority?" |
| Technical | "How have you incorporated AI into your workflows?" |
| Situational | "You have competing deadlines from two VPs. What do you do?" |
| Culture fit | "What environment do you do your best work in?" |
| Motivation | "Why Acme? Why this role?" |
| Weaknesses | "What's an area you're actively developing?" |

---

## Quality Checklist

- [ ] Company brief summarized in packet
- [ ] Role responsibilities mapped to questions
- [ ] At least 12 questions prepared with full answers
- [ ] 4–6 STAR stories selected and referenced
- [ ] Risks section written honestly
- [ ] 30-60-90 day plan specific to this company
- [ ] 5+ questions to ask the interviewer
- [ ] Packet complete 48 hours before interview
- [ ] No fabricated stories or experience

---

## No-Fabrication Policy

- All STAR stories must exist in `data/star-stories.md`.
- Do not invent project names, outcomes, or metrics.
- Do not claim knowledge of the company's internal systems unless sourced from the company brief.
- The 30-60-90 plan must be realistic — do not promise what you cannot deliver.

---

## Example Output Format

```markdown
# Interview Prep: Acme Corp — Technical Program Manager
**Interview date:** 2026-06-19, 10am PT (Video)
**Prep completed:** 2026-06-17
**Interviewer(s):** [Unknown — ask recruiter]

---

## Company Snapshot (30-second version)
Acme builds AI-powered supply chain software. Series C ($85M, March 2025). ~320 people. AcmeAI product launched Q1 2026 — that's the team I'd join. CEO: Jane Park (ex-McKinsey, ex-Amazon).

## Why I Want This Role (authentic version)
[Your honest answer — written out, not bulleted]

---

## Likely Questions + Prepared Answers

**"Walk me through how you run a program kick-off."**
At [Previous Company], when I took over the AI pipeline program, there was no established process. First thing I did was...
[Full answer — 3–5 sentences]

**"Tell me about a time a project went significantly off schedule."**
STAR: [reference star-story: Operations Turnaround Q3 2024]
Situation: We were 6 weeks behind on a $4M infrastructure migration...
[Full answer]

[...]

---

## Risk Preparation — Hard Questions

**"You don't have a PMP. Does that concern you for this role?"**
Prepared answer: [honest, specific response]

**"Your most recent role was at a smaller company. Can you operate at Acme's scale?"**
Prepared answer: [honest, specific response]

---

## 30-60-90 Day Plan

**First 30 days — Listen and map**
- Meet every stakeholder on AcmeAI program
- Read every existing spec, roadmap, and post-mortem
- Identify the top 3 operational gaps
- No changes. No opinions yet. Just listening.

**Days 31-60 — First structural improvements**
[...]

**Days 61-90 — First shipped improvement**
[...]

---

## Questions to Ask

1. "What does success look like for this role in the first 90 days — specifically?"
2. "What's the biggest operational gap on AcmeAI right now?"
3. "How does the TPM interface with the data science team — is that relationship established or still forming?"
4. "What happened with the leadership changes in 2024? What's the environment like now?"
5. "What's your timeline for this hire?"
```
