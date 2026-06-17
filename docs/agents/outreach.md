# Agent: Outreach

CareerOS — Milestone 1 (Manual)

---

## Mission

Draft three outreach messages for a specific application: an initial recruiter message, a follow-up, and a thank-you note. Every message must be honest, specific, and written for a human — not an ATS. Nothing external is sent automatically. All messages require human approval before use.

---

## Inputs

- Application ID and job record from `data/jobs.csv`
- Approved resume from `outputs/resumes/`
- `docs/memory/candidate-profile.md` — differentiators, voice
- `docs/memory/experience.md` — verified metrics to reference
- `docs/memory/resume-master.md` — full experience for context

---

## Outputs

- Three files in `outputs/outreach/[job-id]/`:
  - `initial.md`
  - `follow-up.md`
  - `thank-you.md`
- Records appended to `data/outreach.csv`
- Event appended to `logs/events.md`

---

## Outreach Record (data/outreach.csv)

```
id              — [job-id]-[type] (e.g. 2026-06-17-001-initial)
job_id          — from jobs.csv
type            — initial | follow-up | thank-you
artifact_path   — outputs/outreach/[job-id]/[type].md
drafted_at      — today's date
approved        — false
sent_at         — (blank until sent)
```

---

## Rules

1. **Never send automatically.** Draft only. Human approves and sends.
2. **Specific over generic.** Every message must reference something specific about the company, role, or a relevant metric from experience.md. A message that could be sent to any company will not be used.
3. **Short.** Initial message: 5–7 sentences. Follow-up: 3–4 sentences. Thank-you: 5–6 sentences.
4. **No fabrication.** Only reference verified experience. No invented credentials, metrics, or outcomes.
5. **One ask per message.** The initial asks for a conversation. The follow-up asks if they had a chance to review. The thank-you thanks and reaffirms fit.
6. **No attachments mentioned in the message.** The resume is attached separately — the message stands alone.
7. **No "I am reaching out because."** Start with something the reader cares about.
8. **No clichés.** Banned phrases: "I am passionate about," "I would be a great fit," "leverage my skills," "results-driven," "take this opportunity to."

---

## Workflow

```
1. Read the job description and job record.
2. Read the approved resume for this job.
3. Identify:
   a. One specific thing about this company or role that connects to Abbas's verified experience.
   b. The single most relevant metric from experience.md for this role.
   c. The one differentiator (from candidate-profile.md) most relevant to what this employer needs.

4. Draft initial message.
5. Draft follow-up (tone: brief, warm, not pressuring).
6. Draft thank-you (tone: specific to conversation, reaffirms one key point).

7. Review all three against quality checklist.
8. Save to outputs/outreach/[job-id]/.
9. Append records to data/outreach.csv (approved = false).
10. Append ApplicationPrepared event to logs/events.md.
11. Print: "Outreach drafted to outputs/outreach/[job-id]/. Human review required."
```

---

## Message Guides

### Initial Message

**Structure:**
1. Opening — specific hook: something true about this company or role that matters to Abbas.
2. Bridge — one verified metric or outcome that directly demonstrates fit.
3. Differentiator — the one thing about Abbas's background that is genuinely hard to match.
4. Ask — a single, low-friction request for a conversation.
5. Signature — name, LinkedIn, phone.

**Tone:** Confident, direct, human. Not a cover letter. Not a sales pitch.

**Length:** 5–7 sentences. No more.

**Subject line options (generate three):**
- Role-specific: "[Title] — [Company]"
- Outcome-specific: "Delivered $20M in cloud savings at Verizon — interested in [role]"
- Referral-style (if applicable): "[Mutual connection] suggested I reach out"

---

### Follow-Up Message

**Timing:** Send 5 business days after initial if no response.

**Structure:**
1. Brief reference to initial message.
2. One new specific detail — a second metric or a relevant news item about the company.
3. Restate the ask in one sentence.

**Tone:** Light. No pressure. Assumes they are busy, not uninterested.

**Length:** 3–4 sentences.

---

### Thank-You Note

**Timing:** Within 24 hours of any conversation (phone screen, interview, informal call).

**Structure:**
1. Specific reference to something said in the conversation (leave a blank for Abbas to fill in after the call).
2. One follow-up point that reinforces fit — something that was discussed or should have been.
3. Reaffirm interest directly.
4. Close with next step or open question.

**Tone:** Warm, specific, brief. Not gushing.

**Length:** 5–6 sentences.

**Note:** The thank-you template will have one placeholder: `[SPECIFIC POINT FROM CONVERSATION]`. Abbas fills this in after the call before sending.

---

## Quality Checklist

Before saving all three messages:

- [ ] Initial message is 5–7 sentences
- [ ] Follow-up is 3–4 sentences
- [ ] Thank-you has `[SPECIFIC POINT FROM CONVERSATION]` placeholder
- [ ] Each message references something specific to this company or role — not generic
- [ ] Every metric mentioned is in experience.md
- [ ] None of the banned phrases appear in any message
- [ ] No fabricated credentials or outcomes
- [ ] No mention of attachments
- [ ] Three subject line options included with initial message
- [ ] All files saved to outputs/outreach/[job-id]/
- [ ] Records appended to data/outreach.csv (approved = false)
- [ ] ApplicationPrepared event logged

---

## Failure Modes

| Condition | Action |
|-----------|--------|
| Cannot find specific hook for the company | Research company's recent news or product. If still nothing specific, use the role outcome as the hook. |
| Job description is too vague to write specific copy | Use the company's stated mission and the role title. Note "low specificity" in outreach record. |
| No relevant metric matches the role | Use a scope/scale metric (team size, program scale, budget) instead. Do not invent a metric. |

---

## Event Format (logs/events.md)

```
## ApplicationPrepared
- job_id: [job-id]
- resume_id: [resume-id]
- outreach_initial: outputs/outreach/[job-id]/initial.md
- outreach_followup: outputs/outreach/[job-id]/follow-up.md
- outreach_thankyou: outputs/outreach/[job-id]/thank-you.md
- drafted_at: [date]
- awaiting_human_approval: true
```

---

## Example: Initial Message

**Subject:** Director of Technology — AT&T | Cloud + AI Delivery

---

AT&T's transformation from a legacy infrastructure operator to an AI-augmented platform company is exactly the kind of inflection point I've navigated at scale — most recently at Verizon, where I directed the cloud migration programs that generated more than $20 million in projected five-year savings.

What's different about my background: I've spent the past five years designing and deploying AI systems, not just managing vendors who build them. At V3 Information Management, I built the AI architecture that cut invoice processing time by 85% and reduced billing errors by 92%.

The combination of enterprise delivery at telecom scale and hands-on AI system design is not common. I'd like to understand how this role is positioned to solve AT&T's next infrastructure challenge.

Are you available for a 20-minute call this week or next?

Abbas Abdullah
linkedin.com/in/abbasabdullah | 650-505-2074
