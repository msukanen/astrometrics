//! Length, Distance, Radii etc.
use std::{cmp::Ordering, ops::{Add, Div, Mul, RangeInclusive, Sub}};
use paste::paste;

use serde::{Deserialize, Serialize};

pub mod iau;
use crate::{DefoAble, MetricsInternalType, defo, iau::*, ratio};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum SpatialUnit {
    /// Meters.
    M(MetricsInternalType),
    /// Astronomical Unit.
    Au(MetricsInternalType),
    /// Light-years.
    Ly(MetricsInternalType),
    /// R⊕ - Earth-radii.
    RE(MetricsInternalType),
    /// R☉ - Solar radii.
    RO(MetricsInternalType),
    /// Parsec.
    Pc(MetricsInternalType,)
}

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

pub trait AsSpatialUnit {
    /// self → meters
    fn m(&self) -> SpatialUnit;
    /// self → au
    fn au(&self) -> SpatialUnit;
    /// self → ly
    fn ly(&self) -> SpatialUnit;
    /// self → Earth radii
    fn re(&self) -> SpatialUnit;
    /// self → Solar radii
    fn ro(&self) -> SpatialUnit;
    /// self → parsec
    fn pc(&self) -> SpatialUnit;
}

impl AsSpatialUnit for SpatialUnit {
    fn m(&self) -> SpatialUnit {
        match self {
            Self::M(_) => *self,
            Self::RE(v) => Self::M(*v * R_EARTH_METERS),
            Self::RO(v) => Self::M(*v * R_SUN_METERS),
            Self::Au(v) => Self::M(*v * AU_METERS),
            Self::Ly(v) => Self::M(*v * LY_METERS),
            Self::Pc(v) => Self::M(*v * PARSEC_METERS),
        }
    }

    fn re(&self) -> SpatialUnit {
        match self {
            Self::M(v) => Self::RE(ratio(*v, R_EARTH_METERS)),
            Self::RE(_) => *self,
            Self::RO(v) => Self::RE(*v * ratio(R_SUN_METERS, R_EARTH_METERS)),
            Self::Au(v) => Self::RE(*v * ratio(AU_METERS, R_EARTH_METERS)),
            Self::Ly(v) => Self::RE(*v * ratio(LY_METERS, R_EARTH_METERS)),
            Self::Pc(v) => Self::RE(*v * ratio(PARSEC_METERS, R_EARTH_METERS)),
        }
    }

    fn ro(&self) -> SpatialUnit {
        match self {
            Self::M(v) => Self::RO(ratio(*v, R_SUN_METERS)),
            Self::RE(v) => Self::RO(*v * ratio(R_EARTH_METERS, R_SUN_METERS)),
            Self::RO(_) => *self,
            Self::Au(v) => Self::RO(*v * ratio(AU_METERS, R_SUN_METERS)),
            Self::Ly(v) => Self::RO(*v * ratio(LY_METERS, R_SUN_METERS)),
            Self::Pc(v) => Self::RO(*v * ratio(PARSEC_METERS, R_SUN_METERS)),
        }
    }

    fn au(&self) -> SpatialUnit {
        match self {
            Self::M(v) => Self::Au(ratio(*v, AU_METERS)),
            Self::RE(v) => Self::Au(*v * ratio(R_EARTH_METERS, AU_METERS)),
            Self::RO(v) => Self::Au(*v * ratio(R_SUN_METERS, AU_METERS)),
            Self::Au(_) => *self,
            Self::Ly(v) => Self::Au(*v * ratio(LY_METERS, AU_METERS)),
            Self::Pc(v) => Self::Au(*v * ratio(PARSEC_METERS, AU_METERS)),
        }
    }

    fn ly(&self) -> SpatialUnit {
        match self {
            Self::M(v) => Self::Ly(ratio(*v, LY_METERS)),
            Self::RE(v) => Self::Ly(*v * ratio(R_EARTH_METERS, LY_METERS)),
            Self::RO(v) => Self::Ly(*v * ratio(R_SUN_METERS, LY_METERS)),
            Self::Au(v) => Self::Ly(*v * ratio(AU_METERS, LY_METERS)),
            Self::Ly(_) => *self,
            Self::Pc(v) => Self::Ly(*v * ratio(PARSEC_METERS, LY_METERS)),
        }
    }

    fn pc(&self) -> SpatialUnit {
        match self {
            Self::M(v) => Self::Pc(*v / PARSEC_METERS),
            Self::RE(v) => Self::Pc(*v * ratio(R_EARTH_METERS, PARSEC_METERS)),
            Self::RO(v) => Self::Pc(*v * ratio(R_SUN_METERS, PARSEC_METERS)),
            Self::Au(v) => Self::Pc(*v * ratio(AU_METERS, PARSEC_METERS)),
            Self::Ly(v) => Self::Pc(*v * ratio(LY_METERS, PARSEC_METERS)),
            Self::Pc(_) => *self,
        }
    }
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

impl SpatialUnit {
    /// Elevates the lower-magnitude mass of the two into higher-magnitude one.
    fn unify(&self, other: &Self) -> (Self, Self) {
        let rank = |su:&SpatialUnit| match su {
            Self::M(_) => 1,
            Self::RE(_) => 2,
            Self::RO(_) => 3,
            Self::Au(_) => 4,
            Self::Ly(_) => 5,
            Self::Pc(_) => 6,
        };

        match rank(self).cmp(&rank(other)) {
            Ordering::Greater => {
                let other_c = match self {
                    Self::M(_) => other.m(),
                    Self::RE(_) => other.re(),
                    Self::RO(_) => other.ro(),
                    Self::Au(_) => other.au(),
                    Self::Ly(_) => other.ly(),
                    Self::Pc(_) => other.pc(),
                };
                (self.clone(), other_c)
            },
            Ordering::Less => {
                let self_c = self.cnv_into(other);
                (self_c, other.clone())
            },
            Ordering::Equal => (self.clone(), other.clone())
        }
    }
}

impl DefoAble for SpatialUnit {
    fn raw(&self) -> MetricsInternalType {
        match self {
            Self::M(v)  |
            Self::RE(v) |
            Self::RO(v) |
            Self::Au(v) |
            Self::Ly(v) |
            Self::Pc(v) => *v,
        }
    }

