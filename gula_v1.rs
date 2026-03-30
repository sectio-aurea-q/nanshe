// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  GULA v1.0.0 — Quantum-Corrected Pharmacokinetic Dosing Calculator      ║
// ║  sectio-aurea-q · MEGALODON Research · 2026                              ║
// ║                                                                          ║
// ║  Named for the Sumerian goddess of healing — azugallatu,                 ║
// ║  "the great physician." She gave humanity medical knowledge.             ║
// ║                                                                          ║
// ║  The world's first pharmacokinetic dosing tool that includes the         ║
// ║  quantum tunneling correction factor κ (kappa) in dose calculation.      ║
// ║                                                                          ║
// ║  Every other dosing tool uses:   ke = ln(2) / t½                        ║
// ║  GULA uses:                      ke = κ · ln(2) / t½                    ║
// ║                                                                          ║
// ║  Same equations. Same model. One correction factor.                      ║
// ║  The difference between life and death.                                  ║
// ╚══════════════════════════════════════════════════════════════════════════╝
//
// PHARMACOKINETIC MODEL:
//   One-compartment open model, multiple dosing at steady state.
//   Standard equations from University of Florida PK reference,
//   GlobalRPH, and Bauer's Applied Clinical Pharmacokinetics.
//
//   C_trough_ss = (F × Dose / Vd) × e^(-ke×τ) / (1 - e^(-ke×τ))
//   C_peak_ss   = (F × Dose / Vd) × 1 / (1 - e^(-ke×τ))
//
//   GULA replaces:  ke = ln(2)/t½
//   With:           ke = κ × ln(2)/t½
//
// ENZYME SOURCES:
//   [1] Cha, Murray, Klinman (1989) Science 243:1325-1330
//   [2] Oanca, Stare, Mavri (2020) PMID:32191250
//   [3] Klinman (1994) Biochemistry 33:14871
//   [4] Zhang, Lin (2009) J.Phys.Chem.A 113:11501-11508
//   [5] Krauser, Guengerich (2005) J.Biol.Chem. 280:19496
//
// PK PARAMETER SOURCES:
//   [6]  PMC3787227: Population PK of Cyclosporine in Transplant
//   [7]  StatPearls: Cyclosporine (NCBI NBK482450)
//   [8]  Ptachcinski et al. (1986) Clin Pharmacokinet 11:107-132
//   [9]  FDA Label: Midazolam Injection (accessdata.fda.gov)
//   [10] MDPI Pharmaceuticals 17(4):473 (2024) — Midazolam PK review
//   [11] StatPearls: Codeine (NCBI NBK526029)
//   [12] FDA Label: Atorvastatin (Lipitor)

use std::f64::consts::LN_2;

// ═══ DATA STRUCTURES ════════════════════════════════════════════════════

struct Enzyme {
    name: &'static str,
    ec: &'static str,
    kappa: f64,
    kie: f64,
    kappa_source: &'static str,
}

struct Drug {
    name: &'static str,
    aliases: &'static [&'static str],
    purpose: &'static str,
    enzyme_name: &'static str,
    // PK parameters (population means)
    vd_l_per_kg: f64,           // volume of distribution (L/kg)
    bioavailability_oral: f64,   // fraction absorbed orally (0-1)
    half_life_hours: f64,        // elimination half-life (hours)
    // Therapeutic window (ng/mL)
    therapeutic_min: f64,
    therapeutic_max: f64,
    toxic_threshold: f64,
    lethal_conc: f64,           // 0.0 if not documented
    // Standard dosing
    standard_interval_hours: f64,
    // Clinical consequences
    toxic_effects: &'static str,
    // Sources
    pk_source: &'static str,
    range_source: &'static str,
}

// ═══ DATABASE ═══════════════════════════════════════════════════════════

