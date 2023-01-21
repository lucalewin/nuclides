
// include!(concat!(env!("OUT_DIR"), "/ground_states.rs"));

pub struct Nuclide {
    pub z: u8,
    pub n: u8,
    pub symbol: &'static str,
    pub decay_1: Decay,
    pub decay_2: Option<Decay>,
    pub decay_3: Option<Decay>
}

pub struct Decay {
    pub mode: DecayMode,
    pub probability: f64,
    pub uncertainty: f64
}

pub enum DecayMode {
    AlphaEmission,
    ProtonEmission,
    DoubleProtonEmission,
    NeutronEmission,
    DoubleNeutronEmission,
    // TODO: add more decay modes
    Stable
}
