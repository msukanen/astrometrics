//! Length, Distance, etc.
//! 
//! * m, au, ly
use std::{cmp::Ordering, ops::{Add, Div, Mul, RangeInclusive, Sub}};
use paste::paste;

use serde::{Deserialize, Serialize};

pub mod iau;
use crate::{MetricsInternalType, define_ops_for_metric, define_prim_ops_for_metric, iau::*, ratio};

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    /// Galactic radii. Highly variable… The ranges, naturally, in ly (but boxed as [SpatialUnit]).
    GR {
        visible_disk: RangeInclusive<Box<SpatialUnit>>,
        arms: RangeInclusive<Box<SpatialUnit>>,
        halo: RangeInclusive<Box<SpatialUnit>>
    }
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
}

impl AsSpatialUnit for SpatialUnit {
    fn m(&self) -> SpatialUnit {
        match self {
            Self::M(_) => self.clone(),
            Self::RE(v) => Self::M(*v * R_EARTH_METERS),
            Self::RO(v) => Self::M(*v * R_SUN_METERS),
            Self::Au(v) => Self::M(*v * AU_METERS),
            Self::Ly(v) => Self::M(*v * LY_METERS),
            _ => unimplemented!("OK - trying to convert {self:?} into meters doesn't quite work like that…")
        }
    }

    fn re(&self) -> SpatialUnit {
        match self {
            Self::M(v) => Self::RE(ratio(*v, R_EARTH_METERS)),
            Self::RE(_) => self.clone(),
            Self::RO(v) => Self::RE(*v * ratio(R_SUN_METERS, R_EARTH_METERS)),
            Self::Au(v) => Self::RE(*v * ratio(AU_METERS, R_EARTH_METERS)),
            Self::Ly(v) => Self::RE(*v * ratio(LY_METERS, R_EARTH_METERS)),
            _ => unimplemented!("OK - trying to convert {self:?} into R⊕ doesn't quite work like that…")
        }
    }

    fn ro(&self) -> SpatialUnit {
        match self {
            Self::M(v) => Self::RO(ratio(*v, R_SUN_METERS)),
            Self::RE(v) => Self::RO(*v * ratio(R_EARTH_METERS, R_SUN_METERS)),
            Self::RO(_) => self.clone(),
            Self::Au(v) => Self::RO(*v * ratio(AU_METERS, R_SUN_METERS)),
            Self::Ly(v) => Self::RO(*v * ratio(LY_METERS, R_SUN_METERS)),
            _ => unimplemented!("OK - trying to convert {self:?} into R☉ doesn't quite work like that…")
        }
    }

    fn au(&self) -> SpatialUnit {
        match self {
            Self::M(v) => Self::Au(ratio(*v, AU_METERS)),
            Self::RE(v) => Self::Au(*v * ratio(R_EARTH_METERS, AU_METERS)),
            Self::RO(v) => Self::Au(*v * ratio(R_SUN_METERS, AU_METERS)),
            Self::Au(_) => self.clone(),
            Self::Ly(v) => Self::Au(*v * ratio(LY_METERS, AU_METERS)),
            _ => unimplemented!("OK - trying to convert {self:?} into au doesn't quite work like that…")
        }
    }

    fn ly(&self) -> SpatialUnit {
        match self {
            Self::M(v) => Self::Ly(ratio(*v, LY_METERS)),
            Self::RE(v) => Self::Ly(*v * ratio(R_EARTH_METERS, LY_METERS)),
            Self::RO(v) => Self::Ly(*v * ratio(R_SUN_METERS, LY_METERS)),
            Self::Au(v) => Self::Ly(*v * ratio(AU_METERS, LY_METERS)),
            Self::Ly(_) => self.clone(),
            _ => unimplemented!("OK - trying to convert {self:?} into ly doesn't quite work like that…")
        }
    }
}

