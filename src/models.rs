use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─────────────────────────────────────────────────────
// Job
// ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub company: String,
    pub url: String,
    pub source: String,
    pub description: String,
    pub location: String,
    pub remote: bool,
    pub discovered_at: DateTime<Utc>,
    pub score: Option<i32>,
    pub qualified: Option<bool>,
    pub qualified_at: Option<DateTime<Utc>>,
    pub disqualified_reason: Option<String>,
}

impl Job {
    pub fn new(
        title: impl Into<String>,
        company: impl Into<String>,
        url: impl Into<String>,
        source: impl Into<String>,
        description: impl Into<String>,
        location: impl Into<String>,
        remote: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: title.into(),
            company: company.into(),
            url: url.into(),
            source: source.into(),
            description: description.into(),
            location: location.into(),
            remote,
            discovered_at: Utc::now(),
            score: None,
            qualified: None,
            qualified_at: None,
            disqualified_reason: None,
        }
    }
}

// ─────────────────────────────────────────────────────
// Resume
// ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    pub id: String,
    pub job_id: String,
    pub version: i32,
    pub content: String,
    pub format: String,
    pub generated_at: DateTime<Utc>,
    pub approved: bool,
    pub approved_at: Option<DateTime<Utc>>,
    pub artifact_path: String,
}

// ─────────────────────────────────────────────────────
// Application
// ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApplicationStatus {
    Draft,
    Approved,
    Submitted,
    Responded,
    Interview,
    Offer,
    Rejected,
    Withdrawn,
}

impl std::fmt::Display for ApplicationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Draft => "draft",
            Self::Approved => "approved",
            Self::Submitted => "submitted",
            Self::Responded => "responded",
            Self::Interview => "interview",
            Self::Offer => "offer",
            Self::Rejected => "rejected",
            Self::Withdrawn => "withdrawn",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for ApplicationStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(Self::Draft),
            "approved" => Ok(Self::Approved),
            "submitted" => Ok(Self::Submitted),
            "responded" => Ok(Self::Responded),
            "interview" => Ok(Self::Interview),
            "offer" => Ok(Self::Offer),
            "rejected" => Ok(Self::Rejected),
            "withdrawn" => Ok(Self::Withdrawn),
            other => Err(anyhow::anyhow!("Unknown status: {}", other)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub job_id: String,
    pub resume_id: Option<String>,
    pub status: ApplicationStatus,
    pub submitted_at: Option<DateTime<Utc>>,
    pub response_at: Option<DateTime<Utc>>,
    pub notes: String,
    pub created_at: DateTime<Utc>,
}

impl Application {
    pub fn new(job_id: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            job_id: job_id.into(),
            resume_id: None,
            status: ApplicationStatus::Draft,
            submitted_at: None,
            response_at: None,
            notes: String::new(),
            created_at: Utc::now(),
        }
    }
}

// ─────────────────────────────────────────────────────
// Outreach
// ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutreachMessage {
    pub id: String,
    pub application_id: String,
    pub message_type: String, // initial | follow_up | thank_you
    pub content: String,
    pub drafted_at: DateTime<Utc>,
    pub approved: bool,
    pub approved_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub artifact_path: String,
}

// ─────────────────────────────────────────────────────
// Event
// ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub payload: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
    pub agent: String,
}

impl Event {
    pub fn new(
        name: impl Into<String>,
        payload: serde_json::Value,
        agent: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            payload,
            occurred_at: Utc::now(),
            agent: agent.into(),
        }
    }
}

// ─────────────────────────────────────────────────────
// Metric
// ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metric {
    pub id: String,
    pub week_start: String,
    pub jobs_discovered: i32,
    pub jobs_qualified: i32,
    pub applications: i32,
    pub responses: i32,
    pub interviews: i32,
    pub offers: i32,
    pub rejections: i32,
    pub recorded_at: DateTime<Utc>,
}

// ─────────────────────────────────────────────────────
// ApprovalQueueItem
// ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalQueueItem {
    pub id: String,
    pub entity_type: String, // resume | outreach | application
    pub entity_id: String,
    pub summary: String,
    pub artifact_path: String,
    pub queued_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub decision: Option<String>, // approved | rejected
}
