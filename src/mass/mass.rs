use std::{cmp::Ordering, ops::{Add, Sub, Div, Mul}};

use serde::{Deserialize, Serialize};

use crate::{AsMass, MetricsInternalType};
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
                let self_c = match other {
                    Self::MO(_) => self.mo(),
                    Self::MJ(_) => self.mj(),
                    Self::ME(_) => self.me(),
                    Self::Kg(_) => self.kg(),
                    Self::G(_) => self.g()
                };
                (self_c, *other)
            },
            Ordering::Equal => (*self, *other)
        }
    }

    /// Get the raw underlying value.
    pub fn raw(&self) -> MetricsInternalType {
        match self {
            Self::MO(v)|
            Self::MJ(v)|
            Self::ME(v)|
            Self::Kg(v)|
            Self::G(v) => *v
        }
    }

    /// self → f64
    pub fn as_f64(&self) -> f64 {
        let v = self.raw();
        #[cfg(feature = "f128_stable")]{if v > f64::MAX {
            log::warn!("Contained raw value '{v}' of the Mass exceeds capacity of `f64`!");
        }}
        v as f64
    }

    /// Adjust self contents.
    fn set(&mut self, to: MetricsInternalType) {
        match self {
            Self::MO(v)|
            Self::MJ(v)|
            Self::ME(v)|
            Self::Kg(v)|
            Self::G(v) => *v = to
        }
    }
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

/// Generic ratio calc.
const fn ratio(num: MetricsInternalType, denom: MetricsInternalType) -> MetricsInternalType {
    num / denom
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
    (f $bits:expr) => {paste!{
        impl AsMass for [<f $bits>] {
            fn mo(&self) -> Mass { Mass::MO(*self as MetricsInternalType) }
            fn mj(&self) -> Mass { Mass::MJ(*self as MetricsInternalType) }
            fn me(&self) -> Mass { Mass::ME(*self as MetricsInternalType) }
            fn kg(&self) -> Mass { Mass::Kg(*self as MetricsInternalType) }
            fn g(&self) -> Mass { Mass::G(*self as MetricsInternalType) }
        }
    }};
    ($bits:expr) => {paste!{
        impl AsMass for [<u $bits>] {
            fn mo(&self) -> Mass { (*self as MetricsInternalType).mo() }
            fn mj(&self) -> Mass { (*self as MetricsInternalType).mj() }
            fn me(&self) -> Mass { (*self as MetricsInternalType).me() }
            fn kg(&self) -> Mass { (*self as MetricsInternalType).kg() }
            fn g(&self) -> Mass { (*self as MetricsInternalType).g() }
        }

        impl AsMass for [<i $bits>] {
            fn mo(&self) -> Mass { (*self as MetricsInternalType).mo() }
            fn mj(&self) -> Mass { (*self as MetricsInternalType).mj() }
            fn me(&self) -> Mass { (*self as MetricsInternalType).me() }
            fn kg(&self) -> Mass { (*self as MetricsInternalType).kg() }
            fn g(&self) -> Mass { (*self as MetricsInternalType).g() }
        }
    }};
}
define_asmass_for_prim!(f 32);
define_asmass_for_prim!(f 64);
#[cfg(feature = "f128_stable")]
define_asmass_for_prim!(f 128);
define_asmass_for_prim!(8);
define_asmass_for_prim!(16);
define_asmass_for_prim!(32);
define_asmass_for_prim!(64);
define_asmass_for_prim!(128);
define_asmass_for_prim!(size);

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

