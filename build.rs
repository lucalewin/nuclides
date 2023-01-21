#![feature(slice_group_by)]

use std::{env, error::Error, fs::File, io::Write, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(unused)]
struct Nuclide {
    z: u8,
    n: u8,
    symbol: String,
    radius: Option<String>,

    #[serde(alias = "unc_r")]
    radius_uncertainty: Option<String>,
    
    abundance: Option<String>,
    
    #[serde(alias = "unc_a")]
    abundance_uncertainty: Option<String>,
    energy_shift: Option<String>,
    energy: Option<String>,
    
    #[serde(alias = "unc_e")]
    energy_uncertainty: Option<String>,
    ripl_shift: Option<String>,
    jp: Option<String>,
    half_life: Option<String>,
    operator_hl: Option<String>,
    unc_hl: Option<String>,
    unit_hl: Option<String>,
    half_life_sec: Option<String>,
    unc_hls: Option<String>,
    decay_1: Option<String>,
    decay_1_percent: Option<String>,
    decay_1_uncertainty: Option<String>,
    decay_2: Option<String>,
    decay_2_percent: Option<String>,
    decay_2_uncertainty: Option<String>,
    decay_3: Option<String>,
    decay_3_percent: Option<String>,
    decay_3_uncertainty: Option<String>,
    isospin: Option<String>,
    magnetic_dipole: Option<String>,
    magnetic_dipole_uncertainty: Option<String>,
    electric_quadrupole: Option<String>,
    electric_quadrupole_uncertainty: Option<String>,
    qbm: Option<String>,
    unc_qb: Option<String>,
    qbm_n: Option<String>,
    unc_qbmn: Option<String>,
    qa: Option<String>,
    unc_qa: Option<String>,
    qec: Option<String>,
    unc_qec: Option<String>,
    sn: Option<String>,
    unc_sn: Option<String>,
    sp: Option<String>,
    unc_sp: Option<String>,
    binding: Option<String>,
    unc_ba: Option<String>,

    atomic_mass: Option<String>,

    #[serde(alias = "unc_am")]
    atomic_mass_uncertainty: Option<String>,
    
    #[serde(alias = "massexcess")]
    mass_excess: Option<String>,

    #[serde(alias = "unc_me")]
    mass_excess_uncertainty: Option<String>,
    // ensdf_publicationcut_off: Option<String>,
    // ensdf_authors: Option<String>,
    // extraction_date: Option<String>
}

enum DecayMode {
    Alpha,
    BetaPlusOrElectronCapture,
    BetaMinus,
    Proton,
    Neutron,
    SpontaneousFission,
    Stable
}

fn main() -> Result<(), Box<dyn Error>> {
    // let out_dir = env::var("OUT_DIR")?;
    // let ground_states_path = Path::new(&out_dir).join("ground_states.rs");

    // let mut reader = csv::ReaderBuilder::new()
    //     .trim(csv::Trim::All)
    //     .from_path("./data/ground_states_all_nuclides.csv")?;

    // let mut file = File::create(ground_states_path)?;

    // let mut nuclides = reader.deserialize().collect::<Result<Vec<Nuclide>, _>>()?;

    // let mut highest_n = 0;
    // let mut highest_z = 0;

    // for nuclide in &nuclides {
    //     if nuclide.n > highest_n {
    //         highest_n = nuclide.n;
    //     }
    //     if nuclide.z > highest_z {
    //         highest_z = nuclide.z;
    //     }
    //     writeln!(
    //         file,
    //         "pub const {}{}: Nuclide = {:?};",
    //         nuclide.symbol.to_uppercase(),
    //         nuclide.z as u32 + nuclide.n as u32,
    //         nuclide 
    //     )?;
    // }

    // writeln!(
    //     file,
    //     "pub const TABLE_OF_NUCLIDES: [[Option<Nuclide>; {}]; {}]",
    //     highest_z, // number of protons  | y-axis
    //     highest_n, // number of neutrons | x-axis 
    // )?;

    // nuclides.sort_by(|a, b| {
    //     a.n.partial_cmp(&b.n).unwrap()
    // });

    // for result in reader.deserialize() {
    //     let nuclide: Nuclide = result?;

    //     writeln!(
    //         file,
    //         "pub const {}{}: Nuclide = {:?};",
    //         nuclide.symbol.to_uppercase(),
    //         nuclide.z as u32 + nuclide.n as u32,
    //         nuclide 
    //     )?;
    // }
    
    // writeln!(file, "
    // #[allow(unused)]
    // pub struct Nuclide {{
    //     z: u8,
    //     n: u8,
    //     symbol: &'static str,
    //     radius: Option<&'static str>,
    //     unc_r: Option<&'static str>,
    //     abundance: Option<&'static str>,
    //     abundance_unc: Option<&'static str>,
    //     energy_shift: Option<&'static str>,
    //     energy: Option<&'static str>,
    //     unc_e: Option<&'static str>,
    //     ripl_shift: Option<&'static str>,
    //     jp: Option<&'static str>,
    //     half_life: Option<&'static str>,
    //     operator_hl: Option<&'static str>,
    //     unc_hl: Option<&'static str>,
    //     unit_hl: Option<&'static str>,
    //     half_life_sec: Option<&'static str>,
    //     unc_hls: Option<&'static str>,
    //     decay_1: Option<&'static str>,
    //     decay_1_percent: Option<&'static str>,
    //     unc_1: Option<&'static str>,
    //     decay_2: Option<&'static str>,
    //     decay_2_percent: Option<&'static str>,
    //     unc_2: Option<&'static str>,
    //     decay_3: Option<&'static str>,
    //     decay_3_percent: Option<&'static str>,
    //     unc_3: Option<&'static str>,
    //     isospin: Option<&'static str>,
    //     magnetic_dipole: Option<&'static str>,
    //     unc_md: Option<&'static str>,
    //     electric_quadrupole: Option<&'static str>,
    //     unc_eq: Option<&'static str>,
    //     qbm: Option<&'static str>,
    //     unc_qb: Option<&'static str>,
    //     qbm_n: Option<&'static str>,
    //     unc_qbmn: Option<&'static str>,
    //     qa: Option<&'static str>,
    //     unc_qa: Option<&'static str>,
    //     qec: Option<&'static str>,
    //     unc_qec: Option<&'static str>,
    //     sn: Option<&'static str>,
    //     unc_sn: Option<&'static str>,
    //     sp: Option<&'static str>,
    //     unc_sp: Option<&'static str>,
    //     binding: Option<&'static str>,
    //     unc_ba: Option<&'static str>,
    //     atomic_mass: Option<&'static str>,
    //     unc_am: Option<&'static str>,
    //     massexcess: Option<&'static str>,
    //     unc_me: Option<&'static str>,
    //     ensdf_publicationcut_off: Option<&'static str>,
    //     ensdf_authors: Option<&'static str>,
    //     extraction_date: Option<&'static str>
    // }}")?;

    Ok(())
}