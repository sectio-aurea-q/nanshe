// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  NANSHE v0.1.0 — Quantum Dosing Error Calculator                        ║
// ║  sectio-aurea-q · MEGALODON Research · 2026                              ║
// ║                                                                          ║
// ║  "She who judges false measures."                                        ║
// ║                                                                          ║
// ║  Named for the Sumerian goddess who punished those who used              ║
// ║  false weights and calculations to harm the vulnerable.                  ║
// ║                                                                          ║
// ║  For Heike. Freunde bis zum Schluss.                                     ║
// ╚══════════════════════════════════════════════════════════════════════════╝
//
// ALL VALUES FROM PEER-REVIEWED LITERATURE — SEE NANSHE-QUELLEN.md
//
// References:
//   [1] Cha, Murray, Klinman (1989) Science 243:1325-1330
//   [2] Kohen, Cannio, Bartolucci, Klinman (1999) Nature 399:496-499
//   [3] Oanca, Stare, Mavri (2020) PMID:32191250
//   [4] Klinman (1994) Biochemistry 33:14871
//   [5] Zhang, Lin (2009) J.Phys.Chem.A 113:11501-11508
//   [6] Krauser, Guengerich (2005) J.Biol.Chem. 280:19496
//   [7] Rydberg, Ryde, Olsen (2006) J.Med.Chem. 49:6400

// ═══ ENZYME TARGET ══════════════════════════════════════════════════════

struct EnzymeTarget {
    name: &'static str,
    short: &'static str,
    ec: &'static str,
    kie_measured: f64,
    kie_source: &'static str,
    tunneling_coeff: f64,
    tunneling_source: &'static str,
    ea_kjmol: f64,
    ea_source: &'static str,
    drug_examples: &'static str,
    drug_class: &'static str,
    drugs_affected_pct: &'static str,
    scenario_under: &'static str,
    scenario_over: &'static str,
}

// ═══ DISPLAY ════════════════════════════════════════════════════════════

fn print_header() {
    eprintln!("\x1b[31m\x1b[1m");
    eprintln!("  ╔══════════════════════════════════════════════════════════════╗");
    eprintln!("  ║  NANSHE v0.1.0 — Quantum Dosing Error Calculator           ║");
    eprintln!("  ║  sectio-aurea-q · MEGALODON Research · 2026                 ║");
    eprintln!("  ║                                                              ║");
    eprintln!("  ║  \"She who judges false measures.\"                           ║");
    eprintln!("  ╚══════════════════════════════════════════════════════════════╝");
    eprintln!("\x1b[0m");
    eprintln!("  For Heike. Freunde bis zum Schluss.\n");
    eprintln!("  ──────────────────────────────────────────────────────────────");
    eprintln!("  THE PROBLEM");
    eprintln!("  ──────────────────────────────────────────────────────────────\n");
    eprintln!("  Pharmaceutical dosing models calculate enzyme reaction rates");
    eprintln!("  using classical transition state theory (Eyring/Arrhenius).");
    eprintln!("  This framework ignores quantum mechanical tunneling of hydrogen.\n");
    eprintln!("  When protons or hydrides tunnel through activation barriers,");
    eprintln!("  the REAL reaction rate is HIGHER than the classical prediction.");
    eprintln!("  The drug is metabolized FASTER than the model says.\n");
    eprintln!("  The classical maximum for H/D kinetic isotope effects is ~7.");
    eprintln!("  Values above 7 are IMPOSSIBLE without quantum tunneling.");
    eprintln!("  Multiple drug-metabolizing enzymes exceed this limit.\n");
    eprintln!("  This tool uses ONLY published, peer-reviewed experimental");
    eprintln!("  data to quantify the dosing error at each step.\n");
    eprintln!("  ──────────────────────────────────────────────────────────────\n");
}

