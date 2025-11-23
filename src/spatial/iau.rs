/// IAU nominal conversion constants (2009/2015)
pub mod iau {
    /// Astronomical Unit (exact, IAU 2012 definition).
    pub const AU_METERS: f64 = 149_597_870_700.0;

    /// Light-year (derived from c = 299,792,458 m/s).
    pub const LY_METERS: f64 = 9.4607e15;

    /// Parsec (exact, derived from AU).
    pub const PARSEC_METERS: f64 = 3.085677581e16;

    /// Nominal Solar Radius (IAU 2015 Resolution B3).
    pub const R_SUN_METERS: f64 = 695_700_000.0;

    /// Nominal Earth Equatorial Radius (IAU 2015 Resolution B3).
    pub const R_EARTH_METERS: f64 = 6_378_100.0;
}