    fn set(&mut self, value: MetricsInternalType) {
        match self {
            Self::M(v)  |
            Self::RE(v) |
            Self::RO(v) |
            Self::Au(v) |
            Self::Ly(v) |
            Self::Pc(v) => *v = value,
        }
    }

    fn cnv_into(&self, other: &Self) -> Self {
        match other {
            Self::M(_) => self.m(),
            Self::RE(_) => self.re(),
            Self::RO(_) => self.ro(),
            Self::Au(_) => self.au(),
            Self::Ly(_) => self.ly(),
            Self::Pc(_) => self.pc(),
        }
    }
}

impl PartialEq for SpatialUnit {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for SpatialUnit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (lhs, rhs) = self.unify(other);
        match (lhs, rhs) {
            (SpatialUnit::M(a), SpatialUnit::M(b))   |
            (SpatialUnit::RE(a), SpatialUnit::RE(b)) |
            (SpatialUnit::RO(a), SpatialUnit::RO(b)) |
            (SpatialUnit::Au(a), SpatialUnit::Au(b)) |
            (SpatialUnit::Ly(a), SpatialUnit::Ly(b)) |
            (SpatialUnit::Pc(a), SpatialUnit::Pc(b)) => a.total_cmp(&b).into(),
            _ => unreachable!("unify() unified already")
        }
    }
}

impl Megastructure {
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

/// Macro to define [AsSpatialUnit] impls for a variety of primitives.
macro_rules! define_asspatial_for_prim {
    (f [ $($bits:expr),+ ]) => {paste!{$(
        impl AsSpatialUnit for [<f $bits>] {
            fn m(&self) -> SpatialUnit { SpatialUnit::M(*self as MetricsInternalType) }
            fn re(&self) -> SpatialUnit { SpatialUnit::RE(*self as MetricsInternalType) }
            fn ro(&self) -> SpatialUnit { SpatialUnit::RO(*self as MetricsInternalType) }
            fn au(&self) -> SpatialUnit { SpatialUnit::Au(*self as MetricsInternalType) }
            fn ly(&self) -> SpatialUnit { SpatialUnit::Ly(*self as MetricsInternalType) }
            fn pc(&self) -> SpatialUnit { SpatialUnit::Pc(*self as MetricsInternalType) }
        }
    )*}};
    ($($bits:expr),+) => {paste!{$(
        // unsigned
        impl AsSpatialUnit for [<u $bits>] {
            fn m(&self) -> SpatialUnit { (*self as MetricsInternalType).m() }
            fn re(&self) -> SpatialUnit { (*self as MetricsInternalType).re() }
            fn ro(&self) -> SpatialUnit { (*self as MetricsInternalType).ro() }
            fn au(&self) -> SpatialUnit { (*self as MetricsInternalType).au() }
            fn ly(&self) -> SpatialUnit { (*self as MetricsInternalType).ly() }
            fn pc(&self) -> SpatialUnit { (*self as MetricsInternalType).pc() }
        }
        // signed
        impl AsSpatialUnit for [<i $bits>] {
            fn m(&self) -> SpatialUnit { (*self as MetricsInternalType).m() }
            fn re(&self) -> SpatialUnit { (*self as MetricsInternalType).re() }
            fn ro(&self) -> SpatialUnit { (*self as MetricsInternalType).ro() }
            fn au(&self) -> SpatialUnit { (*self as MetricsInternalType).au() }
            fn ly(&self) -> SpatialUnit { (*self as MetricsInternalType).ly() }
            fn pc(&self) -> SpatialUnit { (*self as MetricsInternalType).pc() }
        }
    )*}};
}

#[cfg(not(feature = "f128_stable"))]
define_asspatial_for_prim!(f [32, 64]);
#[cfg(feature = "f128_stable")]
define_asspatial_for_prim!(f [32, 64, 128]);
define_asspatial_for_prim!(8, 16, 32, 64, 128, size);
defo!(SpatialUnit; float [32, 64, 128], int [8, 16, 32, 64, 128, size]);

#[cfg(test)]
mod spatial_tests {
    use super::*;

    #[test]
    fn gr_range_works() {
        let gr = Megastructure::from(((6.ly(), 12.ly()), (15.ly(), 30.ly()), (40.ly(), 42.ly())));
        assert_eq!(Some(SpatialContained::VisibleDisk), gr.contains(&7.0.ly()));
        assert_ne!(Some(SpatialContained::Arms), gr.contains(&(15.0 - f64::EPSILON*10.0).ly()));
        assert_eq!(Some(SpatialContained::Arms), gr.contains(&(15.0 + f64::EPSILON*10.0).ly()));
    }

    #[test]
    fn comparison() {
        let a = 1.ly();
        assert!(a < 2);
        assert!(2 > a);
    }
}