/// Interview prep agent — generates a full preparation package for a scheduled interview.
///
/// Reads job details from DB, generates:
///   - Company research checklist
///   - STAR story prompts tailored to the job description
///   - Question bank (interviewer questions to ask)
///   - 30-60-90 day plan template
///
/// Sources:
///   - docs/memory/project-stories.md  (your STAR story bank)
///   - docs/memory/career-profile.md   (your bio and strengths)
///
/// Output: outputs/interviews/{app_id_short}_r{round}.md
use crate::db::Db;
use crate::models::Event;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

const OUTPUT_DIR: &str = "outputs/interviews";
const STORIES_PATH: &str = "docs/memory/project-stories.md";

pub fn run(db: &Db, app_id: &str, round: u32) -> Result<PathBuf> {
    let app = db.get_application(app_id)
        .with_context(|| format!("Application '{}' not found", app_id))?;
    let job = db.get_job(&app.job_id)
        .with_context(|| format!("Job '{}' not found", app.job_id))?;

    let stories = fs::read_to_string(STORIES_PATH).unwrap_or_else(|_| {
        "<!-- No project-stories.md found. Create docs/memory/project-stories.md -->".to_string()
    });

    let content = generate_prep_package(&job, &stories, round);

    fs::create_dir_all(OUTPUT_DIR)?;
    let short_id = &app_id[..8.min(app_id.len())];
    let filename = format!("{}_r{}.md", short_id, round);
    let path = PathBuf::from(OUTPUT_DIR).join(&filename);

    fs::write(&path, &content)
        .with_context(|| format!("Cannot write interview prep to {}", path.display()))?;

    // Emit event
    db.emit_event(&Event::new(
        "InterviewScheduled",
        serde_json::json!({
            "application_id": app_id,
            "job_id": app.job_id,
            "round": round,
            "company": job.company,
        }),
        "interview",
    ))?;

    Ok(path)
}

// ── Content generators ────────────────────────────────────────────────────────

fn generate_prep_package(job: &crate::models::Job, stories: &str, round: u32) -> String {
    let company_research = company_research_checklist(&job.company);
    let star_prompts = star_story_prompts(job);
    let questions = interviewer_question_bank(job);
    let plan = plan_30_60_90(job);

    format!(
        "# Interview Prep — {} @ {}
## Round {round} | Generated: {date}

---

{company_research}

---

{star_prompts}

---

{questions}

---

{plan}

---

## Stories Reference

Pulled from docs/memory/project-stories.md. Review these before the interview and
select the 3–5 most relevant to this role.

{stories_summary}
",
        job.title,
        job.company,
        round = round,
        date = chrono::Utc::now().format("%Y-%m-%d"),
        company_research = company_research,
        star_prompts = star_prompts,
        questions = questions,
        plan = plan,
        stories_summary = summarize_stories(stories),
    )
}

fn company_research_checklist(company: &str) -> String {
    format!(
        "## Company Research — {company}

Complete before the interview. Add your findings inline.

- [ ] **Mission and values** — What does the company stand for?
      > _Your notes:_

- [ ] **Recent news** — Last 3–6 months of announcements, earnings, product launches
      > _Your notes:_

- [ ] **Business model** — How does the company make money?
      > _Your notes:_

- [ ] **Engineering organization** — Size, key teams, public engineering blog posts
      > _Your notes:_

- [ ] **AI/ML initiatives** — What is the company building in this space?
      > _Your notes:_

- [ ] **Competitors** — Top 2–3 competitors and how {company} differentiates
      > _Your notes:_

- [ ] **Interviewers** — LinkedIn profiles of each person you'll speak with
      > _Your notes:_

- [ ] **Culture signals** — Glassdoor reviews, LinkedIn posts, employee interviews
      > _Your notes:_
",
        company = company
    )
}

