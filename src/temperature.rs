//! Temperature
//! 
//! Kelvin, Celsius, and the special cases of stellar remnants.
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, Sub, Div, Mul};
use paste::paste;
use serde::{Deserialize, Serialize};

mod k;
pub use k::ABS_ZERO;
use k::K_C_DELTA;
use crate::{DefoAble, MetricsInternalType, Squared, defo};
const K_NEUTRON: Temperature = Temperature::K(1e6);
const K_WDWARF: Temperature = Temperature::K(1e5);

/// Temperature variants.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Temperature {
    /// White Dwarf
    D,
    /// Neutron Star
    N,
    /// Black Hole
    X,
    /// Kelvin.
    K(MetricsInternalType),
    /// Celsius.
    C(MetricsInternalType),
}

/// A trait for anything that could fathomably be represented as [Temperature]…
pub trait AsTemperature {
    /// `self` as Kelvin.
    fn k(&self) -> Temperature;
    /// `self` as Celsius.
    fn c(&self) -> Temperature;
}

impl AsTemperature for Temperature {
    /// `self` as Kelvin. Output value's minimum is clamped to abs.zero.
    fn k(&self) -> Self {
        match self {
            Self::K(v) => Self::K(v.max(0.0)),
            Self::C(v) => Self::K(*v - K_C_DELTA).k(),
            Self::N => K_NEUTRON,
            Self::D => K_WDWARF,
            Self::X => Self::X,
        }
    }

    /// `self` as Celsius. Output value's minimum is clamped to abs.zero.
    fn c(&self) -> Temperature {
        match self {
            Self::C(v) => Self::C(v.max(-K_C_DELTA)),
            Self::K(v) => Self::C(v.max(0.0) + K_C_DELTA),
            Self::N => K_NEUTRON - K_C_DELTA,
            Self::D => K_NEUTRON - K_C_DELTA,
            Self::X => Self::X
        }
    }
}

impl Temperature {
    /// self → `f64`.
    pub fn as_f64(&self) -> f64 {
        let v = self.raw();
        #[cfg(feature = "f128_stable")]{
            if v > f64::MAX { log::warn!("The internally combusted f128 '{v}' is too hot for f64 to handle. We're forced to cool it down, a lot…, down to {}", v as f64)}
        }
        v as f64
    }
}

impl DefoAble for Temperature {
    /// Get the raw underlying value.
    /// 
    /// **Note** that black hole temperature is `NaN`.
    fn raw(&self) -> MetricsInternalType {
        match self {
            Self::C(v) |
            Self::K(v) => *v,
            Self::D => K_WDWARF.raw(),
            Self::N => K_NEUTRON.raw(),
            Self::X => MetricsInternalType::NAN
        }
    }

    /// Set internal value as `to`.
    fn set(&mut self, to: MetricsInternalType) {
        match self {
            Self::C(v) |
            Self::K(v) => *v = to,
            // Stellar remnants stubbornly stay stubborn…
            Self::D |
            Self::N |
            Self::X => ()
        }
    }

    fn cnv_into(&self, other: &Self) -> Self {
        match other {
            Self::X => Self::X,
            Self::N => match self {
                Self::X => Self::X,
                _ => Self::N
            },
            Self::D => match self {
                Self::X => Self::X,
                Self::N => Self::N,
                _ => Self::D
            },
            Self::C(_) => self.c(),
            Self::K(_) => self.k()
        }
    }
}

/// Macro to define [AsMass] impls for a variety of primitives.
macro_rules! define_astemp_for_prim {
    (f [ $($bits:expr),+ ]) => {paste!{$(
        impl AsTemperature for [<f $bits>] {
            fn k(&self) -> Temperature { Temperature::K(*self as MetricsInternalType) }
            fn c(&self) -> Temperature { Temperature::C(*self as MetricsInternalType) }
        }
    )*}};
    ($($bits:expr),+) => {paste!{$(
        // unsigned
        impl AsTemperature for [<u $bits>] {
            fn k(&self) -> Temperature { (*self as MetricsInternalType).k() }
            fn c(&self) -> Temperature { (*self as MetricsInternalType).c() }
        }
        // signed
        impl AsTemperature for [<i $bits>] {
            fn k(&self) -> Temperature { (*self as MetricsInternalType).k() }
            fn c(&self) -> Temperature { (*self as MetricsInternalType).c() }
        }
    )*}};
}

/// PartialEq quirks 101: [Temperature::X] is never eq() with anything *nor* is it ne() either …
impl PartialEq for Temperature {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Black holes can't be compared…
            (Self::X, _) |
            (_, Self::X) => false,
            