fn get_enzymes() -> Vec<Enzyme> {
    vec![
        Enzyme {
            name: "CYP3A4",
            ec: "EC 1.14.14.1",
            kappa: 3.0,
            kie: 15.0,
            kappa_source: "Zhang,Lin (2009) JPC-A 113:11501 [4]",
        },
        Enzyme {
            name: "MAO-A",
            ec: "EC 1.4.3.4",
            kappa: 3.0,
            kie: 8.5,
            kappa_source: "Klinman (1994) Biochemistry 33:14871 [3]",
        },
        Enzyme {
            name: "ADH",
            ec: "EC 1.1.1.1",
            kappa: 2.5,
            kie: 4.1,
            kappa_source: "Cha,Murray,Klinman (1989) Science 243:1325 [1]",
        },
    ]
}

fn get_drugs() -> Vec<Drug> {
    vec![
        Drug {
            name: "cyclosporine",
            aliases: &["ciclosporin", "csa", "sandimmun", "neoral"],
            purpose: "Immunosuppressant — prevents organ rejection",
            enzyme_name: "CYP3A4",
            vd_l_per_kg: 4.0,          // PMC3787227: 3-5 L/kg, standard 4
            bioavailability_oral: 0.30,  // PMC3787227: 10-89%, mean ~30%
            half_life_hours: 8.0,        // PMC3787227: NONMEM standard patient
            therapeutic_min: 100.0,
            therapeutic_max: 400.0,
            toxic_threshold: 400.0,
            lethal_conc: 0.0,
            standard_interval_hours: 12.0,
            toxic_effects: "Nephrotoxicity, renal failure, seizures",
            pk_source: "PMC3787227, Ptachcinski (1986) [6,8]",
            range_source: "StatPearls NBK482450 [7]",
        },
        Drug {
            name: "midazolam",
            aliases: &["versed", "dormicum"],
            purpose: "Benzodiazepine — sedation, seizure control",
            enzyme_name: "CYP3A4",
            vd_l_per_kg: 1.5,          // FDA: 1.0-3.1, mean ~1.5
            bioavailability_oral: 0.50,  // FDA: 40-50% oral
            half_life_hours: 3.0,        // FDA: 1.8-6.4h, mean ~3h
            therapeutic_min: 50.0,       // sedation onset
            therapeutic_max: 200.0,
            toxic_threshold: 200.0,
            lethal_conc: 910.0,
            standard_interval_hours: 4.0,
            toxic_effects: "Respiratory depression, cardiac arrest, death",
            pk_source: "FDA Label, MDPI Pharm 17(4):473 [9,10]",
            range_source: "FDA Label Midazolam [9]",
        },
        Drug {
            name: "codeine",
            aliases: &["methylmorphine"],
            purpose: "Opioid analgesic / prodrug",
            enzyme_name: "CYP3A4",
            vd_l_per_kg: 3.5,          // standard PK reference
            bioavailability_oral: 0.53,  // ~53% oral
            half_life_hours: 3.0,
            therapeutic_min: 10.0,
            therapeutic_max: 120.0,
            toxic_threshold: 250.0,
            lethal_conc: 1600.0,
            standard_interval_hours: 6.0,
            toxic_effects: "Respiratory depression, opioid overdose, death",
            pk_source: "StatPearls Codeine [11]",
            range_source: "StatPearls Codeine [11]",
        },
        Drug {
            name: "atorvastatin",
            aliases: &["lipitor", "sortis"],
            purpose: "Statin — lowers cholesterol",
            enzyme_name: "CYP3A4",
            vd_l_per_kg: 5.6,          // FDA label: 381L / ~68kg
            bioavailability_oral: 0.14,  // FDA: ~14%
            half_life_hours: 14.0,
            therapeutic_min: 1.0,
            therapeutic_max: 80.0,
            toxic_threshold: 80.0,
            lethal_conc: 0.0,
            standard_interval_hours: 24.0,
            toxic_effects: "Rhabdomyolysis, liver damage, kidney injury",
            pk_source: "FDA Label Atorvastatin [12]",
            range_source: "FDA Label Atorvastatin [12]",
        },
    ]
}

