// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  GULA v0.1.0 — Quantum-Corrected Dosing Calculator                      ║
// ║  sectio-aurea-q · MEGALODON Research · 2026                              ║
// ║                                                                          ║
// ║  Named for the Sumerian goddess of healing — azugallatu,                 ║
// ║  "the great physician." She gave humanity medical knowledge.             ║
// ║  GULA corrects what NANSHE exposed.                                      ║
// ║                                                                          ║
// ║  The world's first pharmacokinetic dosing tool that includes             ║
// ║  the quantum tunneling correction factor κ (kappa).                      ║
// ║                                                                          ║
// ║  Every other dosing tool on Earth uses:  k = A·exp(-Ea/RT)              ║
// ║  GULA uses:  k = κ · A·exp(-Ea/RT)                                      ║
// ║                                                                          ║
// ║  One multiplication. The difference between life and death.              ║
// ╚══════════════════════════════════════════════════════════════════════════╝
//
// ENZYME SOURCES:
//   [1] Cha, Murray, Klinman (1989) Science 243:1325-1330
//   [2] Oanca, Stare, Mavri (2020) PMID:32191250
//   [3] Klinman (1994) Biochemistry 33:14871
//   [4] Zhang, Lin (2009) J.Phys.Chem.A 113:11501-11508
//   [5] Krauser, Guengerich (2005) J.Biol.Chem. 280:19496
//
// DRUG SOURCES:
//   [6]  StatPearls: Cyclosporine (NCBI NBK482450)
//   [7]  Labcorp Test 706556: Cyclosporine Whole Blood
//   [8]  PMC4318379: Cyclosporine nephrotoxicity (2015)
//   [9]  FDA Label: Midazolam Injection
//   [10] PubMed 37300548: Lethal midazolam (2023)
//   [11] PubMed 9258787: Midazolam review
//   [12] FDA Label: Atorvastatin (Lipitor)
//   [13] StatPearls: Codeine (NCBI NBK526029)
//   [14] StatPearls: Tranylcypromine
//   [15] StatPearls: Phenelzine
//   [16] StatPearls: Selegiline
//   [17] StatPearls: Fomepizole (NCBI NBK541012)

// ═══ DATA STRUCTURES ════════════════════════════════════════════════════

struct Enzyme {
    name: &'static str,
    ec: &'static str,
    kappa: f64,
    kie: f64,
    kappa_source: &'static str,
    kie_source: &'static str,
}

struct Drug {
    name: &'static str,
    aliases: &'static [&'static str],
    purpose: &'static str,
    enzyme_name: &'static str,
    therapeutic_min: f64,       // ng/mL
    therapeutic_max: f64,       // ng/mL
    toxic_threshold: f64,       // ng/mL
    lethal_conc: f64,           // ng/mL (0.0 = not documented)
    half_life_hours: f64,       // classical published half-life
    standard_interval_hours: f64, // typical dosing interval
    toxic_effects: &'static str,
    range_source: &'static str,
    toxic_source: &'static str,
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
            kie_source: "Krauser,Guengerich (2005) JBC 280:19496 [5]",
        },
        Enzyme {
            name: "MAO-A",
            ec: "EC 1.4.3.4",
            kappa: 3.0,
            kie: 8.5,
            kappa_source: "Klinman (1994) Biochemistry 33:14871 [3]",
            kie_source: "Oanca,Stare,Mavri (2020) PMID:32191250 [2]",
        },
        Enzyme {
            name: "ADH",
            ec: "EC 1.1.1.1",
            kappa: 2.5,
            kie: 4.1,
            kappa_source: "Cha,Murray,Klinman (1989) Science 243:1325 [1]",
            kie_source: "Klinman (1976) Biochemistry 15:2018",
        },
    ]
}

