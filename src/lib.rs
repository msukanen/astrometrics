// [Mass]
mod mass;
pub use mass::{Mass, AsMass};
// [Temperature]
mod temperature;
pub use temperature::{Temperature, AsTemperature};

// Whenever 'f128' is stable, we're ready for it.
#[cfg(not(feature = "f128_stable"))]
type MetricsInternalType = f64;
#[cfg(feature = "f128_stable")]
type MetricsInternalType = f128;
