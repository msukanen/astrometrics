//! Kelvin Specific Stuff
//! 
//! Absolute zero.
use crate::{MetricsInternalType, Temperature};

/// Absolute zero, in Kelvin.
pub const ABS_ZERO: Temperature = Temperature::K(0.0);

/// K/⁰C conversion delta.
pub(crate) const K_C_DELTA: MetricsInternalType = 273.15;

pub const K_EPSILON: MetricsInternalType = 0.00001;
