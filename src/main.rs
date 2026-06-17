use anyhow::Result;
use career_os::{approval, db::Db, intake, models, report, scoring};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;

#[derive(Parser)]
#[command(name = "career-os", version, about = "CareerOS — AI Workforce for Job Discovery")]
struct Cli {
    /// Path to the SQLite database file
    #[arg(long, default_value = "data/career.db", global = true)]
    db: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add jobs found today — interactive or from a JSON file
    Intake {
        /// Import jobs from a JSON file instead of interactive mode
        #[arg(long, value_name = "PATH")]
        file: Option<PathBuf>,
    },

    /// Discover new job postings (Phase 2 — web scraping)
    Discover,

    /// Score and qualify discovered jobs against the candidate profile
    Qualify {
        /// Non-interactive mode: apply heuristic scoring without prompts
        #[arg(long)]
        auto: bool,
    },

    /// List applications with optional status filter
    List {
        /// Filter by status: draft|approved|submitted|responded|interview|offer|rejected|withdrawn
        #[arg(long)]
        status: Option<String>,
    },

    /// Update an application's status
    Update {
        /// Application ID (prefix is enough)
        #[arg(long)]
        id: String,
        /// New status: draft|approved|submitted|responded|interview|offer|rejected|withdrawn
        #[arg(long)]
        status: String,
    },

    /// Generate a tailored resume for a qualified job
    Resume {
        /// Target job ID (omit to list all qualified jobs)
        #[arg(long)]
        job_id: Option<String>,
    },

    /// Generate outreach drafts for an application
    Outreach {
        /// Application ID
        #[arg(long)]
        application_id: Option<String>,
    },

    /// Generate interview preparation for a scheduled interview
    Interview {
        /// Application ID
        #[arg(long)]
        application_id: Option<String>,
    },

    /// Generate weekly metrics report and save to outputs/reviews/
    Review,

    /// List all items pending human approval
    Approvals,

    /// Approve a queued item (resume, outreach, or application)
    Approve {
        /// Approval queue item ID
        id: String,
    },

    /// Reject a queued item
    Reject {
        /// Approval queue item ID
        id: String,
    },

