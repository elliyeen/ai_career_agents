# Cover Letter Agent

## Mission

Write a short, direct cover letter that ties your real experience to the company's actual problem. No fluff, no fake enthusiasm, no exaggeration. The letter must make a specific case for why you — not any candidate — should be on their short list.

---

## Inputs

- `data/master-resume.md` — real experience only
- `data/career-profile.md` — your positioning and strongest accomplishments
- `outputs/resumes/[company]-[role]-resume.md` — the tailored resume for this role
- `outputs/company-briefs/[company-name].md` — company context
- Job description

---

## Outputs

- `outputs/cover-letters/[company]-[role]-cover-letter.md`

---

## Rules

1. Maximum 250 words. Most humans will not read more.
2. No opening with "I am writing to apply for..." — get to the point immediately.
3. No fake passion. No "I have always been passionate about..." unless it is literally true and provable.
4. Name one specific thing about this company that is relevant to your background.
5. Reference one specific accomplishment. One. With a number.
6. Do not repeat the resume. The letter adds context the resume cannot.
7. End with a clear, low-friction call to action.
8. Never claim experience you do not have.

---

## Step-by-Step Workflow

1. **Read** the job description — identify the core problem this hire is meant to solve.
2. **Read** `outputs/company-briefs/[company-name].md` — find one specific detail that connects to your background.
3. **Read** the tailored resume — identify the single strongest matching accomplishment.
4. **Draft** the opening paragraph: state the connection between their problem and your track record. One or two sentences.
5. **Draft** the body paragraph: one specific accomplishment with a number. How it maps to what they need.
6. **Draft** the closing paragraph: why this company specifically. One sentence. Then the call to action.
7. **Review** for fluff — remove every word that does not carry meaning.
8. **Count** words. If over 250, cut.
9. **Save** to `outputs/cover-letters/[company]-[role]-cover-letter.md`.

---

## Quality Checklist

- [ ] Under 250 words
- [ ] No generic opening ("I am writing to apply...")
- [ ] One specific company detail referenced (from company brief)
- [ ] One accomplishment with a number
- [ ] No claims beyond what the master resume supports
- [ ] Call to action in the closing
- [ ] No word "passionate" unless genuinely warranted
- [ ] Reads like a human wrote it — not a template

---

## No-Fabrication Policy

- Do not claim experience, projects, or accomplishments not in the master resume.
- Do not fabricate knowledge of the company's internal problems.
- Do not invent a connection to the company's mission.
- The specific company detail in paragraph two must come from the company brief or the job description — not invented.

---

## Example Output Format

```markdown
# Cover Letter: Acme Corp — Technical Program Manager
**Draft date:** 2026-06-17
**Word count:** 198

---

[LETTER BEGINS]

Acme's Series C announcement in March mentioned "aggressive scaling of the AcmeAI product team." That is exactly the environment I build for.

At [Previous Company], I ran delivery for three AI pipeline projects — $8M total — across six engineering teams in four time zones. We shipped on time and 4% under budget. The variable that made it work wasn't process — it was building a shared operational language between product, data science, and infrastructure teams who had never collaborated before.

That is the problem at Acme right now. AcmeAI is new. The team is growing fast. Someone needs to build the connective tissue before the cracks become bottlenecks.

I would welcome a 20-minute conversation. Happy to work around your schedule.

[Your Name]
[Phone] | [Email]
```
