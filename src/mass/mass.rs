//! Mass
//! 
//! Grams, kilograms, M⊕, M♃, and M☉
use std::{cmp::Ordering, fmt::Display, ops::{Add, Div, Mul, Sub}};

use serde::{Deserialize, Serialize};

use crate::{AsMass, DefoAble, MetricsInternalType, defo, ratio};
use paste::paste;

/// Some mass "magnitudes".
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Mass {
    /// Grams.
    G(MetricsInternalType),
    /// Kilograms.
    Kg(MetricsInternalType),
    /// M⊕ - Earth masses.
    ME(MetricsInternalType),
    /// M♃, Mjup - Jovian/Jupiter masses.
    MJ(MetricsInternalType),
    /// M☉ - Solar masses.
    MO(MetricsInternalType)
}

impl DefoAble for Mass {
    fn raw(&self) -> MetricsInternalType {
        match self {
            Self::MO(v)|
            Self::MJ(v)|
            Self::ME(v)|
            Self::Kg(v)|
            Self::G(v) => *v
        }
    }

    fn set(&mut self, to: MetricsInternalType) {
        match self {
            Self::MO(v)|
            Self::MJ(v)|
            Self::ME(v)|
            Self::Kg(v)|
            Self::G(v) => *v = to
        }
    }

    fn cnv_into(&self, other: &Self) -> Self {
        match other {
            Self::G(_) => self.g(),
            Self::Kg(_) => self.kg(),
            Self::ME(_) => self.me(),
            Self::MJ(_) => self.mj(),
            Self::MO(_) => self.mo()
        }
    }
}

impl Mass {
    /// Elevates the lower-magnitude mass of the two into higher-magnitude one.
    fn unify(&self, other: &Self) -> (Self, Self) {
        let rank = |m:&Mass| match m {
            Self::MO(_) => 5,
            Self::MJ(_) => 4,
            Self::ME(_) => 3,
            Self::Kg(_) => 2,
            Self::G(_) => 1
        };

        match rank(self).cmp(&rank(other)) {
            Ordering::Greater => {
                let other_c = match self {
                    Self::MO(_) => other.mo(),
                    Self::MJ(_) => other.mj(),
                    Self::ME(_) => other.me(),
                    Self::Kg(_) => other.kg(),
                    Self::G(_) => other.g()
                };
                (*self, other_c)
            },
            Ordering::Less => {
                let self_c = self.cnv_into(other);
                (self_c, *other)
            },
            Ordering::Equal => (*self, *other)
        }
    }

    /// self → `f64`
    pub fn as_f64(&self) -> f64 { self.into() }
}

const SOL_KG: MetricsInternalType = 1.98847e30;
const JUP_KG: MetricsInternalType = 1.89813e27;
const EARTH_KG: MetricsInternalType = 5.9722e24;

/// Grams to kg.
const fn g_to_kg(g: MetricsInternalType) -> MetricsInternalType {
    g / 1_000.0 as MetricsInternalType
}

/// kg to grams.
const fn kg_to_g(kg: MetricsInternalType) -> MetricsInternalType {
    kg * 1_000.0 as MetricsInternalType
}

impl AsMass for Mass {
    fn mo(&self) -> Mass {
        match self {
            Self::MO(_) => *self,
            Self::MJ(v) => Self::MO(*v * ratio(JUP_KG, SOL_KG)),
            Self::ME(v) => Self::MO(*v * ratio(EARTH_KG, SOL_KG)),
            Self::Kg(v) => Self::MO(*v * SOL_KG),
            Self::G(v) => Self::MO(*v * kg_to_g(SOL_KG)),
        }
    }

    fn mj(&self) -> Mass {
        match self {
            Self::MO(v) => Self::MJ(*v * ratio(SOL_KG, JUP_KG)),
            Self::MJ(_) => *self,
            Self::ME(v) => Self::MJ(*v * ratio(EARTH_KG, JUP_KG)),
            Self::Kg(v) => Self::MJ(*v * JUP_KG),
            Self::G(v) => Self::MJ(*v / kg_to_g(JUP_KG)),
        }
    }

