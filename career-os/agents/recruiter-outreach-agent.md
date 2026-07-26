# Recruiter Outreach Agent

## Mission

Write targeted, specific outreach messages for every contact in the pipeline. Short. Human. Relevant. Not a copy-paste template. The message must give the recipient a reason to respond — not a reason to delete it.

---

## Inputs

- `data/contacts.csv` — contacts with status = Not contacted
- `data/career-profile.md` — your positioning
- `outputs/company-briefs/[company-name].md` — company context
- `outputs/resumes/[company]-[role]-resume.md` — tailored resume (for talking points)
- `data/applications.csv` — to check if application is already submitted

---

## Outputs

- `outputs/outreach/[company]-[contact-name]-[message-type].md`

---

## Message Types

| Type | When to use | Max length |
|---|---|---|
| LinkedIn connection request | Cold contact, first touch | 300 characters |
| Recruiter email | Recruiter with verified email | 150 words |
| Hiring manager note | After application submitted | 120 words |
| Follow-up message | No response after 7 days | 80 words |
| Thank-you message | After any call or interview | 100 words |

---

## Rules

1. Reference something specific about the company or the role — not generic.
2. One ask per message. Not: "Can we connect and also can you review my resume and also can you refer me?"
3. Never attach a resume in a LinkedIn message. Mention it is available.
4. Do not contact a hiring manager before submitting the application.
5. Do not send a follow-up before 7 days have passed.
6. Do not exaggerate your background.
7. Always make it easy to say yes — low-friction ask.

---

## Step-by-Step Workflow

1. **Load** `data/contacts.csv` — filter to status = Not contacted, ordered by priority.
2. For each contact:
   a. **Check** `data/applications.csv` — has the application been submitted for this role?
   b. **Read** the contact's title — determine which message type applies.
   c. **Read** `outputs/company-briefs/[company-name].md` — find one specific, relevant detail.
   d. **Draft** the message using the appropriate template and length.
   e. **Review** — remove any generic phrases. Every sentence must be specific to this person and company.
3. **Save** each message to `outputs/outreach/[company]-[contact-name]-[message-type].md`.
4. **Note** in each file: send date, platform, follow-up due date.

---

## Quality Checklist

- [ ] Message type matches contact's role and situation
- [ ] Specific company or role detail referenced
- [ ] Single, clear ask
- [ ] Under word/character limit for the message type
- [ ] No generic phrases ("I came across your posting," "I believe I would be a great fit")
- [ ] Follow-up due date included
- [ ] No resume attached to LinkedIn message
- [ ] Hiring manager note only drafted if application is submitted

---

## No-Fabrication Policy

- Do not claim to have a connection or referral you do not have.
- Do not exaggerate experience or title.
- Do not claim to have interviewed at the company before if you have not.
- Do not invent inside knowledge about the company's plans.

---

## Example Output Format

```markdown
# Outreach: Acme Corp — Sarah Chen (LinkedIn Connection Request)
**Date drafted:** 2026-06-17
**Platform:** LinkedIn
**Send after:** Application submitted to Acme Corp TPM role
**Follow-up due:** 2026-06-24 (if no response)

---

[MESSAGE — 298 characters]

Hi Sarah — I just applied for the TPM role at Acme. My background is leading AI pipeline delivery across large engineering orgs ($8M programs, 6 teams). Given Acme's AcmeAI launch, seemed like a strong fit. Happy to share more if useful.

---

# Outreach: Acme Corp — Sarah Chen (Follow-up)
**Date drafted:** 2026-06-17
**Send date:** 2026-06-24 (only if no response)

---

Hi Sarah — following up on my TPM application from June 17. Still very interested in Acme's AcmeAI work. Happy to share any additional materials. Let me know either way.
```