impl From<((SpatialUnit, SpatialUnit), (SpatialUnit, SpatialUnit), (SpatialUnit, SpatialUnit))> for SpatialUnit {
    fn from(value: ((SpatialUnit, SpatialUnit), (SpatialUnit, SpatialUnit), (SpatialUnit, SpatialUnit))) -> Self {
        Self::GR {
            visible_disk: Box::new(value.0.0)..=Box::new(value.0.1),
            arms: Box::new(value.1.0)..=Box::new(value.1.1),
            halo: Box::new(value.2.0)..=Box::new(value.2.1)
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
            _ => unimplemented!("GR is a rankless variant of SpatialUnit…")
        };

        match rank(self).cmp(&rank(other)) {
            Ordering::Greater => {
                let other_c = match self {
                    Self::M(_) => other.m(),
                    Self::RE(_) => other.re(),
                    Self::RO(_) => other.ro(),
                    Self::Au(_) => other.au(),
                    Self::Ly(_) => other.ly(),
                    _ => unimplemented!("Converting non-GR '{other:?} into GR isn't exactly straightforward…")
                };
                (self.clone(), other_c)
            },
            Ordering::Less => {
                let self_c = match other {
                    Self::M(_) => self.m(),
                    Self::RE(_) => self.re(),
                    Self::RO(_) => self.ro(),
                    Self::Au(_) => self.au(),
                    Self::Ly(_) => self.ly(),
                    _ => unimplemented!("Converting non-GR '{self:?} into GR isn't exactly straightforward…")
                };
                (self_c, other.clone())
            },
            Ordering::Equal => (self.clone(), other.clone())
        }
    }

    /// Get the raw underlying value.
    pub fn raw(&self) -> MetricsInternalType {
        match self {
            Self::M(v)|
            Self::RE(v)|
            Self::RO(v)|
            Self::Au(v)|
            Self::Ly(v) => *v,
            _ => unimplemented!("GR contains multiple *ranges*, not any specific single raw value…")
        }
    }

    fn set(&mut self, value: MetricsInternalType) {
        match self {
            Self::M(v)|
            Self::RE(v)|
            Self::RO(v)|
            Self::Au(v)|
            Self::Ly(v) => *v = value,
            _ => unimplemented!("GR contains multiple *ranges*, not any specific single raw value… So yeah, not even trying")
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
        match (self, other) {
            // normie units use unify()…
            _ if !matches!(self, SpatialUnit::GR {..}) &&
                 !matches!(other, SpatialUnit::GR {..}) => {
                let (lhs, rhs) = self.unify(other);
                match (lhs, rhs) {
                    (SpatialUnit::M(a), SpatialUnit::M(b))   |
                    (SpatialUnit::RE(a), SpatialUnit::RE(b)) |
                    (SpatialUnit::RO(a), SpatialUnit::RO(b)) |
                    (SpatialUnit::Au(a), SpatialUnit::Au(b)) |
                    (SpatialUnit::Ly(a), SpatialUnit::Ly(b)) => a.total_cmp(&b).into(),
                    _ => unimplemented!("Someone forgot to add match branch(es) for new SpatialUnit branch(es)…!")
                }
            },

            (SpatialUnit::GR { halo: h1,.. },
             SpatialUnit::GR { halo: h2,.. }) => {
                let m1: SpatialUnit = (h1.start().ly() + h1.end().ly()) / 2.0;
                let m2: SpatialUnit = (h2.start().ly() + h2.end().ly()) / 2.0;
                m1.partial_cmp(&m2)
            },

            _ => unimplemented!("The first branch should've captured all but GR variant…!")
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
        }
        // signed
        impl AsSpatialUnit for [<i $bits>] {
            fn m(&self) -> SpatialUnit { (*self as MetricsInternalType).m() }
            fn re(&self) -> SpatialUnit { (*self as MetricsInternalType).re() }
            fn ro(&self) -> SpatialUnit { (*self as MetricsInternalType).ro() }
            fn au(&self) -> SpatialUnit { (*self as MetricsInternalType).au() }
            fn ly(&self) -> SpatialUnit { (*self as MetricsInternalType).ly() }
        }
    )*}};
}
#[cfg(not(feature = "f128_stable"))]
define_asspatial_for_prim!(f [32, 64]);
#[cfg(feature = "f128_stable")]
define_asspatial_for_prim!(f [32, 64, 128]);
define_asspatial_for_prim!(8, 16, 32, 64, 128, size);

// Fun for the whole family!
#[cfg(not(feature = "f128_stable"))]
define_prim_ops_for_metric!(f [32, 64]; SpatialUnit);
#[cfg(feature = "f128_stable")]
define_prim_ops_for_metric!(f [32, 64, 128]; SpatialUnit);
define_prim_ops_for_metric!([8, 16, 32, 64, 128, size]; SpatialUnit);
// Ye typical ops …
define_ops_for_metric!([(Add, add), (Sub, sub), (Mul, mul), (Div, div)]; SpatialUnit);