    fn me(&self) -> Mass {
        match self {
            Self::MO(v) => Self::ME(*v * ratio(SOL_KG, EARTH_KG)),
            Self::MJ(v) => Self::ME(*v * ratio(JUP_KG, EARTH_KG)),
            Self::ME(_) => *self,
            Self::Kg(v) => Self::ME(*v / EARTH_KG),
            Self::G(v) => Self::ME(*v / kg_to_g(EARTH_KG)),
        }
    }

    fn kg(&self) -> Mass {
        match self {
            Self::MO(v) => Self::Kg(*v * SOL_KG),
            Self::MJ(v) => Self::Kg(*v * JUP_KG),
            Self::ME(v) => Self::Kg(*v * EARTH_KG),
            Self::Kg(_) => *self,
            Self::G(v) => Self::Kg(g_to_kg(*v))
        }
    }

    fn g(&self) -> Mass {
        match self {
            Self::MO(v) => Self::G(*v * kg_to_g(SOL_KG)),
            Self::MJ(v) => Self::G(*v * kg_to_g(JUP_KG)),
            Self::ME(v) => Self::G(*v * kg_to_g(EARTH_KG)),
            Self::Kg(v) => Self::G(kg_to_g(*v)),
            Self::G(_) => *self
        }
    }
}

/// Macro to define [AsMass] impls for a variety of primitives.
macro_rules! define_asmass_for_prim {
    (f [ $($bits:expr),+ ]) => {paste!{$(
        impl AsMass for [<f $bits>] {
            fn mo(&self) -> Mass { Mass::MO(*self as MetricsInternalType) }
            fn mj(&self) -> Mass { Mass::MJ(*self as MetricsInternalType) }
            fn me(&self) -> Mass { Mass::ME(*self as MetricsInternalType) }
            fn kg(&self) -> Mass { Mass::Kg(*self as MetricsInternalType) }
            fn g(&self) -> Mass { Mass::G(*self as MetricsInternalType) }
        }
    )*}};
    ($($bits:expr),+) => {paste!{$(
        // unsigned
        impl AsMass for [<u $bits>] {
            fn mo(&self) -> Mass { (*self as MetricsInternalType).mo() }
            fn mj(&self) -> Mass { (*self as MetricsInternalType).mj() }
            fn me(&self) -> Mass { (*self as MetricsInternalType).me() }
            fn kg(&self) -> Mass { (*self as MetricsInternalType).kg() }
            fn g(&self) -> Mass { (*self as MetricsInternalType).g() }
        }
        // signed
        impl AsMass for [<i $bits>] {
            fn mo(&self) -> Mass { (*self as MetricsInternalType).mo() }
            fn mj(&self) -> Mass { (*self as MetricsInternalType).mj() }
            fn me(&self) -> Mass { (*self as MetricsInternalType).me() }
            fn kg(&self) -> Mass { (*self as MetricsInternalType).kg() }
            fn g(&self) -> Mass { (*self as MetricsInternalType).g() }
        }
    )*}};
}

impl PartialEq for Mass {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Mass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (s, o) = self.unify(other);
        Some(s.raw().total_cmp(&o.raw()))
    }
}

#[cfg(not(feature = "f128_stable"))]
define_asmass_for_prim!(f [32, 64]);
#[cfg(feature = "f128_stable")]
define_asmass_for_prim!(f [32, 64, 128]);
define_asmass_for_prim!(8, 16, 32, 64, 128, size);
defo!(Mass; float [32, 64, 128], int [8, 16, 32, 64, 128, size]);

impl Display for Mass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MO(v) => write!(f, "{:.2} M☉", v),
            Self::MJ(v) => write!(f, "{:.3} M♃", v),
            Self::ME(v) => write!(f, "{:.3} M⊕", v),
            Self::Kg(v) => write!(f, "{:.1} kg", v),// preferably use grams if you need more than one decimal…
            Self::G(v) => write!(f, "{:.0}g", v),// there's no mg (yet), but less than gram is not really in the menu for *this* library, currently.
        }
    }
}

#[cfg(test)]
mod mass_tests {
    use super::*;

    #[test]
    fn comparison() {
        let a = 1.kg();
        let b = 1.5.kg();
        let c = 1.0.kg();
        assert!(a < b);
        assert!(a == c);
        assert!(b > c);
        assert!(a < 2.0);
    }

    #[test]
    fn operators() {
        let a = 1.kg();
        let b = 0.5.kg();
        let a_b = &a + &b;
        assert_eq!(1.5.kg(), a_b);
    }
}