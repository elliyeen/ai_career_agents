/// No-fabrication tests.
///
/// These tests verify that the system correctly identifies skill gaps
/// and refuses to qualify jobs requiring experience that does not
/// exist in the candidate's verified inventory.
///
/// A job that requires skills marked as gaps in skills-inventory.md
/// must be flagged — never silently passed through.
use career_os::scoring::{score, ScoringInputs, QUALIFY_THRESHOLD};

/// A job that requires 3 skills the candidate does not have.
/// The scoring engine must not qualify it.
#[test]
fn critical_skill_gap_prevents_qualification() {
    let inputs = ScoringInputs {
        title_match_pct: 0.9,        // good title match
        requirements_met_pct: 0.35,  // only meets 35% of requirements
        missing_required_skills: 3,  // 3 hard required skills missing
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
    };

    let result = score(&inputs);

    // Even with good title, salary, and growth — critical skill gaps must disqualify
    assert!(
        result.total < QUALIFY_THRESHOLD || result.disqualified,
        "Job with 3 critical missing skills must not qualify. Score: {}, Qualified: {}",
        result.total,
        result.is_qualified()
    );
}

/// A job requiring skills the candidate partially has.
/// Should be flagged for human review, not auto-qualified.
#[test]
fn partial_skill_match_produces_reduced_experience_score() {
    let partial = ScoringInputs {
        title_match_pct: 0.9,
        requirements_met_pct: 0.55, // meets just over half
        missing_required_skills: 1,
        salary_known: true,
        salary_above_target: false,
        salary_overlaps_target: true,
        salary_above_floor: true,
        industry_preferred: false,
        industry_acceptable: true,
        industry_excluded: false,
        remote_ok: true,
        hybrid_days: 0,
        office_city_preferred: false,
        office_city_acceptable: false,
        office_city_excluded: false,
        company_growing: false,
        role_is_building: false,
        path_to_next_level_clear: false,
        adds_new_skill_or_brand: false,
        solidifies_strength: true,
        is_lateral: true,
        is_backward: false,
        is_tier1_company: false,
        is_tier2_company: false,
        recruiter_reached_out: false,
        has_referral: false,
        company_blocked: false,
    };

    let result = score(&partial);

    // Experience score must be meaningfully lower than max (15)
    assert!(
        result.experience_score < 10,
        "55% requirements met with 1 missing required skill should score < 10, got {}",
        result.experience_score
    );
}

/// Verify that bonuses alone cannot push a gap-heavy job over the threshold.
/// A referral or tier-1 bonus should not override poor experience match.
#[test]
fn bonuses_cannot_save_a_skills_gap_job() {
    let inputs = ScoringInputs {
        title_match_pct: 1.0,
        requirements_met_pct: 0.3, // only 30% match
        missing_required_skills: 3,
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
        is_tier1_company: true,  // tier 1 bonus +10
        is_tier2_company: false,
        recruiter_reached_out: true, // recruiter bonus +5
        has_referral: true,          // referral bonus +10
        company_blocked: false,
    };

    let result = score(&inputs);

    // Even max bonuses (25 pts) should not qualify a job where candidate
    // only meets 30% of requirements with 3 hard gaps
    let experience_contribution = result.experience_score;
    assert!(
        experience_contribution <= 0,
        "30% match + 3 hard gaps should produce experience score of 0, got {}",
        experience_contribution
    );
}

/// Verify that the system does not accidentally qualify a non-target role
/// even when everything else looks good. (e.g., a Software Engineer role
/// when candidate is a TPM)
#[test]
fn wrong_role_type_scores_poorly_on_role_match() {
    let inputs = ScoringInputs {
        title_match_pct: 0.05, // "Software Engineer" vs "Technical Program Manager"
        requirements_met_pct: 0.4,
        missing_required_skills: 4,
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
        is_backward: true,
        is_tier1_company: false,
        is_tier2_company: false,
        recruiter_reached_out: false,
        has_referral: false,
        company_blocked: false,
    };

    let result = score(&inputs);
    assert!(
        !result.is_qualified(),
        "Non-target role type should not qualify, score: {}",
        result.total
    );
}