fn print_enzyme(e: &EnzymeTarget) {
    let kappa = e.tunneling_coeff;

    eprintln!("  \x1b[1m\x1b[33m═══ {} ({}) ═══\x1b[0m", e.name, e.ec);
    eprintln!("  {}", e.drug_class);
    eprintln!("  Drugs affected: {}\n", e.drugs_affected_pct);

    // STEP 1: KIE evidence
    eprintln!("  \x1b[1mSTEP 1 — EXPERIMENTAL EVIDENCE: KINETIC ISOTOPE EFFECT\x1b[0m\n");
    eprintln!("  The KIE measures how much slower a reaction runs when H is");
    eprintln!("  replaced by deuterium (D). Classical physics sets a hard");
    eprintln!("  upper limit of ~7.0 for this ratio.\n");
    eprintln!("  ┌──────────────────────────────────────────────────────────┐");
    eprintln!("  │ Classical maximum KIE (H/D):       ~7.0                 │");
    eprintln!("  │ Measured KIE for this enzyme:      {:<5.1}                │", e.kie_measured);
    if e.kie_measured > 7.0 {
    eprintln!("  │                                                          │");
    eprintln!("  │ \x1b[1m\x1b[31m{:.1} > 7.0 — EXCEEDS CLASSICAL LIMIT\x1b[0m                     │", e.kie_measured);
    eprintln!("  │ \x1b[1m\x1b[31mImpossible without quantum tunneling.\x1b[0m                     │");
    } else {
    eprintln!("  │                                                          │");
    eprintln!("  │ \x1b[1m\x1b[33mAnomalous Swain-Schaad exponents + temperature\x1b[0m            │");
    eprintln!("  │ \x1b[1m\x1b[33mindependence confirm tunneling. [1]\x1b[0m                       │");
    }
    eprintln!("  │                                                          │");
    eprintln!("  │ Source: {}│", format!("{:<44}", e.kie_source));
    eprintln!("  └──────────────────────────────────────────────────────────┘\n");

    // STEP 2: Tunneling coefficient
    eprintln!("  \x1b[1mSTEP 2 — THE ERROR FACTOR: TUNNELING COEFFICIENT (κ)\x1b[0m\n");
    eprintln!("  κ tells us: the real rate is κ× FASTER than classical.\n");
    eprintln!("  ┌──────────────────────────────────────────────────────────┐");
    eprintln!("  │ k_real  =  k_classical  ×  κ                            │");
    eprintln!("  │                                                          │");
    eprintln!("  │ Published κ for this enzyme:  \x1b[1m\x1b[31m{:.1}×\x1b[0m                       │", kappa);
    eprintln!("  │                                                          │");
    eprintln!("  │ Every half-life calculated without κ is wrong.           │");
    eprintln!("  │ Every dose derived from that half-life is wrong.         │");
    eprintln!("  │                                                          │");
    eprintln!("  │ Source: {}│", format!("{:<44}", e.tunneling_source));
    eprintln!("  └──────────────────────────────────────────────────────────┘\n");

    // STEP 3a: UNDERDOSING
    // After one classical half-life, pharma expects 50% remaining.
    // Real: C = C0 * exp(-κ * k_cl * t_half_cl) = C0 * 2^(-κ)
    let actual_remaining = (2.0_f64).powf(-kappa) * 100.0;
    let underdose_pct = ((50.0 - actual_remaining) / 50.0) * 100.0;

    eprintln!("  \x1b[1mSTEP 3a — PATIENT HARM: UNDERDOSING\x1b[0m\n");
    eprintln!("  When this enzyme DEACTIVATES the drug:\n");
    eprintln!("  ┌──────────────────────────────────────────────────────────┐");
    eprintln!("  │ Drugs: {:<49}│", e.drug_examples);
    eprintln!("  │                                                          │");
    eprintln!("  │ After one classical dosing interval:                     │");
    eprintln!("  │   Model predicts:     50.0% drug remaining              │");
    eprintln!("  │   Reality (κ={:.1}):    {:>5.1}% drug remaining              │", kappa, actual_remaining);
    eprintln!("  │                                                          │");
    eprintln!("  │ \x1b[1m\x1b[31m→ Patient has {:.0}% LESS drug in blood than prescribed.\x1b[0m  │", underdose_pct);
    eprintln!("  │                                                          │");
    eprintln!("  │ {}│", format!("{:<56}", e.scenario_under));
    eprintln!("  └──────────────────────────────────────────────────────────┘\n");

    // STEP 3b: OVERDOSE
    eprintln!("  \x1b[1mSTEP 3b — PATIENT HARM: TOXICITY (PRODRUG ACTIVATION)\x1b[0m\n");
    eprintln!("  When this enzyme ACTIVATES a prodrug:\n");
    eprintln!("  ┌──────────────────────────────────────────────────────────┐");
    eprintln!("  │ Activation rate is {:.1}× faster than predicted.          │", kappa);
    eprintln!("  │ Peak metabolite concentration: {:.1}× higher.             │", kappa);
    eprintln!("  │                                                          │");
    eprintln!("  │ \x1b[1m\x1b[31m→ {:.1}× MORE active compound than expected.\x1b[0m               │", kappa);
    eprintln!("  │                                                          │");
    eprintln!("  │ {}│", format!("{:<56}", e.scenario_over));
    eprintln!("  └──────────────────────────────────────────────────────────┘\n");

    eprintln!("  ──────────────────────────────────────────────────────────────\n");
}