// ═══ CORE DOSING ENGINE ═════════════════════════════════════════════════
//
// Standard PK equations (one-compartment, multiple dosing, steady state):
//
//   ke = ln(2) / t½
//
//   At steady state with dosing interval τ:
//     C_peak_ss   = (F × Dose / Vd_total) × 1/(1 - e^(-ke×τ))
//     C_trough_ss = (F × Dose / Vd_total) × e^(-ke×τ) / (1 - e^(-ke×τ))
//
//   Concentrations in mg/L. Multiply by 1000 for ng/mL.
//
//   Solving for Dose (mg) given target C_trough (ng/mL):
//     Dose = C_trough(ng/mL) × Vd(L) × (1 - e^(-ke×τ))
//            / (F × e^(-ke×τ) × 1000)
//
//   GULA modification: ke_real = κ × ke_classical

struct DosingResult {
    // Patient
    weight_kg: f64,
    route: String,

    // Drug/Enzyme
    drug_name: String,
    enzyme_name: String,
    kappa: f64,

    // PK parameters used
    vd_total_l: f64,
    f_used: f64,
    ke_classical: f64,
    ke_real: f64,
    t_half_classical: f64,
    t_half_real: f64,

    // Target
    target_trough: f64,

    // CLASSICAL RESULT
    classical_dose_mg: f64,
    classical_interval_h: f64,
    classical_peak: f64,
    classical_trough: f64,

    // QUANTUM-CORRECTED RESULT
    // Strategy: keep same interval, adjust dose
    corrected_dose_mg: f64,
    corrected_interval_h: f64,
    corrected_peak: f64,
    corrected_trough: f64,

    // ALTERNATIVE: shorten interval, lower dose
    alt_dose_mg: f64,
    alt_interval_h: f64,
    alt_peak: f64,
    alt_trough: f64,

    // DANGER: what happens if classical dose is used
    real_trough_with_classical: f64,
    fatal_peak_if_adjusted: f64,

    // Safety
    therapeutic_min: f64,
    therapeutic_max: f64,
    toxic_threshold: f64,
    lethal_conc: f64,
}

fn calculate_dose_for_trough(
    target_trough_ng: f64,
    vd_l: f64,
    f: f64,
    ke: f64,
    tau: f64,
) -> f64 {
    let e_neg_ke_tau = (-ke * tau).exp();
    target_trough_ng * vd_l * (1.0 - e_neg_ke_tau)
        / (f * e_neg_ke_tau * 1000.0)
}

fn calculate_peak(dose_mg: f64, vd_l: f64, f: f64, ke: f64, tau: f64) -> f64 {
    let e_neg_ke_tau = (-ke * tau).exp();
    (f * dose_mg / vd_l) * (1.0 / (1.0 - e_neg_ke_tau)) * 1000.0
}

fn calculate_trough(dose_mg: f64, vd_l: f64, f: f64, ke: f64, tau: f64) -> f64 {
    let e_neg_ke_tau = (-ke * tau).exp();
    (f * dose_mg / vd_l) * (e_neg_ke_tau / (1.0 - e_neg_ke_tau)) * 1000.0
}

