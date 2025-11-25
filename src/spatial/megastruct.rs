//! Megastructures to be

use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

use crate::{AsSpatialUnit, MetricsInternalType, SpatialUnit};

/// Spatials for megastructures (e.g. Galaxies).
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Megastructure {
    /// Galactic radii. Highly variable…
    GR {
        visible_disk: RangeInclusive<SpatialUnit>,
        arms: RangeInclusive<SpatialUnit>,
        halo: RangeInclusive<SpatialUnit>
    }
}

/// For e.g. [`Megastructure::contains()`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialContained {
    // GR-specific trio:
    VisibleDisk, Arms, Halo,
}

impl From<((SpatialUnit, SpatialUnit), (SpatialUnit, SpatialUnit), (SpatialUnit, SpatialUnit))> for Megastructure {
    fn from(value: ((SpatialUnit, SpatialUnit), (SpatialUnit, SpatialUnit), (SpatialUnit, SpatialUnit))) -> Self {
        Self::GR {
            visible_disk: value.0.0..=value.0.1,
            arms: value.1.0..=value.1.1,
            halo: value.2.0..=value.2.1
        }
    }
}

impl From<((MetricsInternalType, MetricsInternalType), (MetricsInternalType, MetricsInternalType), (MetricsInternalType, MetricsInternalType))> for Megastructure {
    fn from(value: ((MetricsInternalType, MetricsInternalType), (MetricsInternalType, MetricsInternalType), (MetricsInternalType, MetricsInternalType))) -> Self {
        Self::GR {
            visible_disk: value.0.0.ly()..=value.0.1.ly(),
            arms: value.1.0.ly()..=value.1.1.ly(),
            halo: value.2.0.ly()..=value.2.1.ly()
        }
    }
}

impl Megastructure {
    /// Check which range contains `s`, if any do.
    pub fn contains(&self, s: &SpatialUnit) -> Option<SpatialContained> {
        match self {
            Self::GR { visible_disk, arms, halo } => match () {
                _ if visible_disk.contains(s) => Some(SpatialContained::VisibleDisk),
                _ if arms.contains(s) => Some(SpatialContained::Arms),
                _ if halo.contains(s) => Some(SpatialContained::Halo),
                _ => None
            }
        }
    }
}