fn star_story_prompts(job: &crate::models::Job) -> String {
    let desc_lower = job.description.to_lowercase();

    let mut prompts = vec![
        "**Tell me about a time you led a cross-functional program from inception to launch.**".to_string(),
        "**Describe a situation where you had to align engineering and business stakeholders on a difficult decision.**".to_string(),
        "**Give an example of a program that was off-track. What did you do?**".to_string(),
        "**Tell me about the largest program you've managed. What was the scope?**".to_string(),
    ];

    // Add role-specific prompts based on job description keywords
    if desc_lower.contains("ai") || desc_lower.contains("ml") || desc_lower.contains("llm") {
        prompts.push("**Describe your experience managing AI or ML programs. What made them different from standard software programs?**".to_string());
    }
    if desc_lower.contains("infrastructure") || desc_lower.contains("platform") {
        prompts.push("**Tell me about a time you drove adoption of a platform or infrastructure product.**".to_string());
    }
    if desc_lower.contains("executive") || desc_lower.contains("vp") || desc_lower.contains("c-suite") {
        prompts.push("**How have you communicated program status to executive-level stakeholders? Give a specific example.**".to_string());
    }
    if desc_lower.contains("risk") || desc_lower.contains("mitigation") {
        prompts.push("**Tell me about a significant program risk you identified and mitigated. What was your process?**".to_string());
    }
    if desc_lower.contains("process") || desc_lower.contains("improvement") || desc_lower.contains("efficiency") {
        prompts.push("**Give an example of a process you redesigned that improved team efficiency or output quality.**".to_string());
    }

    let prompt_list = prompts
        .iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {}\n   - Situation:\n   - Task:\n   - Action:\n   - Result:", i + 1, p))
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        "## STAR Story Prompts — {} @ {}

For each prompt, prepare a 2–3 minute story using the STAR framework.
Pull from docs/memory/project-stories.md. Reference real metrics only.

{}
",
        job.title,
        job.company,
        prompt_list
    )
}

fn interviewer_question_bank(job: &crate::models::Job) -> String {
    let desc_lower = job.description.to_lowercase();

    let mut questions = vec![
        "What does success look like in this role in the first 90 days?",
        "What are the biggest challenges facing the team or program right now?",
        "How does this role interact with the engineering leadership team?",
        "What does the program planning cycle look like here?",
        "How is the TPM function staffed relative to engineering?",
        "What does good look like for this role at the 1-year mark?",
    ];

    if desc_lower.contains("ai") || desc_lower.contains("ml") {
        questions.push("How mature is the AI/ML program infrastructure? What's the biggest gap?");
        questions.push("How do you measure success for AI initiatives differently from traditional software programs?");
    }
    if desc_lower.contains("infrastructure") || desc_lower.contains("platform") {
        questions.push("What is the biggest platform adoption challenge you're facing?");
    }

    let question_list = questions
        .iter()
        .enumerate()
        .map(|(i, q)| format!("{}. {}", i + 1, q))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "## Questions to Ask the Interviewer

Prepare 3–5 from this list. Prioritize based on who you're speaking with.

{}
",
        question_list
    )
}

fn plan_30_60_90(job: &crate::models::Job) -> String {
    format!(
        "## 30-60-90 Day Plan — {} @ {}

Use this as a conversation anchor in the final interview round.
Edit with specifics based on your company research.

### First 30 Days — Learn
- [ ] Shadow each key stakeholder: engineering leads, product, legal, finance
- [ ] Read all existing program docs, PRDs, roadmaps, postmortems
- [ ] Map the current program portfolio: what is running, what is at risk, what is stalled
- [ ] Identify the team's biggest communication pain points
- [ ] Establish 1:1 cadence with direct collaborators

### 31–60 Days — Contribute
- [ ] Take ownership of one live program or workstream
- [ ] Deliver first program status report in the team's format
- [ ] Identify and surface one risk or gap that hasn't been formally tracked
- [ ] Propose one process improvement (lightweight, fast to ship)
- [ ] Build relationships with key partner teams

### 61–90 Days — Drive
- [ ] Own a complete program cycle end-to-end
- [ ] Deliver a retrospective or program health review
- [ ] Present to leadership with a clear recommendation
- [ ] Define what you will own for the next 6 months and align with manager
- [ ] Establish your team's operating rhythm if it doesn't exist
",
        job.title,
        job.company
    )
}

fn summarize_stories(stories: &str) -> String {
    if stories.contains("No project-stories.md") {
        return "⚠ docs/memory/project-stories.md not found. Create it with your STAR story bank.\n  See docs/memory/master-resume.md for the recommended format.".to_string();
    }

    // Count headers (##) as story count proxy
    let story_count = stories.lines().filter(|l| l.starts_with("## ")).count();
    if story_count == 0 {
        return "ℹ docs/memory/project-stories.md exists but has no ## story headers.\n  Add your STAR stories using ## Story Title as each entry.".to_string();
    }

    let titles: Vec<&str> = stories
        .lines()
        .filter(|l| l.starts_with("## "))
        .map(|l| l.trim_start_matches('#').trim())
        .take(10)
        .collect();

    format!("{} stories available:\n{}", story_count, titles.iter().map(|t| format!("- {}", t)).collect::<Vec<_>>().join("\n"))
}
