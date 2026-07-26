pub mod discover;
pub mod interview;
pub mod outreach;
pub mod qualify;
pub mod resume;
pub mod review;

use crate::db::Db;
use anyhow::Result;
use std::time::Instant;
use tracing::info;

/// Run `f` and record the execution in the `logs` table regardless of outcome.
pub fn run_logged<F, T>(db: &Db, agent_name: &str, f: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    let started = chrono::Utc::now();
    let timer = Instant::now();
    info!(agent = agent_name, "starting");

    match f() {
        Ok(val) => {
            let ms = timer.elapsed().as_millis() as i64;
            let _ = db.log_run(agent_name, &started.to_rfc3339(), ms, "success", None, None);
            info!(agent = agent_name, duration_ms = ms, "done");
            Ok(val)
        }
        Err(e) => {
            let ms = timer.elapsed().as_millis() as i64;
            let msg = e.to_string();
            let _ = db.log_run(agent_name, &started.to_rfc3339(), ms, "failure", Some(&msg), None);
            Err(e)
        }
    }
}