fn print_summary(enzymes: &[EnzymeTarget]) {
    eprintln!("  \x1b[1m\x1b[31m══════════════════════════════════════════════════════════════\x1b[0m");
    eprintln!("  \x1b[1m\x1b[31m  NANSHE VERDICT — THE COST OF IGNORING QUANTUM MECHANICS\x1b[0m");
    eprintln!("  \x1b[1m\x1b[31m══════════════════════════════════════════════════════════════\x1b[0m\n");

    eprintln!("  \x1b[1m{:<28} {:>6} {:>6} {:>10}\x1b[0m", "Enzyme", "KIE", "κ", "Underdose");
    eprintln!("  {}", "─".repeat(54));
    for e in enzymes {
        let actual = (2.0_f64).powf(-e.tunneling_coeff) * 100.0;
        let underdose = ((50.0 - actual) / 50.0) * 100.0;
        eprintln!("  {:<28} {:>5.1} {:>5.1}× {:>9.0}%",
            e.short, e.kie_measured, e.tunneling_coeff, underdose);
    }

    eprintln!("\n  \x1b[1mFACTS:\x1b[0m\n");
    eprintln!("  • Classical KIE limit: ~7. MAO-A measured: 8.5. CYP3A4: 15.");
    eprintln!("    These values PROVE quantum tunneling occurs.");
    eprintln!("  • CYP3A4 alone metabolizes ~50% of all oral drugs.");
    eprintln!("    Published tunneling coefficient: 3×.");
    eprintln!("  • The correction is one line of math: k_real = k_classical × κ.");
    eprintln!("  • Bell published the theory in 1980. Klinman proved it in 1989.");
    eprintln!("  • No clinical dosing model includes this correction.\n");

    eprintln!("  \x1b[1mPEER-REVIEWED SOURCES:\x1b[0m\n");
    eprintln!("  [1] Cha, Murray, Klinman — Science 243:1325-1330 (1989)");
    eprintln!("  [2] Kohen, Klinman — Nature 399:496-499 (1999)");
    eprintln!("  [3] Oanca, Stare, Mavri — PMID:32191250 (2020)");
    eprintln!("  [4] Klinman — Biochemistry 33:14871 (1994)");
    eprintln!("  [5] Zhang, Lin — J.Phys.Chem.A 113:11501 (2009)");
    eprintln!("  [6] Krauser, Guengerich — J.Biol.Chem. 280:19496 (2005)");
    eprintln!("  [7] Rydberg, Ryde, Olsen — J.Med.Chem. 49:6400 (2006)\n");

    eprintln!("  \x1b[36m\x1b[1mNANSHE sees the false measures.\x1b[0m");
    eprintln!("  \x1b[36m\x1b[1mNow you see them too.\x1b[0m\n");
}

// ═══ MAIN ════════════════════════════════════════════════════════════════

fn main() {
    print_header();

    let enzymes = vec![
        EnzymeTarget {
            name: "Alcohol Dehydrogenase",
            short: "ADH (EC 1.1.1.1)",
            ec: "EC 1.1.1.1",
            kie_measured: 4.1,
            kie_source: "Klinman (1976) Biochemistry 15:2018",
            tunneling_coeff: 2.5,
            tunneling_source: "Cha,Murray,Klinman (1989) Science 243:1325",
            ea_kjmol: 25.5,
            ea_source: "Silverstein et al., J.Chem.Educ.",
            drug_examples: "Fomepizole, Ethanol (drug interactions)",
            drug_class: "Oxidizes alcohols to aldehydes. Key in alcohol-drug interactions.",
            drugs_affected_pct: "All alcohol-metabolized drugs",
            scenario_under: "Antidote timing wrong in methanol/ethylene glycol poisoning.",
            scenario_over: "Acetaldehyde surge → liver toxicity, carcinogenesis.",
        },
        EnzymeTarget {
            name: "Monoamine Oxidase A",
            short: "MAO-A (EC 1.4.3.4)",
            ec: "EC 1.4.3.4",
            kie_measured: 8.5,
            kie_source: "Oanca,Stare,Mavri (2020) PMID:32191250",
            tunneling_coeff: 3.0,
            tunneling_source: "Klinman (1994) Biochemistry 33:14871",
            ea_kjmol: 55.0,
            ea_source: "QM/MM — Mavri et al.",
            drug_examples: "Tranylcypromine, Phenelzine, Selegiline",
            drug_class: "Decomposes serotonin, dopamine, norepinephrine. MAOI target.",
            drugs_affected_pct: "All MAO inhibitor antidepressants",
            scenario_under: "Antidepressant fails. Neurotransmitters cleared too fast.",
            scenario_over: "Serotonin syndrome — hyperthermia, seizures. Fatal.",
        },
        EnzymeTarget {
            name: "Cytochrome P450 3A4",
            short: "CYP3A4 (EC 1.14.14.1)",
            ec: "EC 1.14.14.1",
            kie_measured: 15.0,
            kie_source: "Krauser,Guengerich (2005) JBC 280:19496",
            tunneling_coeff: 3.0,
            tunneling_source: "Zhang,Lin (2009) JPC-A 113:11501",
            ea_kjmol: 61.0,
            ea_source: "Rydberg,Ryde,Olsen (2006) JMedChem 49:6400",
            drug_examples: "Atorvastatin, Cyclosporine, Midazolam, Codeine",
            drug_class: "Metabolizes ~50% of all oral drugs. Most important drug enzyme.",
            drugs_affected_pct: "~50% of ALL oral pharmaceuticals",
            scenario_under: "Statin failure → cardiovascular events. Lost immunosuppression.",
            scenario_over: "Cyclosporine toxicity → organ damage. Opioid overdose.",
        },
    ];

    for e in &enzymes {
        print_enzyme(e);
    }

    print_summary(&enzymes);
}
