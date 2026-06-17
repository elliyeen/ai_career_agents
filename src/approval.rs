/// Human approval gate.
///
/// Nothing external happens automatically. This module enforces
/// the approval requirement before any resume, outreach, or
/// application can move forward.
///
/// Workflow:
///   1. Agent calls `enqueue(db, entity_type, entity_id, summary, path)`
///   2. Human runs `career-os review` — sees pending items
///   3. Human runs `career-os approve <id>` or `career-os reject <id>`
///   4. Downstream agent checks `is_approved()` before proceeding
use anyhow::Result;
use chrono::Utc;
use tracing::info;
use uuid::Uuid;

use crate::db::Db;
use crate::models::ApprovalQueueItem;

pub fn enqueue(
    db: &Db,
    entity_type: &str,
    entity_id: &str,
    summary: &str,
    artifact_path: &str,
) -> Result<String> {
    let item = ApprovalQueueItem {
        id: Uuid::new_v4().to_string(),
        entity_type: entity_type.to_string(),
        entity_id: entity_id.to_string(),
        summary: summary.to_string(),
        artifact_path: artifact_path.to_string(),
        queued_at: Utc::now(),
        resolved_at: None,
        decision: None,
    };
    let id = item.id.clone();
    db.enqueue_approval(&item)?;
    info!("Queued for approval: {} ({}) — {}", entity_type, entity_id, summary);
    Ok(id)
}

pub fn approve(db: &Db, queue_id: &str) -> Result<()> {
    db.resolve_approval(queue_id, "approved")?;
    info!("Approved: {}", queue_id);
    Ok(())
}

pub fn reject(db: &Db, queue_id: &str) -> Result<()> {
    db.resolve_approval(queue_id, "rejected")?;
    info!("Rejected: {}", queue_id);
    Ok(())
}

/// Print pending approval items to stdout in a human-readable format.
pub fn list_pending(db: &Db) -> Result<()> {
    let items = db.pending_approvals()?;
    if items.is_empty() {
        println!("No items pending approval.");
        return Ok(());
    }
    println!("\n─── Pending Approvals ({}) ───────────────────────────\n", items.len());
    for item in &items {
        println!("  ID:      {}", item.id);
        println!("  Type:    {}", item.entity_type);
        println!("  Summary: {}", item.summary);
        println!("  File:    {}", item.artifact_path);
        println!("  Queued:  {}", item.queued_at.format("%Y-%m-%d %H:%M UTC"));
        println!();
    }
    println!("  Approve: career-os approve <id>");
    println!("  Reject:  career-os reject <id>");
    println!("  Review:  open the file listed above before deciding\n");
    Ok(())
}
