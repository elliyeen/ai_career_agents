/// Job scoring engine.
///
/// Implements the rubric defined in docs/config/scoring.md.
/// All weights and thresholds live here as constants so they
/// can be changed without touching business logic.
// ─────────────────────────────────────────────────────
// Weights (must sum to 100)
// ─────────────────────────────────────────────────────
const W_ROLE: i32 = 25;
const W_EXPERIENCE: i32 = 15;
const W_SALARY: i32 = 15;
const W_INDUSTRY: i32 = 15;
const W_LOCATION: i32 = 10;
const W_GROWTH: i32 = 10;
const W_STRATEGIC: i32 = 10;

/// Minimum score to qualify a job.
pub const QUALIFY_THRESHOLD: i32 = 75;
/// Score at which we prioritize the job and generate resume immediately.
pub const PRIORITY_THRESHOLD: i32 = 90;

// ─────────────────────────────────────────────────────
// Scoring inputs
// ─────────────────────────────────────────────────────

/// All inputs needed to score a job against the candidate profile.
/// Percentages are 0.0–1.0. Boolean flags map to clear yes/no.
#[derive(Debug, Default)]
pub struct ScoringInputs {
    // Role match
    pub title_match_pct: f32,       // 0.0–1.0, 1.0 = exact title match

    // Experience match
    pub requirements_met_pct: f32,  // fraction of required skills you possess
    pub missing_required_skills: u32, // hard missing (not "nice to have")

    // Salary match
    pub salary_known: bool,
    pub salary_above_target: bool,  // posted max >= your target
    pub salary_above_floor: bool,   // posted max >= your hard floor
    pub salary_overlaps_target: bool,

    // Industry match
    pub industry_preferred: bool,
    pub industry_acceptable: bool,
    pub industry_excluded: bool,    // hard disqualify

    // Location match
    pub remote_ok: bool,
    pub hybrid_days: u32,           // days/week in office (0 = fully remote)
    pub office_city_preferred: bool,
    pub office_city_acceptable: bool,
    pub office_city_excluded: bool, // hard disqualify

    // Growth potential
    pub company_growing: bool,
    pub role_is_building: bool,     // building vs. maintaining
    pub path_to_next_level_clear: bool,

    // Strategic value
    pub adds_new_skill_or_brand: bool,
    pub solidifies_strength: bool,
    pub is_lateral: bool,
    pub is_backward: bool,

    // Bonuses
    pub is_tier1_company: bool,
    pub is_tier2_company: bool,
    pub recruiter_reached_out: bool,
    pub has_referral: bool,

    // Company on blocklist
    pub company_blocked: bool,
}

// ─────────────────────────────────────────────────────
// Scoring result
// ─────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ScoringResult {
    pub total: i32,
    pub role_score: i32,
    pub experience_score: i32,
    pub salary_score: i32,
    pub industry_score: i32,
    pub location_score: i32,
    pub growth_score: i32,
    pub strategic_score: i32,
    pub bonus: i32,
    pub disqualified: bool,
    pub disqualify_reason: Option<String>,
}

impl ScoringResult {
    pub fn is_qualified(&self) -> bool {
        !self.disqualified && self.total >= QUALIFY_THRESHOLD
    }

    pub fn is_priority(&self) -> bool {
        self.is_qualified() && self.total >= PRIORITY_THRESHOLD
    }
}

// ─────────────────────────────────────────────────────
// Score a job
// ─────────────────────────────────────────────────────