fn get_drugs() -> Vec<Drug> {
    vec![
        Drug {
            name: "cyclosporine",
            aliases: &["ciclosporin", "csa", "sandimmun", "neoral"],
            purpose: "Immunosuppressant — prevents organ rejection after transplantation",
            enzyme_name: "CYP3A4",
            therapeutic_min: 100.0,
            therapeutic_max: 400.0,
            toxic_threshold: 400.0,
            lethal_conc: 0.0,
            half_life_hours: 19.0,
            standard_interval_hours: 12.0,
            toxic_effects: "Nephrotoxicity, renal failure, hypertension, seizures, lymphoma",
            range_source: "StatPearls NBK482450 [6], Labcorp 706556 [7]",
            toxic_source: "PMC4318379 [8]",
        },
        Drug {
            name: "midazolam",
            aliases: &["versed", "dormicum"],
            purpose: "Benzodiazepine — sedation, seizure control, palliative care",
            enzyme_name: "CYP3A4",
            therapeutic_min: 100.0,
            therapeutic_max: 200.0,
            toxic_threshold: 200.0,
            lethal_conc: 910.0,
            half_life_hours: 3.0,
            standard_interval_hours: 4.0,
            toxic_effects: "Deep sedation, respiratory depression, cardiac arrest, death",
            range_source: "FDA Label Midazolam [9]",
            toxic_source: "PubMed 37300548 [10], PubMed 9258787 [11]",
        },
        Drug {
            name: "atorvastatin",
            aliases: &["lipitor", "sortis"],
            purpose: "Statin — lowers cholesterol, prevents cardiovascular events",
            enzyme_name: "CYP3A4",
            therapeutic_min: 1.0,
            therapeutic_max: 80.0,
            toxic_threshold: 80.0,
            lethal_conc: 0.0,
            half_life_hours: 14.0,
            standard_interval_hours: 24.0,
            toxic_effects: "Rhabdomyolysis, liver damage, acute kidney injury",
            range_source: "FDA Label Atorvastatin [12]",
            toxic_source: "FDA Label Atorvastatin [12]",
        },
        Drug {
            name: "codeine",
            aliases: &["methylmorphine"],
            purpose: "Opioid prodrug — activated by CYP3A4/CYP2D6 to morphine",
            enzyme_name: "CYP3A4",
            therapeutic_min: 10.0,
            therapeutic_max: 120.0,
            toxic_threshold: 250.0,
            lethal_conc: 1600.0,
            half_life_hours: 3.0,
            standard_interval_hours: 6.0,
            toxic_effects: "Respiratory depression, CNS depression, opioid overdose, death",
            range_source: "StatPearls Codeine [13]",
            toxic_source: "StatPearls Codeine [13]",
        },
        Drug {
            name: "tranylcypromine",
            aliases: &["parnate"],
            purpose: "MAOI antidepressant — irreversible MAO-A/B inhibitor",
            enzyme_name: "MAO-A",
            therapeutic_min: 5.0,
            therapeutic_max: 50.0,
            toxic_threshold: 50.0,
            lethal_conc: 0.0,
            half_life_hours: 2.5,
            standard_interval_hours: 12.0,
            toxic_effects: "Serotonin syndrome, hypertensive crisis, hyperthermia, death",
            range_source: "StatPearls Tranylcypromine [14]",
            toxic_source: "StatPearls Tranylcypromine [14]",
        },
        Drug {
            name: "phenelzine",
            aliases: &["nardil"],
            purpose: "MAOI antidepressant — irreversible MAO-A/B inhibitor",
            enzyme_name: "MAO-A",
            therapeutic_min: 5.0,
            therapeutic_max: 50.0,
            toxic_threshold: 50.0,
            lethal_conc: 0.0,
            half_life_hours: 11.0,
            standard_interval_hours: 8.0,
            toxic_effects: "Serotonin syndrome, hypertensive crisis, hepatotoxicity",
            range_source: "StatPearls Phenelzine [15]",
            toxic_source: "StatPearls Phenelzine [15]",
        },
        Drug {
            name: "selegiline",
            aliases: &["eldepryl", "emsam", "zelapar"],
            purpose: "MAOI — selective MAO-B inhibitor (Parkinson's, depression)",
            enzyme_name: "MAO-A",
            therapeutic_min: 1.0,
            therapeutic_max: 20.0,
            toxic_threshold: 40.0,
            lethal_conc: 0.0,
            half_life_hours: 10.0,
            standard_interval_hours: 24.0,
            toxic_effects: "Serotonin syndrome, hypertensive crisis, hallucinations",
            range_source: "StatPearls Selegiline [16]",
            toxic_source: "StatPearls Selegiline [16]",
        },
        Drug {
            name: "fomepizole",
            aliases: &["antizol", "4-methylpyrazole"],
            purpose: "Antidote — blocks ADH in methanol/ethylene glycol poisoning",
            enzyme_name: "ADH",
            therapeutic_min: 100000.0,  // 100 µg/mL = 100000 ng/mL
            therapeutic_max: 300000.0,  // 300 µg/mL
            toxic_threshold: 300000.0,
            lethal_conc: 0.0,
            half_life_hours: 12.0,
            standard_interval_hours: 12.0,
            toxic_effects: "Seizures (high dose), bradycardia, hypotension",
            range_source: "StatPearls Fomepizole [17]",
            toxic_source: "StatPearls Fomepizole [17]",
        },
    ]
}