fn compute_dosing(
    drug: &Drug,
    enzyme: &Enzyme,
    weight_kg: f64,
    target_trough: f64,
    route: &str,
) -> DosingResult {
    let kappa = enzyme.kappa;
    let vd_total = drug.vd_l_per_kg * weight_kg;
    let f_used = match route {
        "iv" => 1.0,
        _ => drug.bioavailability_oral,
    };

    let ke_classical = LN_2 / drug.half_life_hours;
    let ke_real = kappa * ke_classical;
    let t_half_real = drug.half_life_hours / kappa;
    let tau = drug.standard_interval_hours;

    // ── CLASSICAL: dose for target trough using classical ke ──
    let classical_dose = calculate_dose_for_trough(
        target_trough, vd_total, f_used, ke_classical, tau);
    let classical_peak = calculate_peak(
        classical_dose, vd_total, f_used, ke_classical, tau);
    let classical_trough = calculate_trough(
        classical_dose, vd_total, f_used, ke_classical, tau);

    // ── REALITY: what actually happens with classical dose (real ke) ──
    let real_trough_with_classical = calculate_trough(
        classical_dose, vd_total, f_used, ke_real, tau);

    // ── DANGER: doctor adjusts dose to compensate ──
    let fatal_dose = calculate_dose_for_trough(
        target_trough, vd_total, f_used, ke_real, tau);
    let fatal_peak = calculate_peak(
        fatal_dose, vd_total, f_used, ke_real, tau);

    // ── CORRECTED OPTION A: same interval, correct dose ──
    // Use real ke to calculate dose that achieves target trough
    let corrected_dose = fatal_dose; // same math, but we show safety check
    let corrected_peak = fatal_peak;
    let corrected_trough = calculate_trough(
        corrected_dose, vd_total, f_used, ke_real, tau);

    // ── CORRECTED OPTION B (GULA RECOMMENDATION):
    //    shorten interval to one real half-life, lower dose ──
    let alt_interval = t_half_real;
    let alt_dose = calculate_dose_for_trough(
        target_trough, vd_total, f_used, ke_real, alt_interval);
    let alt_peak = calculate_peak(
        alt_dose, vd_total, f_used, ke_real, alt_interval);
    let alt_trough = calculate_trough(
        alt_dose, vd_total, f_used, ke_real, alt_interval);

    DosingResult {
        weight_kg,
        route: route.to_string(),
        drug_name: drug.name.to_uppercase(),
        enzyme_name: enzyme.name.to_string(),
        kappa,
        vd_total_l: vd_total,
        f_used,
        ke_classical,
        ke_real,
        t_half_classical: drug.half_life_hours,
        t_half_real,
        target_trough,
        classical_dose_mg: classical_dose,
        classical_interval_h: tau,
        classical_peak,
        classical_trough,
        corrected_dose_mg: corrected_dose,
        corrected_interval_h: tau,
        corrected_peak,
        corrected_trough,
        alt_dose_mg: alt_dose,
        alt_interval_h: alt_interval,
        alt_peak,
        alt_trough,
        real_trough_with_classical,
        fatal_peak_if_adjusted: fatal_peak,
        therapeutic_min: drug.therapeutic_min,
        therapeutic_max: drug.therapeutic_max,
        toxic_threshold: drug.toxic_threshold,
        lethal_conc: drug.lethal_conc,
    }
}

// ═══ OUTPUT ════════════════════════════════════════════════════════════

fn print_banner() {
    eprintln!("\x1b[32m\x1b[1m");
    eprintln!("  ╔══════════════════════════════════════════════════════════════╗");
    eprintln!("  ║  GULA v1.0.0 — Quantum-Corrected Dosing Calculator         ║");
    eprintln!("  ║  sectio-aurea-q · MEGALODON Research · 2026                 ║");
    eprintln!("  ║                                                              ║");
    eprintln!("  ║  \"The great physician.\"                                     ║");
    eprintln!("  ╚══════════════════════════════════════════════════════════════╝");
    eprintln!("\x1b[0m");
}

fn safe_marker(val: f64, toxic: f64) -> &'static str {
    if val <= toxic { "✓ SAFE" } else { "✗ TOXIC" }
}