pub fn score(inputs: &ScoringInputs) -> ScoringResult {
    // Hard disqualifications before scoring
    if inputs.company_blocked {
        return disqualify("EXCLUDED_COMPANY");
    }
    if inputs.industry_excluded {
        return disqualify("EXCLUDED_INDUSTRY");
    }
    if inputs.office_city_excluded && !inputs.remote_ok {
        return disqualify("LOCATION_INCOMPATIBLE");
    }
    if inputs.salary_known && !inputs.salary_above_floor {
        return disqualify("SALARY_BELOW_FLOOR");
    }
    // Two or more hard missing required skills is a critical gap — do not proceed.
    // Documented in docs/config/scoring.md as EXPERIENCE_GAP_CRITICAL.
    if inputs.missing_required_skills >= 2 {
        return disqualify("EXPERIENCE_GAP_CRITICAL");
    }

    // Role match (0–25)
    let role_score = {
        let pct = inputs.title_match_pct;
        if pct >= 0.9 { W_ROLE }
        else if pct >= 0.7 { (W_ROLE as f32 * 0.82) as i32 }
        else if pct >= 0.5 { (W_ROLE as f32 * 0.58) as i32 }
        else if pct >= 0.25 { (W_ROLE as f32 * 0.30) as i32 }
        else { (W_ROLE as f32 * 0.10) as i32 }
    };

    // Experience match (0–15), minus 5 per hard missing skill
    let experience_score = {
        let pct = inputs.requirements_met_pct;
        let base = if pct >= 0.9 { W_EXPERIENCE }
            else if pct >= 0.7 { (W_EXPERIENCE as f32 * 0.73) as i32 }
            else if pct >= 0.5 { (W_EXPERIENCE as f32 * 0.43) as i32 }
            else { (W_EXPERIENCE as f32 * 0.13) as i32 };
        (base - 5 * inputs.missing_required_skills as i32).max(0)
    };

    // Salary match (0–15)
    let salary_score = if !inputs.salary_known {
        4  // unknown = low score, not zero
    } else if inputs.salary_above_target {
        W_SALARY
    } else if inputs.salary_overlaps_target {
        (W_SALARY as f32 * 0.73) as i32
    } else if inputs.salary_above_floor {
        (W_SALARY as f32 * 0.43) as i32
    } else {
        0
    };

    // Industry match (0–15)
    let industry_score = if inputs.industry_preferred {
        W_INDUSTRY
    } else if inputs.industry_acceptable {
        (W_INDUSTRY as f32 * 0.73) as i32
    } else {
        (W_INDUSTRY as f32 * 0.33) as i32 // neutral
    };

    // Location match (0–10)
    let location_score = if inputs.remote_ok && inputs.hybrid_days == 0 {
        W_LOCATION
    } else if inputs.remote_ok && inputs.hybrid_days <= 2 {
        (W_LOCATION as f32 * 0.80) as i32
    } else if inputs.remote_ok && inputs.hybrid_days <= 4 {
        (W_LOCATION as f32 * 0.60) as i32
    } else if inputs.office_city_preferred {
        (W_LOCATION as f32 * 0.40) as i32
    } else if inputs.office_city_acceptable {
        (W_LOCATION as f32 * 0.25) as i32
    } else {
        (W_LOCATION as f32 * 0.10) as i32
    };

    // Growth potential (0–10)
    let growth_score = if inputs.company_growing && inputs.path_to_next_level_clear && inputs.role_is_building {
        W_GROWTH
    } else if inputs.company_growing && inputs.role_is_building {
        (W_GROWTH as f32 * 0.80) as i32
    } else if inputs.company_growing || inputs.path_to_next_level_clear {
        (W_GROWTH as f32 * 0.60) as i32
    } else {
        (W_GROWTH as f32 * 0.30) as i32
    };

    // Strategic value (0–10)
    let strategic_score = if inputs.adds_new_skill_or_brand {
        W_STRATEGIC
    } else if inputs.solidifies_strength {
        (W_STRATEGIC as f32 * 0.80) as i32
    } else if inputs.is_lateral {
        (W_STRATEGIC as f32 * 0.50) as i32
    } else if inputs.is_backward {
        (W_STRATEGIC as f32 * 0.20) as i32
    } else {
        (W_STRATEGIC as f32 * 0.50) as i32
    };

    // Bonuses
    let bonus = (if inputs.is_tier1_company { 10 } else { 0 })
        + (if inputs.is_tier2_company { 5 } else { 0 })
        + (if inputs.recruiter_reached_out { 5 } else { 0 })
        + (if inputs.has_referral { 10 } else { 0 });

    let total = role_score + experience_score + salary_score + industry_score
        + location_score + growth_score + strategic_score + bonus;

    ScoringResult {
        total,
        role_score,
        experience_score,
        salary_score,
        industry_score,
        location_score,
        growth_score,
        strategic_score,
        bonus,
        disqualified: false,
        disqualify_reason: None,
    }
}

fn disqualify(reason: &str) -> ScoringResult {
    ScoringResult {
        total: 0,
        role_score: 0,
        experience_score: 0,
        salary_score: 0,
        industry_score: 0,
        location_score: 0,
        growth_score: 0,
        strategic_score: 0,
        bonus: 0,
        disqualified: true,
        disqualify_reason: Some(reason.to_string()),
    }
}

// ─────────────────────────────────────────────────────
// Display
// ─────────────────────────────────────────────────────

impl std::fmt::Display for ScoringResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.disqualified {
            write!(f, "DISQUALIFIED — {}", self.disqualify_reason.as_deref().unwrap_or("unknown"))
        } else {
            write!(
                f,
                "Score: {}/100 (role={} exp={} salary={} industry={} location={} growth={} strategic={} bonus={})",
                self.total,
                self.role_score,
                self.experience_score,
                self.salary_score,
                self.industry_score,
                self.location_score,
                self.growth_score,
                self.strategic_score,
                self.bonus,
            )
        }
    }
}