/// Define primitives-related ops for [Mass].
macro_rules! define_prim_ops_for_mass {
    (f $bits:expr) => {paste!{
        impl Add<[<f $bits>]> for Mass {
            type Output = Self;
            fn add(self, rhs: [<f $bits>]) -> Self::Output { let mut s = self; s.set(self.raw() + rhs as MetricsInternalType); s }
        }
        impl Sub<[<f $bits>]> for Mass {
            type Output = Self;
            fn sub(self, rhs: [<f $bits>]) -> Self::Output { let mut s = self; s.set(self.raw() - rhs as MetricsInternalType); s }
        }
        impl Div<[<f $bits>]> for Mass {
            type Output = Self;
            fn div(self, rhs: [<f $bits>]) -> Self::Output { let mut s = self; s.set(self.raw() / rhs as MetricsInternalType); s }
        }
        impl Mul<[<f $bits>]> for Mass {
            type Output = Self;
            fn mul(self, rhs: [<f $bits>]) -> Self::Output { let mut s = self; s.set(self.raw() * rhs as MetricsInternalType); s }
        }
        impl Mul<Mass> for [<f $bits>] {
            type Output = Mass;
            fn mul(self, rhs: Mass) -> Self::Output { rhs * self }
        }
    }};
    ($bits:expr) => {paste!{
        // Unsigned ones …
        impl Add<[<u $bits>]> for Mass {
            type Output = Self;
            fn add(self, rhs: [<u $bits>]) -> Self::Output { let mut s = self; s.set(self.raw() + rhs as MetricsInternalType); s }
        }
        impl Sub<[<u $bits>]> for Mass {
            type Output = Self;
            fn sub(self, rhs: [<u $bits>]) -> Self::Output { let mut s = self; s.set(self.raw() - rhs as MetricsInternalType); s }
        }
        impl Div<[<u $bits>]> for Mass {
            type Output = Self;
            fn div(self, rhs: [<u $bits>]) -> Self::Output { let mut s = self; s.set(self.raw() / rhs as MetricsInternalType); s }
        }
        impl Mul<[<u $bits>]> for Mass {
            type Output = Self;
            fn mul(self, rhs: [<u $bits>]) -> Self::Output { let mut s = self; s.set(self.raw() * rhs as MetricsInternalType); s }
        }
        impl Mul<Mass> for [<u $bits>] {
            type Output = Mass;
            fn mul(self, rhs: Mass) -> Self::Output { rhs * self as MetricsInternalType }
        }
        // Signatures for all!
        impl Add<[<i $bits>]> for Mass {
            type Output = Self;
            fn add(self, rhs: [<i $bits>]) -> Self::Output { let mut s = self; s.set(self.raw() + rhs as MetricsInternalType); s }
        }
        impl Sub<[<i $bits>]> for Mass {
            type Output = Self;
            fn sub(self, rhs: [<i $bits>]) -> Self::Output { let mut s = self; s.set(self.raw() - rhs as MetricsInternalType); s }
        }
        impl Div<[<i $bits>]> for Mass {
            type Output = Self;
            fn div(self, rhs: [<i $bits>]) -> Self::Output { let mut s = self; s.set(self.raw() / rhs as MetricsInternalType); s }
        }
        impl Mul<[<i $bits>]> for Mass {
            type Output = Self;
            fn mul(self, rhs: [<i $bits>]) -> Self::Output { let mut s = self; s.set(self.raw() * rhs as MetricsInternalType); s }
        }
        impl Mul<Mass> for [<i $bits>] {
            type Output = Mass;
            fn mul(self, rhs: Mass) -> Self::Output { rhs * self as MetricsInternalType }
        }
    }};
}
// Fun for the whole family!
define_prim_ops_for_mass!(f 32);
define_prim_ops_for_mass!(f 64);
#[cfg(feature = "f128_stable")]
define_prim_ops_for_mass!(f 128);
define_prim_ops_for_mass!(8);
define_prim_ops_for_mass!(16);
define_prim_ops_for_mass!(32);
define_prim_ops_for_mass!(64);
define_prim_ops_for_mass!(128);
define_prim_ops_for_mass!(size);

/// Define `$trait` for [Mass] along with the associated `$fn`.
macro_rules! define_ops_for_mass {
    ( [ $( ($trait:ident, $fn:ident) ),* ] ) => {
        $(
            // The root of all not-so-evil…
            impl $trait<&Mass> for &Mass {
                type Output = Mass;
                fn $fn(self, rhs: &Mass) -> Self::Output { (*self).$fn(rhs.raw()) }
            }
            impl $trait for Mass {
                type Output = Self;
                fn $fn(self, rhs: Self) -> Self::Output {<&Mass as $trait<&Mass>>::$fn(&self, &rhs)}
            }
            impl $trait<Mass> for &Mass {
                type Output = Mass;
                fn $fn(self, rhs: Mass) -> Self::Output {<&Mass as $trait<&Mass>>::$fn(&self, &rhs)}
            }
            impl $trait<&Mass> for Mass {
                type Output = Mass;
                fn $fn(self, rhs: &Mass) -> Self::Output {<&Mass as $trait<&Mass>>::$fn(&self, rhs)}
            }
        )*
    };
}
// Ye typical ops …
define_ops_for_mass!([(Add, add), (Sub, sub), (Mul, mul), (Div, div)]);

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
    }

    #[test]
    fn calc_ops() {
        let a = 1.kg();
        let b = 0.5.kg();
        let a_b = &a + &b;
    }
}