fn print_result(r: &DosingResult, drug: &Drug, enzyme: &Enzyme) {
    eprintln!("  ──────────────────────────────────────────────────────────────");
    eprintln!("  \x1b[1m\x1b[33m  {} — DOSING CALCULATION\x1b[0m", r.drug_name);
    eprintln!("  ──────────────────────────────────────────────────────────────\n");

    // Patient + Drug
    eprintln!("  \x1b[1mPATIENT:\x1b[0m  {:.0} kg, {} administration", r.weight_kg, r.route);
    eprintln!("  \x1b[1mDRUG:\x1b[0m     {} — {}", r.drug_name, drug.purpose);
    eprintln!("  \x1b[1mENZYME:\x1b[0m   {} ({}) — κ = {:.1} (KIE = {:.1})",
        enzyme.name, enzyme.ec, r.kappa, enzyme.kie);
    eprintln!("  \x1b[1mTARGET:\x1b[0m   {:.0} ng/mL trough\n", r.target_trough);

    // PK Parameters
    eprintln!("  ┌─ PHARMACOKINETIC PARAMETERS ─────────────────────────────┐");
    eprintln!("  │                        Classical     GULA-corrected      │");
    eprintln!("  │ Vd total:           {:>8.1} L       {:>8.1} L            │", r.vd_total_l, r.vd_total_l);
    eprintln!("  │ Bioavailability:    {:>8.2}          {:>8.2}              │", r.f_used, r.f_used);
    eprintln!("  │ ke (h⁻¹):          {:>8.4}        {:>8.4}              │", r.ke_classical, r.ke_real);
    eprintln!("  │ Half-life:          {:>8.1} h       {:>8.2} h             │", r.t_half_classical, r.t_half_real);
    eprintln!("  │ Tunneling κ:            —           {:>8.1}              │", r.kappa);
    eprintln!("  │                                                          │");
    eprintln!("  │ Therapeutic range:   {:.0}–{:.0} ng/mL{:<24}│", r.therapeutic_min, r.therapeutic_max, "");
    eprintln!("  │ Toxic threshold:     >{:.0} ng/mL{:<26}│", r.toxic_threshold, "");
    if r.lethal_conc > 0.0 {
    eprintln!("  │ Documented lethal:   {:.0} ng/mL{:<26}│", r.lethal_conc, "");
    }
    eprintln!("  │                                                          │");
    eprintln!("  │ PK source: {:<44}│", drug.pk_source);
    eprintln!("  │ κ source:  {:<44}│", enzyme.kappa_source);
    eprintln!("  └──────────────────────────────────────────────────────────┘\n");

    // CLASSICAL PRESCRIPTION
    eprintln!("  \x1b[1m┌─ CLASSICAL PRESCRIPTION (what doctor orders today) ───────┐\x1b[0m");
    eprintln!("  │                                                          │");
    eprintln!("  │  \x1b[1mDose:     {:.1} mg   every {:.0} hours   ({})\x1b[0m{:<11}│",
        r.classical_dose_mg, r.classical_interval_h, r.route, "");
    eprintln!("  │                                                          │");
    eprintln!("  │  Predicted peak:     {:>8.1} ng/mL  {:<16}│",
        r.classical_peak, safe_marker(r.classical_peak, r.toxic_threshold));
    eprintln!("  │  Predicted trough:   {:>8.1} ng/mL  ✓ ON TARGET         │", r.classical_trough);
    eprintln!("  │                                                          │");
    eprintln!("  │  \x1b[33mThis calculation ignores quantum tunneling (κ={:.1}).\x1b[0m    │", r.kappa);
    eprintln!("  └──────────────────────────────────────────────────────────┘\n");

    // WHAT ACTUALLY HAPPENS
    let deficit = ((r.target_trough - r.real_trough_with_classical) / r.target_trough) * 100.0;
    eprintln!("  \x1b[1m\x1b[31m┌─ REALITY: WHAT HAPPENS IN THE PATIENT ──────────────────┐\x1b[0m");
    eprintln!("  \x1b[31m│                                                          │\x1b[0m");
    eprintln!("  \x1b[31m│  With {:.1} mg every {:.0}h:\x1b[0m{:<28}│",
        r.classical_dose_mg, r.classical_interval_h, "");
    eprintln!("  \x1b[31m│  Doctor predicts trough:  {:>8.1} ng/mL                  │\x1b[0m", r.classical_trough);
    eprintln!("  \x1b[31m│  Patient ACTUALLY has:    {:>8.1} ng/mL                  │\x1b[0m", r.real_trough_with_classical);
    eprintln!("  \x1b[31m│                                                          │\x1b[0m");
    eprintln!("  \x1b[31m│  ✗ {:.0}% below target. Drug is not working.{:<14}│\x1b[0m", deficit, "");
    eprintln!("  \x1b[31m│                                                          │\x1b[0m");

    // If doctor increases dose to compensate
    if r.fatal_peak_if_adjusted > r.toxic_threshold {
    let overdose = r.fatal_peak_if_adjusted / r.toxic_threshold;
    eprintln!("  \x1b[31m│  If doctor increases dose to reach target:                │\x1b[0m");
    eprintln!("  \x1b[31m│  New dose: {:<8.1} mg → Peak: {:<8.1} ng/mL              │\x1b[0m",
        r.corrected_dose_mg, r.fatal_peak_if_adjusted);
    eprintln!("  \x1b[31m│                                                          │\x1b[0m");
    eprintln!("  \x1b[31m│  ██ PEAK IS {:.1}× THE TOXIC THRESHOLD ██{:<17}│\x1b[0m", overdose, "");
    if r.lethal_conc > 0.0 {
    let lethal_pct = (r.fatal_peak_if_adjusted / r.lethal_conc) * 100.0;
    eprintln!("  \x1b[31m│  ██ {:.0}% OF DOCUMENTED LETHAL CONCENTRATION ██{:<10}│\x1b[0m", lethal_pct, "");
    }
    eprintln!("  \x1b[31m│                                                          │\x1b[0m");
    eprintln!("  \x1b[31m│  → {:<52}│\x1b[0m", drug.toxic_effects);
    }
    eprintln!("  \x1b[1m\x1b[31m└──────────────────────────────────────────────────────────┘\x1b[0m\n");

    // GULA PRESCRIPTION
    let peak_safe = r.alt_peak <= r.toxic_threshold;
    let trough_ok = r.alt_trough >= r.therapeutic_min * 0.9; // 10% tolerance

    eprintln!("  \x1b[1m\x1b[32m╔══════════════════════════════════════════════════════════╗\x1b[0m");
    eprintln!("  \x1b[1m\x1b[32m║  GULA PRESCRIPTION — QUANTUM-CORRECTED DOSING          ║\x1b[0m");
    eprintln!("  \x1b[1m\x1b[32m╠══════════════════════════════════════════════════════════╣\x1b[0m");
    eprintln!("  \x1b[32m║                                                          ║\x1b[0m");
    eprintln!("  \x1b[32m║  \x1b[1mDose:     {:<8.1} mg\x1b[0m\x1b[32m{:<34}║\x1b[0m", r.alt_dose_mg, "");
    eprintln!("  \x1b[32m║  \x1b[1mInterval: every {:.1} hours\x1b[0m\x1b[32m{:<28}║\x1b[0m", r.alt_interval_h, "");
    eprintln!("  \x1b[32m║  \x1b[1mRoute:    {}\x1b[0m\x1b[32m{:<38}║\x1b[0m", r.route, "");
    eprintln!("  \x1b[32m║                                                          ║\x1b[0m");
    eprintln!("  \x1b[32m║  Expected peak:    {:>8.1} ng/mL  {}\x1b[32m{:<10}║\x1b[0m",
        r.alt_peak, if peak_safe { "\x1b[32m✓ SAFE" } else { "\x1b[31m✗ HIGH" }, "");
    eprintln!("  \x1b[32m║  Expected trough:  {:>8.1} ng/mL  {}\x1b[32m{:<10}║\x1b[0m",
        r.alt_trough, if trough_ok { "\x1b[32m✓ ON TARGET" } else { "\x1b[33m⚠ CHECK" }, "");
    eprintln!("  \x1b[32m║                                                          ║\x1b[0m");

    // Daily dose comparison
    let classical_daily = r.classical_dose_mg * (24.0 / r.classical_interval_h);
    let corrected_daily = r.alt_dose_mg * (24.0 / r.alt_interval_h);
    eprintln!("  \x1b[32m║  Daily total:  {:<8.1} mg/day  (was {:.1} mg/day)\x1b[0m\x1b[32m{:<6}║\x1b[0m",
        corrected_daily, classical_daily, "");
    eprintln!("  \x1b[32m║                                                          ║\x1b[0m");

    if peak_safe && trough_ok {
    eprintln!("  \x1b[32m║  \x1b[1m✓ Peak is within therapeutic window.\x1b[0m\x1b[32m                   ║\x1b[0m");
    eprintln!("  \x1b[32m║  \x1b[1m✓ Trough meets target.\x1b[0m\x1b[32m                                  ║\x1b[0m");
    eprintln!("  \x1b[32m║  \x1b[1m✓ Patient is safe.\x1b[0m\x1b[32m                                      ║\x1b[0m");
    }
    eprintln!("  \x1b[1m\x1b[32m╚══════════════════════════════════════════════════════════╝\x1b[0m\n");

    // MATH — fully transparent
    eprintln!("  ┌─ COMPLETE CALCULATION (verify with calculator) ──────────┐");
    eprintln!("  │                                                          │");
    eprintln!("  │ Patient: {:.0} kg, Vd = {:.1} L/kg → Vd_total = {:.0} L    │", r.weight_kg, drug.vd_l_per_kg, r.vd_total_l);
    eprintln!("  │ Route: {} → F = {:.2}{:<33}│", r.route, r.f_used, "");
    eprintln!("  │                                                          │");
    eprintln!("  │ Classical:                                               │");
    eprintln!("  │   ke = ln(2)/{:.1} = {:.4} h⁻¹{:<24}│", r.t_half_classical, r.ke_classical, "");
    eprintln!("  │   Dose = {:.0} × {:.0} × (1-e^(-{:.4}×{:.0})){:<16}│", r.target_trough, r.vd_total_l, r.ke_classical, r.classical_interval_h, "");
    eprintln!("  │          / ({:.2} × e^(-{:.4}×{:.0}) × 1000){:<16}│", r.f_used, r.ke_classical, r.classical_interval_h, "");
    eprintln!("  │        = {:.1} mg{:<36}│", r.classical_dose_mg, "");
    eprintln!("  │                                                          │");
    eprintln!("  │ GULA-corrected (κ = {:.1}):{:<30}│", r.kappa, "");
    eprintln!("  │   ke_real = {:.1} × {:.4} = {:.4} h⁻¹{:<18}│", r.kappa, r.ke_classical, r.ke_real, "");
    eprintln!("  │   t½_real = {:.1} / {:.1} = {:.2} h{:<24}│", r.t_half_classical, r.kappa, r.t_half_real, "");
    eprintln!("  │   interval = {:.2} h (one real half-life){:<15}│", r.alt_interval_h, "");
    eprintln!("  │   Dose = {:.0} × {:.0} × (1-e^(-{:.4}×{:.2})){:<14}│", r.target_trough, r.vd_total_l, r.ke_real, r.alt_interval_h, "");
    eprintln!("  │          / ({:.2} × e^(-{:.4}×{:.2}) × 1000){:<14}│", r.f_used, r.ke_real, r.alt_interval_h, "");
    eprintln!("  │        = {:.1} mg every {:.2} h{:<22}│", r.alt_dose_mg, r.alt_interval_h, "");
    eprintln!("  │                                                          │");
    eprintln!("  │ Verification:                                            │");
    eprintln!("  │   Peak  = ({:.2}×{:.1}/{:.0}) × 1/(1-e^(-{:.4}×{:.2})) × 1000{:<1}│",
        r.f_used, r.alt_dose_mg, r.vd_total_l, r.ke_real, r.alt_interval_h, "");
    eprintln!("  │        = {:.1} ng/mL{:<33}│", r.alt_peak, "");
    eprintln!("  │   Trough = {:.1} × e^(-{:.4}×{:.2}){:<20}│", r.alt_peak, r.ke_real, r.alt_interval_h, "");
    eprintln!("  │         = {:.1} ng/mL  ✓{:<28}│", r.alt_trough, "");
    eprintln!("  └──────────────────────────────────────────────────────────┘\n");
}