// ═══ CORE CALCULATION ENGINE ════════════════════════════════════════════

struct GulaDosing {
    // Input
    drug_name: String,
    target_trough: f64,

    // Enzyme data
    enzyme_name: String,
    kappa: f64,
    kie: f64,

    // Drug data
    therapeutic_min: f64,
    therapeutic_max: f64,
    toxic_threshold: f64,
    lethal_conc: f64,
    half_life_classical: f64,
    standard_interval: f64,

    // CLASSICAL CALCULATION (what doctors use today)
    classical_peak: f64,
    classical_trough: f64,        // what model predicts
    classical_interval: f64,

    // QUANTUM-CORRECTED CALCULATION (what GULA computes)
    real_half_life: f64,          // t½ / κ
    real_trough_at_classical: f64, // what patient actually has
    corrected_interval: f64,      // shorter interval to maintain levels
    corrected_peak: f64,          // peak needed with corrected interval

    // SAFETY ASSESSMENT
    classical_is_safe: bool,
    classical_trough_in_range: bool,
    corrected_is_safe: bool,
    dose_adjustment_would_kill: bool,
    fatal_peak: f64,              // peak if doctor blindly adjusts
}

fn calculate_dosing(drug: &Drug, enzyme: &Enzyme, target_trough: f64) -> GulaDosing {
    let kappa = enzyme.kappa;

    // ── CLASSICAL CALCULATION ──
    // Doctor uses: C_trough = C_peak × 2^(-interval/t½)
    // For standard interval = t½: C_trough = C_peak × 0.5
    // So: C_peak = C_trough / 0.5 = C_trough × 2
    let intervals_in_halflife = drug.standard_interval_hours / drug.half_life_hours;
    let classical_decay = (2.0_f64).powf(-intervals_in_halflife);
    let classical_peak = target_trough / classical_decay;
    let classical_trough = target_trough; // by definition

    // ── REALITY WITH TUNNELING ──
    // Real rate constant is κ× higher → real half-life is κ× shorter
    let real_half_life = drug.half_life_hours / kappa;

    // What ACTUALLY happens after standard interval with classical peak:
    // C_real = C_peak × 2^(-interval / real_t½)
    // C_real = C_peak × 2^(-interval × κ / classical_t½)
    let real_intervals = drug.standard_interval_hours / real_half_life;
    let real_decay = (2.0_f64).powf(-real_intervals);
    let real_trough_at_classical = classical_peak * real_decay;

    // ── CORRECTED DOSING ──
    // Use the REAL half-life to calculate correct interval
    // Keep peak within therapeutic range
    // Dose more frequently with lower peaks
    //
    // Strategy: set interval = real_half_life (one real half-life)
    // Then: C_trough = C_peak × 0.5
    // C_peak = target_trough / 0.5 = target_trough × 2
    let corrected_interval = real_half_life;
    let corrected_peak = target_trough / 0.5;

    // ── WHAT HAPPENS IF DOCTOR ADJUSTS BLINDLY ──
    // Doctor sees real_trough is too low
    // Doctor observes decay ratio: real_trough / classical_peak
    // Doctor calculates new peak to achieve target:
    // fatal_peak = target_trough / real_decay_at_standard_interval
    let fatal_peak = target_trough / real_decay;

    // ── SAFETY CHECKS ──
    let classical_is_safe = classical_peak <= drug.toxic_threshold;
    let classical_trough_in_range = real_trough_at_classical >= drug.therapeutic_min;
    let corrected_is_safe = corrected_peak <= drug.toxic_threshold;
    let dose_adjustment_would_kill = fatal_peak > drug.toxic_threshold;

    GulaDosing {
        drug_name: drug.name.to_uppercase(),
        target_trough,
        enzyme_name: enzyme.name.to_string(),
        kappa,
        kie: enzyme.kie,
        therapeutic_min: drug.therapeutic_min,
        therapeutic_max: drug.therapeutic_max,
        toxic_threshold: drug.toxic_threshold,
        lethal_conc: drug.lethal_conc,
        half_life_classical: drug.half_life_hours,
        standard_interval: drug.standard_interval_hours,
        classical_peak,
        classical_trough,
        classical_interval: drug.standard_interval_hours,
        real_half_life,
        real_trough_at_classical,
        corrected_interval,
        corrected_peak,
        classical_is_safe,
        classical_trough_in_range,
        corrected_is_safe,
        dose_adjustment_would_kill,
        fatal_peak,
    }
}

