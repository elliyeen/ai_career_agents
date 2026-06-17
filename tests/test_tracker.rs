/// Tests for application tracker state machine.
/// Validates that jobs move through statuses correctly
/// and that approval gates are enforced.
use career_os::db::Db;
use career_os::models::{Application, ApplicationStatus, Job};

fn temp_db() -> Db {
    // Use an in-memory database for each test
    Db::open(":memory:").expect("Failed to open in-memory database")
}

fn sample_job() -> Job {
    Job::new(
        "Senior TPM",
        "TestCo",
        "https://testco.com/jobs/tpm-1",
        "linkedin",
        "Lead cross-functional programs.",
        "Remote, US",
        true,
    )
}

// ─────────────────────────────────────────────────────
// Deduplication
// ─────────────────────────────────────────────────────

#[test]
fn duplicate_url_is_detected() {
    let db = temp_db();
    let job = sample_job();
    db.insert_job(&job).unwrap();

    let exists = db.url_exists(&job.url).unwrap();
    assert!(exists, "URL should be detected as duplicate after first insert");
}

#[test]
fn unique_url_is_not_flagged_as_duplicate() {
    let db = temp_db();
    let exists = db.url_exists("https://brand-new-url.com/jobs/1").unwrap();
    assert!(!exists, "New URL should not be flagged as duplicate");
}

// ─────────────────────────────────────────────────────
// Job qualification flow
// ─────────────────────────────────────────────────────

#[test]
fn job_starts_unqualified() {
    let db = temp_db();
    let job = sample_job();
    db.insert_job(&job).unwrap();

    let unqualified = db.unqualified_jobs().unwrap();
    assert_eq!(unqualified.len(), 1);
    assert!(unqualified[0].qualified.is_none());
}

#[test]
fn qualify_job_moves_it_out_of_unqualified_queue() {
    let db = temp_db();
    let job = sample_job();
    db.insert_job(&job).unwrap();

    db.update_job_qualification(&job.id, 82, true, None).unwrap();

    let unqualified = db.unqualified_jobs().unwrap();
    assert!(unqualified.is_empty(), "Qualified job should leave the unqualified queue");

    let qualified = db.qualified_jobs().unwrap();
    assert_eq!(qualified.len(), 1);
    assert_eq!(qualified[0].score, Some(82));
}

#[test]
fn disqualified_job_is_not_in_qualified_list() {
    let db = temp_db();
    let job = sample_job();
    db.insert_job(&job).unwrap();

    db.update_job_qualification(&job.id, 45, false, Some("SCORE_BELOW_THRESHOLD")).unwrap();

    let qualified = db.qualified_jobs().unwrap();
    assert!(qualified.is_empty(), "Disqualified job must not appear in qualified list");
}

// ─────────────────────────────────────────────────────
// Application status lifecycle
// ─────────────────────────────────────────────────────

#[test]
fn application_lifecycle_found_to_interview() {
    let db = temp_db();
    let job = sample_job();
    db.insert_job(&job).unwrap();

    // Create application in draft
    let app = Application::new(&job.id);
    db.insert_application(&app).unwrap();

    // Verify draft
    let drafts = db.applications_by_status(&ApplicationStatus::Draft).unwrap();
    assert_eq!(drafts.len(), 1);

    // Advance: draft → approved
    db.update_application_status(&app.id, &ApplicationStatus::Approved).unwrap();
    assert!(db.applications_by_status(&ApplicationStatus::Draft).unwrap().is_empty());

    // Advance: approved → submitted
    db.update_application_status(&app.id, &ApplicationStatus::Submitted).unwrap();

    // Advance: submitted → interview
    db.update_application_status(&app.id, &ApplicationStatus::Interview).unwrap();
    let interviews = db.applications_by_status(&ApplicationStatus::Interview).unwrap();
    assert_eq!(interviews.len(), 1);
}

// ─────────────────────────────────────────────────────
// Approval queue
// ─────────────────────────────────────────────────────

#[test]
fn approval_queue_starts_empty() {
    let db = temp_db();
    let pending = db.pending_approvals().unwrap();
    assert!(pending.is_empty());
}

#[test]
fn enqueued_item_appears_in_pending_list() {
    let db = temp_db();
    let queue_id = career_os::approval::enqueue(
        &db,
        "resume",
        "resume-id-001",
        "Senior TPM @ Google — v1",
        "outputs/resumes/resume-id-001-v1.txt",
    ).unwrap();

    let pending = db.pending_approvals().unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].id, queue_id);
    assert_eq!(pending[0].entity_type, "resume");
}

#[test]
fn approved_item_leaves_pending_queue() {
    let db = temp_db();
    let queue_id = career_os::approval::enqueue(
        &db,
        "outreach",
        "outreach-id-001",
        "Initial message to recruiter — TestCo",
        "outputs/outreach/outreach-id-001.txt",
    ).unwrap();

    career_os::approval::approve(&db, &queue_id).unwrap();

    let pending = db.pending_approvals().unwrap();
    assert!(pending.is_empty(), "Approved item should leave the pending queue");
}

#[test]
fn rejected_item_also_leaves_pending_queue() {
    let db = temp_db();
    let queue_id = career_os::approval::enqueue(
        &db,
        "resume",
        "resume-id-002",
        "Senior TPM @ Startup — v1",
        "outputs/resumes/resume-id-002-v1.txt",
    ).unwrap();

    career_os::approval::reject(&db, &queue_id).unwrap();

    let pending = db.pending_approvals().unwrap();
    assert!(pending.is_empty(), "Rejected item should leave the pending queue");
}

// ─────────────────────────────────────────────────────
// Metrics
// ─────────────────────────────────────────────────────

#[test]
fn metrics_reflect_pipeline_state() {
    let db = temp_db();
    let job = sample_job();
    db.insert_job(&job).unwrap();
    db.update_job_qualification(&job.id, 85, true, None).unwrap();

    let app = Application::new(&job.id);
    db.insert_application(&app).unwrap();
    db.update_application_status(&app.id, &ApplicationStatus::Submitted).unwrap();

    let m = db.current_metrics().unwrap();
    assert_eq!(m.jobs_discovered, 1);
    assert_eq!(m.jobs_qualified, 1);
    assert_eq!(m.applications, 1);
    assert_eq!(m.offers, 0);
}
