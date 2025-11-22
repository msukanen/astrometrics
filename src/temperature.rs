use serde::{Deserialize, Serialize};

use crate::MetricsInternalType;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd)]
pub enum Temperature {
    K(MetricsInternalType)
}

/// A trait for anything that could fathomably be represented as [Temperature]…
pub trait AsTemperature {
    fn k(&self) -> Temperature;
}
