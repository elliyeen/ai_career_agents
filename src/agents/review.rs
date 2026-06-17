/// Weekly review agent — computes pipeline metrics, identifies the weakest
/// funnel stage, and surfaces actionable recommendations.
use crate::db::Db;
use crate::models::Metric;
use anyhow::Result;

pub fn run(db: &Db) -> Result<Metric> {
    let m = db.current_metrics()?;

    println!("\n─── CareerOS Weekly Review ──────────────────────────────\n");
    println!("  Jobs discovered:   {}", m.jobs_discovered);
    println!("  Jobs qualified:    {}", m.jobs_qualified);
    println!("  Applications:      {}", m.applications);
    println!("  Responses:         {}", m.responses);
    println!("  Interviews:        {}", m.interviews);
    println!("  Offers:            {}", m.offers);
    println!("  Rejections:        {}", m.rejections);
    println!();

    // Conversion rates
    if m.jobs_discovered > 0 {
        let q_rate = m.jobs_qualified as f64 / m.jobs_discovered as f64 * 100.0;
        println!("  Discovery → Qualify:    {:.0}%", q_rate);
    }
    if m.jobs_qualified > 0 {
        let a_rate = m.applications as f64 / m.jobs_qualified as f64 * 100.0;
        println!("  Qualify → Apply:        {:.0}%", a_rate);
    }
    if m.applications > 0 {
        let r_rate = m.responses as f64 / m.applications as f64 * 100.0;
        println!("  Apply → Response:       {:.0}%  (industry avg ~10-15%)", r_rate);
    }
    if m.responses > 0 {
        let i_rate = m.interviews as f64 / m.responses as f64 * 100.0;
        println!("  Response → Interview:   {:.0}%", i_rate);
    }
    if m.interviews > 0 {
        let o_rate = m.offers as f64 / m.interviews as f64 * 100.0;
        println!("  Interview → Offer:      {:.0}%", o_rate);
    }
    println!();

    // Weakest stage
    let weak = weakest_stage(&m);
    println!("  Weakest stage:     {}", weak);
    println!();

    // Recommendations
    let recs = recommendations(&m);
    if !recs.is_empty() {
        println!("─── Recommendations ─────────────────────────────────────\n");
        for rec in &recs {
            println!("  • {}", rec);
        }
        println!();
    }

    Ok(m)
}

fn weakest_stage(m: &Metric) -> &'static str {
    if m.jobs_discovered == 0 {
        return "Discovery — no jobs found yet";
    }
    if m.jobs_qualified == 0 {
        return "Qualification — no jobs have been scored";
    }
    if m.applications == 0 {
        return "Application — qualified jobs exist but nothing submitted";
    }
    if m.responses == 0 && m.applications > 3 {
        return "Response rate — applications sent but no recruiter responses";
    }
    if m.interviews == 0 && m.responses > 2 {
        return "Conversion to interview — getting responses but no interview offers";
    }
    if m.offers == 0 && m.interviews > 2 {
        return "Offer conversion — interviewing but not reaching offer stage";
    }
    "No clear bottleneck identified yet"
}

fn recommendations(m: &Metric) -> Vec<String> {
    let mut recs = Vec::new();

    if m.jobs_discovered == 0 {
        recs.push("Add job files to data/inbox/ and run `career-os discover`".to_string());
        return recs;
    }

    if m.jobs_qualified == 0 {
        recs.push("Run `career-os qualify` to score discovered jobs against the rubric".to_string());
        return recs;
    }

    if m.applications == 0 {
        recs.push("Run `career-os resume --job-id <id>` for your top qualified job".to_string());
    }

    if m.applications > 0 && m.responses == 0 {
        recs.push("Response rate is 0% — review resume and outreach message quality".to_string());
        recs.push("Consider running `career-os outreach --application-id <id>` to generate a follow-up".to_string());
    }

    if m.applications > 5 && m.responses > 0 {
        let rate = m.responses as f64 / m.applications as f64;
        if rate < 0.10 {
            recs.push(format!(
                "Response rate is {:.0}% — below 10% target. Review resume keyword alignment.",
                rate * 100.0
            ));
        }
    }

    if m.interviews > 0 && m.offers == 0 && m.interviews >= 3 {
        recs.push("0 offers from interviews — run `career-os interview` to improve prep materials".to_string());
    }

    if recs.is_empty() {
        recs.push("Pipeline looks healthy. Keep applying.".to_string());
    }

    recs
}
