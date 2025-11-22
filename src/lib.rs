// [Mass]
mod mass;
pub use mass::AsMass;
pub use mass::Mass;
// [Temperature]
mod temperature;

#[cfg(not(feature = "f128_stable"))]
type MetricsInternalType = f64;
#[cfg(feature = "f128_stable")]
type MetricsInternalType = f128;