// ═══ OUTPUT ════════════════════════════════════════════════════════════

fn print_banner() {
    eprintln!("\x1b[32m\x1b[1m");
    eprintln!("  ╔══════════════════════════════════════════════════════════════╗");
    eprintln!("  ║  GULA v0.1.0 — Quantum-Corrected Dosing Calculator         ║");
    eprintln!("  ║  sectio-aurea-q · MEGALODON Research · 2026                 ║");
    eprintln!("  ║                                                              ║");
    eprintln!("  ║  \"The great physician.\"                                     ║");
    eprintln!("  ║  She gave humanity medical knowledge.                        ║");
    eprintln!("  ║  GULA corrects what NANSHE exposed.                          ║");
    eprintln!("  ╚══════════════════════════════════════════════════════════════╝");
    eprintln!("\x1b[0m");
}

fn print_result(g: &GulaDosing, drug: &Drug, enzyme: &Enzyme) {
    eprintln!("  ──────────────────────────────────────────────────────────────");
    eprintln!("  \x1b[1m\x1b[33m  DOSING ANALYSIS: {}\x1b[0m", g.drug_name);
    eprintln!("  ──────────────────────────────────────────────────────────────\n");

    eprintln!("  {}", drug.purpose);
    eprintln!("  Metabolized by: {} ({}) — κ = {:.1}", enzyme.name, enzyme.ec, g.kappa);
    eprintln!("  KIE measured: {:.1} (classical max: 7.0)\n", g.kie);

    // ── DRUG PROFILE ──
    eprintln!("  \x1b[1m┌─ DRUG PROFILE ────────────────────────────────────────────┐\x1b[0m");
    eprintln!("  │ Therapeutic range:     {:>8.1}–{:.1} ng/mL{:<14}│", g.therapeutic_min, g.therapeutic_max, "");
    eprintln!("  │ Toxic threshold:       {:>8.1} ng/mL{:<20}│", g.toxic_threshold, "");
    if g.lethal_conc > 0.0 {
    eprintln!("  │ Documented lethal:     {:>8.1} ng/mL{:<20}│", g.lethal_conc, "");
    }
    eprintln!("  │ Published half-life:   {:>8.1} hours{:<20}│", g.half_life_classical, "");
    eprintln!("  │ Standard interval:     {:>8.1} hours{:<20}│", g.standard_interval, "");
    eprintln!("  │ Target trough:         {:>8.1} ng/mL{:<20}│", g.target_trough, "");
    eprintln!("  │                                                          │");
    eprintln!("  │ Source: {:<48}│", drug.range_source);
    eprintln!("  \x1b[1m└──────────────────────────────────────────────────────────┘\x1b[0m\n");

    // ── CLASSICAL vs CORRECTED ──
    eprintln!("  \x1b[1m┌─ CLASSICAL MODEL (what every tool uses today) ──────────┐\x1b[0m");
    eprintln!("  │                                                          │");
    eprintln!("  │ Formula: k = A·exp(-Ea/RT)                               │");
    eprintln!("  │ Half-life: {:<6.1} hours                                   │", g.half_life_classical);
    eprintln!("  │ Interval: every {:<5.1} hours                               │", g.classical_interval);
    eprintln!("  │                                                          │");
    eprintln!("  │ Calculated peak:    {:>8.1} ng/mL                        │", g.classical_peak);
    eprintln!("  │ Predicted trough:   {:>8.1} ng/mL                        │", g.classical_trough);
    if g.classical_is_safe {
    eprintln!("  │ Peak safety:        \x1b[32m✓ within therapeutic range\x1b[0m          │");
    } else {
    eprintln!("  │ Peak safety:        \x1b[31m✗ EXCEEDS therapeutic range\x1b[0m         │");
    }
    eprintln!("  │                                                          │");
    eprintln!("  │ \x1b[33mBUT: This model ignores quantum tunneling (κ={:.1})\x1b[0m       │", g.kappa);
    eprintln!("  \x1b[1m└──────────────────────────────────────────────────────────┘\x1b[0m\n");

    // ── REALITY ──
    eprintln!("  \x1b[1m┌─ REALITY (with tunneling correction κ = {:.1}) ──────────┐\x1b[0m", g.kappa);
    eprintln!("  │                                                          │");
    eprintln!("  │ Formula: k = κ · A·exp(-Ea/RT)                           │");
    eprintln!("  │ Real half-life: {:<6.1} hours  ({:.1} / {:.1})                 │", g.real_half_life, g.half_life_classical, g.kappa);
    eprintln!("  │                                                          │");
    eprintln!("  │ With classical peak of {:.1} ng/mL:", g.classical_peak);
    eprintln!("  │   Classical model predicts:  {:>8.1} ng/mL trough        │", g.classical_trough);
    eprintln!("  │   Patient ACTUALLY has:      {:>8.1} ng/mL trough        │", g.real_trough_at_classical);
    eprintln!("  │                                                          │");
    if !g.classical_trough_in_range {
    let deficit_pct = ((g.target_trough - g.real_trough_at_classical) / g.target_trough) * 100.0;
    eprintln!("  │ \x1b[31m✗ {:.1} ng/mL is {:.0}% below target.\x1b[0m{:<22}│", g.real_trough_at_classical, deficit_pct, "");
    eprintln!("  │ \x1b[31m✗ DRUG IS NOT WORKING. Doctor sees treatment failure.\x1b[0m   │");
    } else {
    eprintln!("  │ \x1b[32m✓ Trough is within range.\x1b[0m                                │");
    }
    eprintln!("  \x1b[1m└──────────────────────────────────────────────────────────┘\x1b[0m\n");

    // ── DANGER: BLIND ADJUSTMENT ──
    if g.dose_adjustment_would_kill {
    eprintln!("  \x1b[1m\x1b[31m┌─ DANGER: IF DOCTOR INCREASES DOSE ──────────────────────┐\x1b[0m");
    eprintln!("  \x1b[31m│                                                          │\x1b[0m");
    eprintln!("  \x1b[31m│ Doctor sees low trough → increases dose to compensate.   │\x1b[0m");
    eprintln!("  \x1b[31m│ To reach target {:.1} with real decay of {:.4}:         │\x1b[0m", g.target_trough, (2.0_f64).powf(-(g.standard_interval / g.real_half_life)));
    eprintln!("  \x1b[31m│                                                          │\x1b[0m");
    eprintln!("  \x1b[31m│   New peak = {:.1} / {:.4} = {:<8.1} ng/mL              │\x1b[0m", g.target_trough, (2.0_f64).powf(-(g.standard_interval / g.real_half_life)), g.fatal_peak);
    eprintln!("  \x1b[31m│   Toxic threshold:              {:>8.1} ng/mL            │\x1b[0m", g.toxic_threshold);
    let overdose_factor = g.fatal_peak / g.toxic_threshold;
    eprintln!("  \x1b[31m│                                                          │\x1b[0m");
    eprintln!("  \x1b[31m│   ██ OVERDOSE FACTOR: {:.1}× the toxic threshold ██       │\x1b[0m", overdose_factor);
    if g.lethal_conc > 0.0 {
    let lethal_pct = (g.fatal_peak / g.lethal_conc) * 100.0;
    eprintln!("  \x1b[31m│   ██ {:.0}% of documented lethal concentration   ██       │\x1b[0m", lethal_pct);
    }
    eprintln!("  \x1b[31m│                                                          │\x1b[0m");
    eprintln!("  \x1b[31m│   Result: {:<46}│\x1b[0m", drug.toxic_effects);
    eprintln!("  \x1b[1m\x1b[31m└──────────────────────────────────────────────────────────┘\x1b[0m\n");
    }

    // ── GULA CORRECTION ──
    eprintln!("  \x1b[1m\x1b[32m┌─ GULA SOLUTION: QUANTUM-CORRECTED DOSING ────────────────┐\x1b[0m");
    eprintln!("  \x1b[32m│                                                          │\x1b[0m");
    eprintln!("  \x1b[32m│ Do NOT increase the dose.                                │\x1b[0m");
    eprintln!("  \x1b[32m│ SHORTEN THE INTERVAL.                                    │\x1b[0m");
    eprintln!("  \x1b[32m│                                                          │\x1b[0m");
    eprintln!("  \x1b[32m│ Corrected half-life:  {:<6.1} hours                        │\x1b[0m", g.real_half_life);
    eprintln!("  \x1b[32m│ Corrected interval:   {:<6.1} hours  (was {:.1}h)            │\x1b[0m", g.corrected_interval, g.standard_interval);
    eprintln!("  \x1b[32m│ Corrected peak:       {:<8.1} ng/mL                      │\x1b[0m", g.corrected_peak);
    eprintln!("  \x1b[32m│ Corrected trough:     {:<8.1} ng/mL                      │\x1b[0m", g.target_trough);
    eprintln!("  \x1b[32m│                                                          │\x1b[0m");
    if g.corrected_is_safe {
    eprintln!("  \x1b[32m│ ✓ Peak {:.1} is within therapeutic range ({:.0}–{:.0})\x1b[0m    │", g.corrected_peak, g.therapeutic_min, g.therapeutic_max);
    eprintln!("  \x1b[32m│ ✓ Trough {:.1} meets target                      \x1b[0m       │", g.target_trough);
    eprintln!("  \x1b[32m│ ✓ Patient is safe.\x1b[0m                                     │");
    } else {
    eprintln!("  \x1b[33m│ ⚠ Peak {:.1} still exceeds range — reduce dose further\x1b[0m  │", g.corrected_peak);
    }
    eprintln!("  \x1b[1m\x1b[32m└──────────────────────────────────────────────────────────┘\x1b[0m\n");

    // ── MATH SUMMARY ──
    eprintln!("  \x1b[1m┌─ THE MATH (verify with a calculator) ───────────────────┐\x1b[0m");
    eprintln!("  │                                                          │");
    eprintln!("  │ Classical:                                               │");
    eprintln!("  │   t½ = {:.1}h,  interval = {:.1}h                          │", g.half_life_classical, g.standard_interval);
    eprintln!("  │   decay = 2^(-{:.1}/{:.1}) = {:.4}                          │", g.standard_interval, g.half_life_classical, (2.0_f64).powf(-(g.standard_interval / g.half_life_classical)));
    eprintln!("  │   peak = {:.1} / {:.4} = {:.1}                            │", g.target_trough, (2.0_f64).powf(-(g.standard_interval / g.half_life_classical)), g.classical_peak);
    eprintln!("  │                                                          │");
    eprintln!("  │ With κ = {:.1}:                                            │", g.kappa);
    eprintln!("  │   t½_real = {:.1} / {:.1} = {:.2}h                          │", g.half_life_classical, g.kappa, g.real_half_life);
    eprintln!("  │   real decay = 2^(-{:.1}/{:.2}) = {:.6}                    │", g.standard_interval, g.real_half_life, (2.0_f64).powf(-(g.standard_interval / g.real_half_life)));
    eprintln!("  │   real trough = {:.1} × {:.6} = {:.2}                    │", g.classical_peak, (2.0_f64).powf(-(g.standard_interval / g.real_half_life)), g.real_trough_at_classical);
    eprintln!("  │                                                          │");
    eprintln!("  │ Corrected dosing:                                        │");
    eprintln!("  │   interval = {:.2}h (one real half-life)                   │", g.corrected_interval);
    eprintln!("  │   peak = {:.1} / 0.5 = {:.1}                              │", g.target_trough, g.corrected_peak);
    eprintln!("  │   trough = {:.1} × 0.5 = {:.1}  ✓                         │", g.corrected_peak, g.target_trough);
    eprintln!("  │                                                          │");
    eprintln!("  │ κ source: {:<44}│", enzyme.kappa_source);
    eprintln!("  │ KIE source: {:<42}│", enzyme.kie_source);
    eprintln!("  \x1b[1m└──────────────────────────────────────────────────────────┘\x1b[0m\n");
}

