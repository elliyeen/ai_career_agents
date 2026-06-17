use anyhow::Result;
use career_os::{approval, db::Db, models, scoring};
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
    /// Discover new job postings
    Discover,
    /// Score and qualify discovered jobs against the candidate profile
    Qualify,
    /// Generate a tailored resume for a qualified job
    Resume {
        /// Target job ID (omit to list all qualified jobs)
        #[arg(long)]
        job_id: Option<String>,
    },
    /// Generate outreach drafts for an application
    Outreach {
        /// Application ID (omit to list applications ready for outreach)
        #[arg(long)]
        application_id: Option<String>,
    },
    /// Update an application's status
    Update {
        /// Application ID
        #[arg(long)]
        id: String,
        /// New status: draft|approved|submitted|responded|interview|offer|rejected|withdrawn
        #[arg(long)]
        status: String,
    },
    /// Generate interview preparation for a scheduled interview
    Interview {
        /// Application ID
        #[arg(long)]
        application_id: Option<String>,
    },
    /// Show weekly metrics and conversion rates
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
    info!("Database ready at {}", cli.db.display());

    match cli.command {
        Commands::Discover => {
            info!("Running job discovery agent");
            println!("Discovery agent — not yet implemented.");
            println!("Sources config: docs/config/sources.md");
        }

        Commands::Qualify => {
            info!("Running qualification agent");
            let jobs = db.unqualified_jobs()?;
            if jobs.is_empty() {
                println!("No unqualified jobs found. Run `career-os discover` first.");
                return Ok(());
            }
            println!("Qualifying {} job(s) — automatic scoring not yet implemented.", jobs.len());
            println!("Scoring rubric: docs/config/scoring.md");
            println!("Qualify threshold: {}+", scoring::QUALIFY_THRESHOLD);
            println!("Priority threshold: {}+", scoring::PRIORITY_THRESHOLD);
        }

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
                            println!("  {} | {} @ {} (score: {})",
                                job.id, job.title, job.company,
                                job.score.map(|s| s.to_string()).unwrap_or_else(|| "?".into()));
                        }
                        println!("\nRun: career-os resume --job-id <id>");
                    }
                }
            }
        }

        Commands::Outreach { application_id } => {
            info!("Running outreach agent");
            match application_id {
                Some(id) => println!("Generating outreach for application {} — not yet implemented.", id),
                None => println!("Outreach agent — specify --application-id or run after resume approval."),
            }
        }

        Commands::Update { id, status } => {
            let new_status: models::ApplicationStatus = status.parse()?;
            db.update_application_status(&id, &new_status)?;
            println!("Application {} → {}", id, new_status);
        }

        Commands::Interview { application_id } => {
            info!("Running interview prep agent");
            match application_id {
                Some(id) => println!("Generating interview prep for {} — not yet implemented.", id),
                None => println!("Interview prep — specify --application-id."),
            }
        }

        Commands::Review => {
            info!("Running weekly review");
            let m = db.current_metrics()?;
            println!("\n─── CareerOS Weekly Review ──────────────────────────\n");
            println!("  Jobs discovered:   {}", m.jobs_discovered);
            println!("  Jobs qualified:    {}", m.jobs_qualified);
            println!("  Applications:      {}", m.applications);
            println!("  Responses:         {}", m.responses);
            println!("  Interviews:        {}", m.interviews);
            println!("  Offers:            {}", m.offers);
            println!("  Rejections:        {}", m.rejections);
            println!();
            if m.applications > 0 {
                println!("  Response rate:     {:.0}%",
                    m.responses as f64 / m.applications as f64 * 100.0);
            }
            if m.responses > 0 {
                println!("  Interview rate:    {:.0}%",
                    m.interviews as f64 / m.responses as f64 * 100.0);
            }
            if m.interviews > 0 {
                println!("  Offer rate:        {:.0}%",
                    m.offers as f64 / m.interviews as f64 * 100.0);
            }
            println!();
        }

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

        Commands::Status => {
            let m = db.current_metrics()?;
            let pending = db.pending_approvals()?;
            println!("\n─── CareerOS Status ─────────────────────────────────\n");
            println!("  Database:          {}", cli.db.display());
            println!("  Jobs discovered:   {}", m.jobs_discovered);
            println!("  Jobs qualified:    {}", m.jobs_qualified);
            println!("  Active apps:       {}", m.applications);
            println!("  Pending approvals: {}", pending.len());
            if !pending.is_empty() {
                println!("\n  Run `career-os approvals` to review.");
            }
            println!();
        }
    }

    Ok(())
}
