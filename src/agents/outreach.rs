/// Outreach agent — generates recruiter messages, follow-ups, and thank-you notes.
///
/// Inputs:
///   - Application ID (from DB)
///   - Message kind: initial | follow_up | thank_you
///
/// Bio source: docs/memory/career-profile.md (the "One-paragraph bio" field)
/// Rule: never send automatically. All messages queue for human approval.
///
/// Output: outputs/outreach/{app_id_short}_{kind}.md
use crate::approval;
use crate::db::Db;
use crate::models::{Event, OutreachMessage};
use anyhow::{bail, Context, Result};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

const PROFILE_PATH: &str = "docs/memory/career-profile.md";
const OUTPUT_DIR: &str = "outputs/outreach";

pub fn run(db: &Db, app_id: &str, kind: &str) -> Result<PathBuf> {
    let valid_kinds = ["initial", "follow_up", "thank_you"];
    if !valid_kinds.contains(&kind) {
        bail!("kind must be one of: initial, follow_up, thank_you. Got '{}'", kind);
    }

    let app = db.get_application(app_id)
        .with_context(|| format!("Application '{}' not found", app_id))?;
    let job = db.get_job(&app.job_id)
        .with_context(|| format!("Job '{}' not found", app.job_id))?;

    let bio = load_bio().unwrap_or_else(|_| "[UPDATE docs/memory/career-profile.md with your bio]".to_string());

    let content = match kind {
        "initial" => generate_initial(&job, &bio),
        "follow_up" => generate_follow_up(&job),
        "thank_you" => generate_thank_you(&job),
        _ => unreachable!(),
    };

    fs::create_dir_all(OUTPUT_DIR)?;
    let short_id = &app_id[..8.min(app_id.len())];
    let filename = format!("{}_{}.md", short_id, kind);
    let path = PathBuf::from(OUTPUT_DIR).join(&filename);

    fs::write(&path, &content)
        .with_context(|| format!("Cannot write outreach to {}", path.display()))?;

    // Record in DB
    let msg = OutreachMessage {
        id: Uuid::new_v4().to_string(),
        application_id: app_id.to_string(),
        message_type: kind.to_string(),
        content: content.clone(),
        drafted_at: Utc::now(),
        approved: false,
        approved_at: None,
        sent_at: None,
        artifact_path: path.to_string_lossy().to_string(),
    };
    db.insert_outreach_message(&msg)?;

    // Queue for approval
    approval::enqueue(
        db,
        "outreach",
        &msg.id,
        &format!("{} message for {} @ {}", kind, job.title, job.company),
        &path.to_string_lossy(),
    )?;

    // Emit event
    db.emit_event(&Event::new(
        "ApplicationPrepared",
        serde_json::json!({
            "outreach_id": msg.id,
            "application_id": app_id,
            "kind": kind,
            "company": job.company,
        }),
        "outreach",
    ))?;

    Ok(path)
}

// ── Message generators ────────────────────────────────────────────────────────

fn generate_initial(job: &crate::models::Job, bio: &str) -> String {
    format!(
        "<!-- OUTREACH: initial message — CareerOS draft -->
<!-- Target role: {} @ {} -->
<!-- ACTION REQUIRED: Personalize before sending. Fill in [RECRUITER NAME]. -->
<!-- NEVER send automatically. Approve first: career-os approve <id> -->

---

Subject: {title} opportunity — {company}

Hi [RECRUITER NAME],

{bio}

I came across the {title} role at {company} and it maps closely to the work I've been doing. {location_note}

I'd welcome a brief conversation to learn more about the team and the program. Happy to share more about my background.

Best,
[YOUR NAME]

---
<!-- Review checklist before approving:
  [ ] Recruiter name filled in
  [ ] Bio paragraph is accurate and current
  [ ] Subject line is specific enough
  [ ] Location/remote note is accurate
  [ ] Tone matches your voice
-->
",
        job.title,
        job.company,
        title = job.title,
        company = job.company,
        bio = bio,
        location_note = if job.remote {
            "The remote-first setup is a strong fit for how I work best.".to_string()
        } else {
            format!("The {} location works well for me.", job.location)
        },
    )
}

fn generate_follow_up(job: &crate::models::Job) -> String {
    format!(
        "<!-- OUTREACH: follow-up — CareerOS draft -->
<!-- Target role: {} @ {} -->
<!-- ACTION REQUIRED: Fill in [RECRUITER NAME] and adjust timing note. -->

---

Subject: Re: {} — following up

Hi [RECRUITER NAME],

Following up on my application for the {} role at {}. I remain very interested and wanted to check in if there is any update on timing or next steps.

Happy to provide any additional information that would be helpful.

Best,
[YOUR NAME]

---
<!-- Recommended send window: 5–7 business days after initial application -->
",
        job.title,
        job.company,
        job.title,
        job.title,
        job.company,
    )
}

fn generate_thank_you(job: &crate::models::Job) -> String {
    format!(
        "<!-- OUTREACH: thank-you — CareerOS draft -->
<!-- Target role: {} @ {} -->
<!-- ACTION REQUIRED: Fill in [INTERVIEWER NAME] and [TOPIC DISCUSSED]. -->
<!-- Send within 24 hours of the interview. -->

---

Subject: Thank you — {} conversation

Hi [INTERVIEWER NAME],

Thank you for taking the time to speak with me about the {} role at {}. I appreciated learning more about [TOPIC DISCUSSED — e.g., the infrastructure program, the team structure, the roadmap challenges].

The conversation reinforced my interest in the role. [Add one specific thing that resonated with you.]

Looking forward to the next steps.

Best,
[YOUR NAME]

---
<!-- Review checklist:
  [ ] Interviewer name filled in
  [ ] Specific topic from the conversation included
  [ ] Sent within 24 hours
  [ ] Tone is warm but professional
-->
",
        job.title,
        job.company,
        job.title,
        job.title,
        job.company,
    )
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Extract the one-paragraph bio from career-profile.md.
/// Looks for a blockquote ("> ") after the heading "One-paragraph bio".
fn load_bio() -> Result<String> {
    let content = fs::read_to_string(PROFILE_PATH)
        .with_context(|| format!("Cannot read {}", PROFILE_PATH))?;

    let bio_section = content
        .find("One-paragraph bio")
        .map(|pos| &content[pos..])
        .unwrap_or(&content);

    let bio: String = bio_section
        .lines()
        .skip(1)
        .filter(|l| l.trim_start().starts_with('>'))
        .map(|l| l.trim_start().trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ");

    if bio.is_empty() || bio.contains('[') {
        // Placeholder not filled in
        Ok("[UPDATE docs/memory/career-profile.md — add your one-paragraph bio in the blockquote under 'One-paragraph bio']".to_string())
    } else {
        Ok(bio)
    }
}
