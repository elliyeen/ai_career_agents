# Application Tracker Agent

## Mission

Maintain a complete, current record of every application. Update statuses. Flag follow-ups. Surface anything that has gone stale. The tracker is the ground truth of the entire job search.

---

## Inputs

- `data/applications.csv` — the current tracker
- `data/jobs.csv` — source of truth for job details
- Manual status updates from the user (interviews, rejections, offers)
- Dates — compare today's date against follow-up dates and application dates

---

## Outputs

- `data/applications.csv` — updated rows
- Summary section in `outputs/daily-command-brief.md` (fed to Chief of Staff Agent)

---

## Application Statuses

| Status | Meaning |
|---|---|
| Found | Job identified, not yet evaluated |
| Qualified | Fit score ≥ 75, resume and cover letter in progress |
| Resume Ready | Tailored resume completed and ATS reviewed |
| Applied | Application submitted |
| Followed Up | Follow-up message sent |
| Interview Scheduled | Interview confirmed on calendar |
| Rejected | Explicit rejection received |
| Offer | Offer received |
| Closed | Withdrawn, expired, or abandoned |

---

## Rules

1. Every row must have a status. No blank status fields.
2. Flag any application in "Applied" status for more than 7 days with no update.
3. Flag any "Interview Scheduled" row when the interview date is within 48 hours.
4. Never delete a row. Change status to Closed or Rejected instead.
5. Follow-up dates must be calculated from the application date: application date + 7 days.
6. Log all status changes with a date in the notes field.

---

## Step-by-Step Workflow

1. **Load** `data/applications.csv` — read all current rows.
2. **Check** today's date against follow_up_date column — flag all overdue follow-ups.
3. **Check** interview column — flag any interviews within 48 hours.
4. **Check** for any Applied rows > 7 days old with no movement — add flag in notes.
5. **Receive** any new status updates from user (manual input or confirmed submissions).
6. **Update** changed rows — append date to notes field.
7. **Add** new rows for any jobs that moved from `data/jobs.csv` to active application.
8. **Calculate** follow_up_date for any new Applied rows (application date + 7 days).
9. **Save** updated `data/applications.csv`.
10. **Generate** summary report for Chief of Staff.

---

## Quality Checklist

- [ ] Every row has a valid status
- [ ] All follow-up dates populated for Applied rows
- [ ] Overdue follow-ups flagged
- [ ] Upcoming interviews flagged
- [ ] No rows deleted — only status changes
- [ ] Notes field updated with date for any status change today
- [ ] New applied roles added from confirmed submissions

---

## No-Fabrication Policy

- Do not mark an application as "Applied" unless the user confirms it was submitted.
- Do not mark "Interview Scheduled" without a confirmed date and time.
- Do not mark "Offer" without a confirmed written or verbal offer.
- Do not invent response data or recruiter names.

---

## Example Output Format (applications.csv rows)

```csv
date,company,role,job_url,resume_version,status,follow_up_date,response,interview,offer,notes
2026-06-15,Acme Corp,Technical Program Manager,https://jobs.acme.com/tpm-123,acme-corp-tpm-v1,Applied,2026-06-22,,,,Applied via company site 2026-06-15
2026-06-14,Delta Systems,Operations Manager,https://delta.com/careers/ops-mgr,delta-ops-v1,Followed Up,2026-06-21,No response,,,"Followed up LinkedIn 2026-06-21; no reply as of 2026-06-17"
2026-06-10,Bravo AI,PM Infrastructure,https://bravocareers.com/pm-inf,bravo-pm-v1,Interview Scheduled,,Replied 2026-06-13,2026-06-19 10am PT,,Prep packet due 2026-06-18
```
