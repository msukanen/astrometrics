//! Length, Distance, Radii etc.
use std::{cmp::Ordering, ops::{Add, Div, DivAssign, Mul, MulAssign, Sub}};
use paste::paste;

use serde::{Deserialize, Serialize};

pub mod iau;
pub(crate) mod megastruct;
use crate::{Cubed, DefoAble, MetricsInternalType, Squared, defo, iau::*, ratio};

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

pub trait AsSpatialUnit : AsCelestialRadii {
    /// self → meters
    fn m(&self) -> SpatialUnit;
    /// self → au
    fn au(&self) -> SpatialUnit;
    /// self → ly
    fn ly(&self) -> SpatialUnit;
    /// self → parsec
    fn pc(&self) -> SpatialUnit;
}

pub trait AsCelestialRadii {
    /// self → Earth radii
    fn re(&self) -> SpatialUnit;
    /// self → Solar radii
    fn ro(&self) -> SpatialUnit;
}

impl AsSpatialUnit for SpatialUnit {
    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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

impl AsCelestialRadii for SpatialUnit {
    #[inline(always)]
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

    #[inline(always)]
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
    #[inline(always)]
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

    #[inline(always)]
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

    #[inline]
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

impl Squared for SpatialUnit {
    #[inline(always)]
    fn sq(&self) -> Self {
        match self {
            Self::M(v) => Self::M(v * v),
            Self::Ly(v) => Self::Ly(v * v),
            Self::Pc(v) => Self::Pc(v * v),
            Self::RE(v) => Self::RE(v * v),
            Self::RO(v) => Self::RO(v * v),
            Self::Au(v) => Self::Au(v * v),
        }
    }
}

impl Cubed for SpatialUnit {
    #[inline(always)]
    fn cubed(&self) -> Self {
        match self {
            Self::M(v) => Self::M(v * v * v),
            Self::Ly(v) => Self::Ly(v * v * v),
            Self::Pc(v) => Self::Pc(v * v * v),
            Self::RE(v) => Self::RE(v * v * v),
            Self::RO(v) => Self::RO(v * v * v),
            Self::Au(v) => Self::Au(v * v * v),
        }
    }
}

/// Macro to define [AsSpatialUnit] impls for a variety of primitives.
macro_rules! define_asspatial_for_prim {
    (f [ $($bits:tt),+ ]) => {$(define_asspatial_for_prim!(@f $bits);)*};
    // f128 special case - drop when f128 is stable enough (and/or hardwarewise useable).
    (@f 128) => {
        #[cfg(feature = "f128-stable")]
        define_asspatial_for_prim!(@f_actual 128);
    };
    (@f $bits:tt) => {define_asspatial_for_prim!(@f_actual $bits);};
    (@f_actual $bits:tt) => {paste!{
        impl AsSpatialUnit for [<f $bits>] {
            fn m(&self) -> SpatialUnit { SpatialUnit::M(*self as MetricsInternalType) }
            fn au(&self) -> SpatialUnit { SpatialUnit::Au(*self as MetricsInternalType) }
            fn ly(&self) -> SpatialUnit { SpatialUnit::Ly(*self as MetricsInternalType) }
            fn pc(&self) -> SpatialUnit { SpatialUnit::Pc(*self as MetricsInternalType) }
        }
        impl AsCelestialRadii for [<f $bits>] {
            fn re(&self) -> SpatialUnit { SpatialUnit::RE(*self as MetricsInternalType) }
            fn ro(&self) -> SpatialUnit { SpatialUnit::RO(*self as MetricsInternalType) }
        }
    }};
    ($($bits:tt),+) => {paste!{$(
        // unsigned
        impl AsSpatialUnit for [<u $bits>] {
            fn m(&self) -> SpatialUnit { (*self as MetricsInternalType).m() }
            fn au(&self) -> SpatialUnit { (*self as MetricsInternalType).au() }
            fn ly(&self) -> SpatialUnit { (*self as MetricsInternalType).ly() }
            fn pc(&self) -> SpatialUnit { (*self as MetricsInternalType).pc() }
        }
        impl AsCelestialRadii for [<u $bits>] {
            fn re(&self) -> SpatialUnit { SpatialUnit::RE(*self as MetricsInternalType) }
            fn ro(&self) -> SpatialUnit { SpatialUnit::RO(*self as MetricsInternalType) }
        }
        // signed
        impl AsSpatialUnit for [<i $bits>] {
            fn m(&self) -> SpatialUnit { (*self as MetricsInternalType).m() }
            fn au(&self) -> SpatialUnit { (*self as MetricsInternalType).au() }
            fn ly(&self) -> SpatialUnit { (*self as MetricsInternalType).ly() }
            fn pc(&self) -> SpatialUnit { (*self as MetricsInternalType).pc() }
        }
        impl AsCelestialRadii for [<i $bits>] {
            fn re(&self) -> SpatialUnit { SpatialUnit::RE(*self as MetricsInternalType) }
            fn ro(&self) -> SpatialUnit { SpatialUnit::RO(*self as MetricsInternalType) }
        }
    )*}};
}
define_asspatial_for_prim!(f [32, 64, 128]);
define_asspatial_for_prim!(8, 16, 32, 64, 128, size);
defo!(SpatialUnit; float [32, 64, 128], int [8, 16, 32, 64, 128, size]);

impl DivAssign<f64> for SpatialUnit {
    /// Div-assign.
    /// 
    /// Note that Div-Z is a possibility, so — plan accordingly.
    fn div_assign(&mut self, rhs: f64) {
        match self {
            Self::Au(v) |
            Self::Ly(v) |
            Self::M(v)  |
            Self::Pc(v) |
            Self::RE(v) |
            Self::RO(v) => *v /= rhs
        }
    }
}

impl DivAssign<f32> for SpatialUnit {
    fn div_assign(&mut self, rhs: f32) {
        self.div_assign(rhs as f64);
    }
}

impl MulAssign<f64> for SpatialUnit {
    fn mul_assign(&mut self, rhs: f64) {
        match self {
            Self::Au(v) |
            Self::Ly(v) |
            Self::M(v)  |
            Self::Pc(v) |
            Self::RE(v) |
            Self::RO(v) => *v *= rhs
        }
    }
}

impl MulAssign<f32> for SpatialUnit {
    fn mul_assign(&mut self, rhs: f32) {
        self.mul_assign(rhs as f64);
    }
}

// Some masquerading:
impl SpatialUnit {
    #[inline] pub fn as_au(&self) -> SpatialUnit { self.raw().au() }
    #[inline] pub fn as_ly(&self) -> SpatialUnit { self.raw().ly() }
    #[inline] pub fn as_m(&self)  -> SpatialUnit { self.raw().m() }
    #[inline] pub fn as_pc(&self) -> SpatialUnit { self.raw().pc() }
    #[inline] pub fn as_re(&self) -> SpatialUnit { self.raw().re() }
    #[inline] pub fn as_ro(&self) -> SpatialUnit { self.raw().ro() }
}