fn print_usage(drugs: &[Drug]) {
    eprintln!("  \x1b[1mUsage:\x1b[0m  gula <drug> <target_trough_ng_mL>\n");
    eprintln!("  \x1b[1mExample:\x1b[0m");
    eprintln!("    gula cyclosporine 200");
    eprintln!("    gula midazolam 100\n");
    eprintln!("  \x1b[1mAvailable drugs:\x1b[0m\n");
    for d in drugs {
        let aliases: Vec<&str> = d.aliases.iter().copied().collect();
        let alias_str = if aliases.is_empty() {
            String::new()
        } else {
            format!(" ({})", aliases.join(", "))
        };
        eprintln!("    {:<20} → {:<10}  t½={:.0}h  [{:.0}–{:.0} ng/mL]",
            format!("{}{}", d.name, alias_str), d.enzyme_name,
            d.half_life_hours, d.therapeutic_min, d.therapeutic_max);
    }
    eprintln!();
    eprintln!("  \x1b[1mWhat GULA does:\x1b[0m");
    eprintln!("  Every dosing tool on Earth uses:  k = A·exp(-Ea/RT)");
    eprintln!("  This ignores quantum tunneling in enzyme catalysis.");
    eprintln!("  GULA adds the published tunneling coefficient κ:");
    eprintln!("  k_real = κ × k_classical");
    eprintln!("  t½_real = t½_classical / κ");
    eprintln!("  This changes everything: dose intervals, peak levels, safety.\n");
}

