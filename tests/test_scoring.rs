/// Tests for the job scoring engine.
/// Validates that the rubric from docs/config/scoring.md is applied correctly.
use career_os::scoring::{score, ScoringInputs, PRIORITY_THRESHOLD, QUALIFY_THRESHOLD};

// ─────────────────────────────────────────────────────
// Helper: baseline passing inputs
// ─────────────────────────────────────────────────────

fn strong_match() -> ScoringInputs {
    ScoringInputs {
        title_match_pct: 1.0,
        requirements_met_pct: 0.95,
        missing_required_skills: 0,
        salary_known: true,
        salary_above_target: true,
        salary_above_floor: true,
        salary_overlaps_target: true,
        industry_preferred: true,
        industry_acceptable: true,
        industry_excluded: false,
        remote_ok: true,
        hybrid_days: 0,
        office_city_preferred: false,
        office_city_acceptable: false,
        office_city_excluded: false,
        company_growing: true,
        role_is_building: true,
        path_to_next_level_clear: true,
        adds_new_skill_or_brand: true,
        solidifies_strength: false,
        is_lateral: false,
        is_backward: false,
        is_tier1_company: false,
        is_tier2_company: false,
        recruiter_reached_out: false,
        has_referral: false,
        company_blocked: false,
    }
}

// ─────────────────────────────────────────────────────
// Threshold tests
// ─────────────────────────────────────────────────────

#[test]
fn strong_match_qualifies() {
    let result = score(&strong_match());
    assert!(!result.disqualified, "Strong match should not be disqualified");
    assert!(
        result.total >= QUALIFY_THRESHOLD,
        "Strong match score {} should be >= {}",
        result.total,
        QUALIFY_THRESHOLD
    );
}

#[test]
fn tier1_company_bonus_pushes_to_priority() {
    let mut inputs = strong_match();
    inputs.is_tier1_company = true;
    let result = score(&inputs);
    assert!(
        result.total >= PRIORITY_THRESHOLD,
        "Tier 1 + strong match should reach priority threshold, got {}",
        result.total
    );
    assert_eq!(result.bonus, 10);
}

#[test]
fn referral_adds_ten_points() {
    let baseline = score(&strong_match()).total;
    let mut inputs = strong_match();
    inputs.has_referral = true;
    let with_referral = score(&inputs).total;
    assert_eq!(with_referral - baseline, 10, "Referral should add exactly 10 points");
}

#[test]
fn weak_title_match_does_not_qualify() {
    let mut inputs = strong_match();
    inputs.title_match_pct = 0.1; // very weak title match
    let result = score(&inputs);
    // With a 10% title match the role score drops significantly
    assert!(
        result.role_score <= 3,
        "10% title match should produce role score <= 3, got {}",
        result.role_score
    );
}

// ─────────────────────────────────────────────────────
// Hard disqualification tests
// ─────────────────────────────────────────────────────

#[test]
fn blocked_company_disqualifies_immediately() {
    let mut inputs = strong_match();
    inputs.company_blocked = true;
    let result = score(&inputs);
    assert!(result.disqualified);
    assert_eq!(result.disqualify_reason.as_deref(), Some("EXCLUDED_COMPANY"));
    assert_eq!(result.total, 0);
}

#[test]
fn excluded_industry_disqualifies_even_with_perfect_role_match() {
    let mut inputs = strong_match();
    inputs.industry_excluded = true;
    let result = score(&inputs);
    assert!(result.disqualified);
    assert_eq!(result.disqualify_reason.as_deref(), Some("EXCLUDED_INDUSTRY"));
}

#[test]
fn salary_below_floor_disqualifies() {
    let mut inputs = strong_match();
    inputs.salary_known = true;
    inputs.salary_above_floor = false;
    inputs.salary_above_target = false;
    inputs.salary_overlaps_target = false;
    let result = score(&inputs);
    assert!(result.disqualified);
    assert_eq!(result.disqualify_reason.as_deref(), Some("SALARY_BELOW_FLOOR"));
}

#[test]
fn excluded_city_inoffice_disqualifies() {
    let mut inputs = strong_match();
    inputs.remote_ok = false;
    inputs.office_city_excluded = true;
    let result = score(&inputs);
    assert!(result.disqualified);
    assert_eq!(result.disqualify_reason.as_deref(), Some("LOCATION_INCOMPATIBLE"));
}

// ─────────────────────────────────────────────────────
// Score component tests
// ─────────────────────────────────────────────────────

#[test]
fn missing_required_skills_reduces_experience_score() {
    let inputs_no_gap = strong_match();
    let mut inputs_with_gap = strong_match();
    inputs_with_gap.missing_required_skills = 2;

    let score_no_gap = score(&inputs_no_gap).experience_score;
    let score_with_gap = score(&inputs_with_gap).experience_score;

    assert!(
        score_no_gap > score_with_gap,
        "Missing required skills should reduce experience score: {} vs {}",
        score_no_gap,
        score_with_gap
    );
    // Each missing skill deducts 5 points
    assert!(score_no_gap - score_with_gap >= 10, "2 missing skills should deduct >= 10 points");
}

#[test]
fn unknown_salary_gets_partial_credit_not_zero() {
    let mut inputs = strong_match();
    inputs.salary_known = false;
    inputs.salary_above_target = false;
    inputs.salary_above_floor = false;
    inputs.salary_overlaps_target = false;
    let result = score(&inputs);
    assert!(!result.disqualified, "Unknown salary should not auto-disqualify");
    assert!(result.salary_score > 0, "Unknown salary should get partial credit");
    assert!(result.salary_score < 8, "Unknown salary should not get full credit");
}

#[test]
fn remote_scores_higher_than_hybrid() {
    let fully_remote = strong_match(); // hybrid_days = 0
    let mut hybrid = strong_match();
    hybrid.hybrid_days = 3;

    let remote_score = score(&fully_remote).location_score;
    let hybrid_score = score(&hybrid).location_score;

    assert!(
        remote_score > hybrid_score,
        "Fully remote should score higher than hybrid: {} vs {}",
        remote_score,
        hybrid_score
    );
}

#[test]
fn score_components_sum_to_total() {
    let result = score(&strong_match());
    let expected_total = result.role_score
        + result.experience_score
        + result.salary_score
        + result.industry_score
        + result.location_score
        + result.growth_score
        + result.strategic_score
        + result.bonus;
    assert_eq!(result.total, expected_total, "Score components must sum to total");
}

// ─────────────────────────────────────────────────────
// Bonus stacking test
// ─────────────────────────────────────────────────────

#[test]
fn all_bonuses_stack_correctly() {
    let mut inputs = strong_match();
    inputs.is_tier1_company = true;
    inputs.has_referral = true;
    inputs.recruiter_reached_out = true;
    let result = score(&inputs);
    assert_eq!(result.bonus, 25, "Tier1(10) + referral(10) + recruiter(5) = 25");
}