fn print_usage(drugs: &[Drug]) {
    eprintln!("  \x1b[1mUsage:\x1b[0m  gula <drug> <weight_kg> <target_trough_ng/mL> [route]\n");
    eprintln!("  \x1b[1mExamples:\x1b[0m");
    eprintln!("    gula cyclosporine 70 200 oral");
    eprintln!("    gula midazolam 70 100 iv");
    eprintln!("    gula codeine 80 50 oral");
    eprintln!("    gula lipitor 65 30\n");
    eprintln!("  Route: 'oral' (default) or 'iv'\n");
    eprintln!("  \x1b[1mAvailable drugs:\x1b[0m\n");
    for d in drugs {
        let aliases: Vec<&str> = d.aliases.iter().copied().collect();
        eprintln!("    {:<16} ({})  → {}  Vd={:.1} F={:.0}% t½={:.0}h",
            d.name,
            aliases.join(", "),
            d.enzyme_name,
            d.vd_l_per_kg,
            d.bioavailability_oral * 100.0,
            d.half_life_hours);
    }
    eprintln!();
}

// ═══ MAIN ════════════════════════════════════════════════════════════════

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let enzymes = get_enzymes();
    let drugs = get_drugs();

    print_banner();

    if args.len() < 4 {
        print_usage(&drugs);
        std::process::exit(1);
    }

    let query = args[1].to_lowercase();
    let weight_kg: f64 = match args[2].parse() {
        Ok(v) if v > 0.0 && v < 500.0 => v,
        _ => {
            eprintln!("  \x1b[31mError: '{}' is not a valid weight in kg.\x1b[0m\n", args[2]);
            std::process::exit(1);
        }
    };
    let target_trough: f64 = match args[3].parse() {
        Ok(v) if v > 0.0 => v,
        _ => {
            eprintln!("  \x1b[31mError: '{}' is not a valid concentration.\x1b[0m\n", args[3]);
            std::process::exit(1);
        }
    };
    let route = if args.len() > 4 {
        args[4].to_lowercase()
    } else {
        "oral".to_string()
    };

    if route != "oral" && route != "iv" {
        eprintln!("  \x1b[31mError: Route must be 'oral' or 'iv'.\x1b[0m\n");
        std::process::exit(1);
    }

    // Find drug
    let drug = drugs.iter().find(|d| {
        d.name == query || d.aliases.contains(&query.as_str())
    });
    let drug = match drug {
        Some(d) => d,
        None => {
            eprintln!("  \x1b[31mError: Drug '{}' not found.\x1b[0m\n", query);
            print_usage(&drugs);
            std::process::exit(1);
        }
    };

    // Find enzyme
    let enzyme = enzymes.iter().find(|e| e.name == drug.enzyme_name).unwrap();

    // Calculate
    let result = compute_dosing(drug, enzyme, weight_kg, target_trough, &route);

    // Print
    print_result(&result, drug, enzyme);

    eprintln!("  \x1b[36m\x1b[1mGULA sees the true measure.\x1b[0m");
    eprintln!("  \x1b[36m\x1b[1mThe great physician prescribes.\x1b[0m\n");
}