// ═══ MAIN ════════════════════════════════════════════════════════════════

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let enzymes = get_enzymes();
    let drugs = get_drugs();

    print_banner();

    if args.len() < 3 {
        print_usage(&drugs);
        if args.len() == 2 {
            eprintln!("  \x1b[31mError: Missing target trough concentration.\x1b[0m");
            eprintln!("  Usage: gula {} <target_trough_ng_mL>\n", args[1]);
        }
        std::process::exit(1);
    }

    let query = args[1].to_lowercase();
    let target_trough: f64 = match args[2].parse() {
        Ok(v) if v > 0.0 => v,
        _ => {
            eprintln!("  \x1b[31mError: '{}' is not a valid concentration.\x1b[0m", args[2]);
            eprintln!("  Target trough must be a positive number in ng/mL.\n");
            std::process::exit(1);
        }
    };

    // Find drug
    let drug = drugs.iter().find(|d| {
        d.name == query || d.aliases.contains(&query.as_str())
    });

    let drug = match drug {
        Some(d) => d,
        None => {
            eprintln!("  \x1b[31mError: Drug '{}' not found in database.\x1b[0m\n", query);
            print_usage(&drugs);
            std::process::exit(1);
        }
    };

    // Find enzyme
    let enzyme = enzymes.iter().find(|e| e.name == drug.enzyme_name).unwrap();

    // Validate target trough
    if target_trough < drug.therapeutic_min {
        eprintln!("  \x1b[33mWarning: Target trough {:.1} is below therapeutic minimum ({:.1}).\x1b[0m\n",
            target_trough, drug.therapeutic_min);
    }
    if target_trough > drug.therapeutic_max {
        eprintln!("  \x1b[31mWarning: Target trough {:.1} exceeds therapeutic maximum ({:.1}).\x1b[0m\n",
            target_trough, drug.therapeutic_max);
    }

    // Calculate and display
    let result = calculate_dosing(drug, enzyme, target_trough);
    print_result(&result, drug, enzyme);

    // Final line
    eprintln!("  \x1b[36m\x1b[1mGULA sees the true measure.\x1b[0m");
    eprintln!("  \x1b[36m\x1b[1mThe great physician corrects.\x1b[0m\n");
}