            (Self::N, x) |
            (x, Self::N) => x.k().eq(&K_NEUTRON),
            (Self::D, x) |
            (x, Self::D) => x.k().eq(&K_WDWARF),
            (Self::C(a), Self::C(b)) |
            (Self::K(a), Self::K(b)) => a.total_cmp(&b) == Ordering::Equal,
            (Self::K(a), Self::C(b)) |
            (Self::C(b), Self::K(a)) => a.total_cmp(&(b - K_C_DELTA)) == Ordering::Equal
        }
    }

    fn ne(&self, other: &Self) -> bool {
        match (self, other) {
            // Black holes can't be compared…
            (Self::X, _) |
            (_, Self::X) => false,
            _ => !self.eq(other)
        }
    }
}

impl PartialOrd for Temperature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            // Black hole… yeah, impossible to order.
            (Self::X,_) |
            (_,Self::X) => None,
            (Self::N, Self::N) |
            (Self::D, Self::D) => Some(Ordering::Equal),
            (Self::N, Self::D) => Some(Ordering::Greater),
            (Self::D, Self::N) => Some(Ordering::Less),
            (Self::N, x) |
            (x, Self::N) => x.k().raw().total_cmp(&K_NEUTRON.raw()).into(),
            (Self::D, x) |
            (x, Self::D) => x.k().raw().total_cmp(&K_WDWARF.raw()).into(),
            (Self::C(a), Self::C(b)) |
            (Self::K(a), Self::K(b)) => a.total_cmp(&b).into(),
            (Self::C(c), Self::K(k)) => (*c - K_C_DELTA).total_cmp(&k).into(),
            (Self::K(k), Self::C(c)) => k.total_cmp(&(*c - K_C_DELTA)).into()
        }
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "\u{221e}K"),
            Self::N => write!(f, "{}", K_NEUTRON),
            Self::D => write!(f, "{}", K_WDWARF),
            Self::K(v) => write!(f, "{:.1}K", v),
            Self::C(v) => write!(f, "{:.1}⁰C", v)
        }
    }
}

impl Squared for Temperature {
    /// Self squared…
    /// 
    /// Note that squaring temperature values is *usually* utterly meaningless, but it is useful in some equations.
    fn sq(&self) -> Self {
        match self {
            Self::C(v) => Self::C(v * v),
            Self::K(v) => Self::K(v * v),
            // No point to do anything about these:
            Self::D => Self::D,
            Self::N => Self::N,
            Self::X => Self::X
        }
    }
}

macro_rules! define_from_prim_temperature {
    (f [$($bits:tt),+]) => {$(define_from_prim_temperature!(@f $bits);)*};
    // f128 special case - drop when f128 is stable enough (and/or hardwarewise useable).
    (@f 128) => {
        #[cfg(feature = "f128_stable")]
        define_from_prim_temperature!(@b f 128);
    };
    (@f $bits:tt) => {define_from_prim_temperature!(@b f $bits);};
    ($($bits:tt),+) => {paste!{$(
        define_from_prim_temperature!(@b u $bits);
        define_from_prim_temperature!(@b i $bits);
    )*}};
    (@b $prefix:ident $bits:tt) => {paste!{
        impl From<[<$prefix $bits>]> for Temperature { fn from(value: [<$prefix $bits>]) -> Self { Self::K(value as MetricsInternalType )}}
    }}
}

define_from_prim_temperature!(f [32, 64, 128]);
define_from_prim_temperature!(8, 16, 32, 64, 128, size);

#[cfg(not(feature = "f128_stable"))]
define_astemp_for_prim!(f [32, 64]);
#[cfg(feature = "f128_stable")]
define_astemp_for_prim!(f [32, 64, 128]);
define_astemp_for_prim!(8, 16, 32, 64, 128, size);
defo!(Temperature; float [32, 64, 128], int [8, 16, 32, 64, 128, size]);

#[cfg(test)]
mod temperature_tests {
    use crate::AsTemperature;

    #[test]
    fn comparison() {
        let a = 1.k();
        let b = 2.k();
        assert!(a < b);
        assert!(b >= a);
    }

    #[test]
    fn operators() {
        let a = 100.k();
        let b = 50.k();
        let c = a - b;
        assert_eq!(50.k(), c);

        let a = 100.k();
        let b = 50.k();
        assert!(a > b);
        assert_ne!(a, b);
        let c = a / 2.0;
    }
}