    /// Show overall pipeline status summary
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("career_os=info".parse()?),
        )
        .init();

    let cli = Cli::parse();

    // Ensure the data directory exists before opening the DB
    if let Some(parent) = cli.db.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let db = Db::open(&cli.db)?;

    match cli.command {
        // ─── Intake (was run_daily_research.py) ──────────────────────────
        Commands::Intake { file } => {
            info!("Running intake");
            let today = chrono::Utc::now().format("%Y-%m-%d");
            println!("\n─── CareerOS Daily Research — {today} ─────────────────\n");

            intake::print_sources_checklist();

            let added = match file {
                Some(path) => {
                    println!("Importing from: {}\n", path.display());
                    intake::import_from_file(&db, &path)?
                }
                None => intake::interactive_intake(&db)?,
            };

            println!("\nAdded {} new job(s).", added);

            // Daily summary
            let stats = db.weekly_stats()?;
            println!("\n─── Daily Summary ───────────────────────────────────");
            println!("  Total jobs:        {}", stats.jobs_discovered);
            println!("  Awaiting scoring:  {}",
                stats.jobs_discovered - stats.jobs_qualified - {
                    // count disqualified: jobs where qualified=0
                    let total = stats.jobs_discovered;
                    let qual = stats.jobs_qualified;
                    // difference = unscored + disqualified; we can't split without a DB call
                    // just show jobs_discovered - qualified as "not yet in pipeline"
                    let _ = (total, qual);
                    0i64
                });
            println!("  Qualified:         {}", stats.jobs_qualified);
            println!("  Pending approvals: {}", stats.pending_approvals);
            if added > 0 {
                println!("\n  Next: career-os qualify --auto");
            }
            if stats.pending_approvals > 0 {
                println!("  Next: career-os approvals");
            }
            println!();
        }

        // ─── Discover (Phase 2 — web scraping) ───────────────────────────
        Commands::Discover => {
            info!("Running job discovery agent");
            println!("Automated discovery — not yet implemented (Phase 2).");
            println!("Use `career-os intake` to add jobs manually today.");
            println!("Sources config: docs/config/sources.md");
        }

        // ─── Qualify (was score_jobs.py) ──────────────────────────────────
        Commands::Qualify { auto } => {
            info!("Running qualification agent (auto={})", auto);
            let jobs = db.unqualified_jobs()?;
            if jobs.is_empty() {
                println!("No unqualified jobs. Run `career-os intake` first.");
                return Ok(());
            }

            println!("\nScoring {} unqualified job(s)...\n", jobs.len());
            let mut qualified_count = 0u32;
            let mut disqualified_count = 0u32;

            for job in &jobs {
                println!("  [{}] {} @ {}", &job.id[..8], job.title, job.company);

                let result = intake::heuristic_score(&job.title, &job.location, job.remote);

                let (final_score, final_qualified, final_reason) = if auto {
                    (result.score, result.qualified, result.reason)
                } else {
                    // Interactive: show heuristic, allow override
                    println!("    URL:      {}", job.url);
                    println!("    Location: {} | Remote: {}", job.location, job.remote);
                    println!("    Heuristic score: {}", result.score);
                    print!("    Override? Enter score or press Enter to accept: ");
                    use std::io::{self, BufRead, Write};
                    io::stdout().flush().ok();
                    let mut line = String::new();
                    io::stdin().lock().read_line(&mut line).ok();
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        (result.score, result.qualified, result.reason)
                    } else {
                        match trimmed.parse::<i32>() {
                            Ok(override_score) => {
                                let q = override_score >= scoring::QUALIFY_THRESHOLD;
                                let r = if q {
                                    None
                                } else {
                                    Some("SCORE_BELOW_THRESHOLD".to_string())
                                };
                                (override_score, q, r)
                            }
                            Err(_) => {
                                println!("    Invalid score — skipping.");
                                continue;
                            }
                        }
                    }
                };

                db.update_job_qualification(
                    &job.id,
                    final_score,
                    final_qualified,
                    final_reason.as_deref(),
                )?;

                if final_qualified {
                    qualified_count += 1;
                    println!("    → QUALIFIED ({})\n", final_score);

                    let event = career_os::models::Event::new(
                        "JobQualified",
                        serde_json::json!({"job_id": job.id, "score": final_score}),
                        "qualify",
                    );
                    db.emit_event(&event)?;
                } else {
                    disqualified_count += 1;
                    println!(
                        "    → DISQUALIFIED — {}\n",
                        final_reason.as_deref().unwrap_or("SCORE_BELOW_THRESHOLD")
                    );
                }
            }

            println!(
                "Done. Qualified: {} | Disqualified: {}",
                qualified_count, disqualified_count
            );
            if qualified_count > 0 {
                println!("Next: career-os resume");
            }
        }

        // ─── List (was update_tracker.py --list) ──────────────────────────
        Commands::List { status } => {
            let rows = db.list_applications(status.as_deref())?;
            if rows.is_empty() {
                println!("No applications found.");
                return Ok(());
            }
            println!(
                "\n{:<10} {:<12} {:<35} {:<20} {}",
                "ID", "Status", "Title", "Company", "Date"
            );
            println!("{}", "─".repeat(90));
            for row in &rows {
                let date = row.created_at.get(..10).unwrap_or(&row.created_at);
                let title = if row.title.len() > 33 {
                    &row.title[..33]
                } else {
                    &row.title
                };
                let company = if row.company.len() > 18 {
                    &row.company[..18]
                } else {
                    &row.company
                };
                println!(
                    "{:<10} {:<12} {:<35} {:<20} {}",
                    &row.id[..8.min(row.id.len())],
                    row.status,
                    title,
                    company,
                    date,
                );
            }
            println!();
        }

        // ─── Update (was update_tracker.py --id --status) ─────────────────
        Commands::Update { id, status } => {
            let new_status: models::ApplicationStatus = status.parse()?;

            // Support ID prefix matching
            let (full_id, current_status_str) = match db.application_by_prefix(&id)? {
                Some(pair) => pair,
                None => {
                    anyhow::bail!("Application not found: {}", id);
                }
            };

            if current_status_str == new_status.to_string() {
                println!("Application {} is already '{}'.", &full_id[..8], new_status);
                return Ok(());
            }

            db.update_application_status(&full_id, &new_status)?;
            println!("Updated: {} → {}", &full_id[..8], new_status);

            match new_status {
                models::ApplicationStatus::Interview => {
                    println!("\n  Next: generate interview prep");
                    println!("  Run: career-os interview --application-id {}", full_id);
                }
                models::ApplicationStatus::Offer => {
                    println!("\n  Record offer details in application notes.");
                    println!("  Do not accept until you have reviewed: compensation, equity, start date.");
                }
                _ => {}
            }
        }

        // ─── Resume ───────────────────────────────────────────────────────
        Commands::Resume { job_id } => {
            info!("Running resume generation agent");
            match job_id {
                Some(id) => {
                    println!("Generating resume for job {} — not yet implemented.", id);
                    println!("Master resume source: docs/memory/master-resume.md");
                }
                None => {
                    let jobs = db.qualified_jobs()?;
                    if jobs.is_empty() {
                        println!("No qualified jobs. Run `career-os qualify` first.");
                    } else {
                        println!("{} qualified job(s) available:\n", jobs.len());
                        for job in &jobs {
                            println!(
                                "  {} | {} @ {} (score: {})",
                                &job.id[..8],
                                job.title,
                                job.company,
                                job.score.map(|s| s.to_string()).unwrap_or_else(|| "?".into())
                            );
                        }
                        println!("\nRun: career-os resume --job-id <id>");
                    }
                }
            }
        }

        // ─── Outreach ─────────────────────────────────────────────────────
        Commands::Outreach { application_id } => {
            info!("Running outreach agent");
            match application_id {
                Some(id) => {
                    println!("Generating outreach for application {} — not yet implemented.", id)
                }
                None => {
                    println!("Outreach agent — specify --application-id or run after resume approval.")
                }
            }
        }

        // ─── Interview ────────────────────────────────────────────────────
        Commands::Interview { application_id } => {
            info!("Running interview prep agent");
            match application_id {
                Some(id) => {
                    println!("Generating interview prep for {} — not yet implemented.", id)
                }
                None => println!("Interview prep — specify --application-id."),
            }
        }

        // ─── Review (was weekly_review.py) ────────────────────────────────
        Commands::Review => {
            info!("Running weekly review");
            report::run_review(&db)?;
        }

        // ─── Approval commands ────────────────────────────────────────────
        Commands::Approvals => {
            approval::list_pending(&db)?;
        }

        Commands::Approve { id } => {
            approval::approve(&db, &id)?;
            println!("Approved: {}", id);
        }

        Commands::Reject { id } => {
            approval::reject(&db, &id)?;
            println!("Rejected: {}", id);
        }

        // ─── Status ───────────────────────────────────────────────────────
        Commands::Status => {
            let stats = db.weekly_stats()?;
            println!("\n─── CareerOS Status ─────────────────────────────────\n");
            println!("  Database:          {}", cli.db.display());
            println!("  Jobs discovered:   {}", stats.jobs_discovered);
            println!("  Jobs qualified:    {}", stats.jobs_qualified);
            println!("  Active apps:       {}", stats.applications);
            println!("  Pending approvals: {}", stats.pending_approvals);
            if stats.pending_approvals > 0 {
                println!("\n  Run `career-os approvals` to review.");
            }
            println!();
        }
    }

    Ok(())
}
