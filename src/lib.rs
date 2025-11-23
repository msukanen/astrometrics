// [Mass]
mod mass;
pub use mass::{Mass, AsMass};
// [Temperature]
mod temperature;
pub use temperature::{Temperature, AsTemperature};
// [Spatial]
mod spatial;
pub use spatial::{AsSpatialUnit, SpatialUnit, iau::*};

// Whenever 'f128' is stable, we're ready for it.
#[cfg(not(feature = "f128_stable"))]
type MetricsInternalType = f64;
#[cfg(feature = "f128_stable")]
type MetricsInternalType = f128;

/// Define primitives-related ops for one or other metric.
/// 
/// Using `self.clone()` because e.g. RangeInclusive isn't Copy due some idiot in the stdlib team had a brainfart…
#[macro_export]
macro_rules! define_prim_ops_for_metric {
    (f [$( $bits:expr ),+]; $metric:ident) => {paste!{$(
        impl Add<[<f $bits>]> for $metric {
            type Output = Self;
            fn add(self, rhs: [<f $bits>]) -> Self::Output { let mut s = self.clone(); s.set(self.raw() + rhs as MetricsInternalType); s }
        }
        impl Sub<[<f $bits>]> for $metric {
            type Output = Self;
            fn sub(self, rhs: [<f $bits>]) -> Self::Output { let mut s = self.clone(); s.set(self.raw() - rhs as MetricsInternalType); s }
        }
        impl Div<[<f $bits>]> for $metric {
            type Output = Self;
            fn div(self, rhs: [<f $bits>]) -> Self::Output { let mut s = self.clone(); s.set(self.raw() / rhs as MetricsInternalType); s }
        }
        impl Mul<[<f $bits>]> for $metric {
            type Output = Self;
            fn mul(self, rhs: [<f $bits>]) -> Self::Output { let mut s = self.clone(); s.set(self.raw() * rhs as MetricsInternalType); s }
        }
        impl Mul<$metric> for [<f $bits>] {
            type Output = $metric;
            fn mul(self, rhs: $metric) -> Self::Output { rhs * self }
        }
    )*}};
    ([$( $bits:expr ),+]; $metric:ident) => {paste!{$(
        // Unsigned ones …
        impl Add<[<u $bits>]> for $metric {
            type Output = Self;
            fn add(self, rhs: [<u $bits>]) -> Self::Output { let mut s = self.clone(); s.set(self.raw() + rhs as MetricsInternalType); s }
        }
        impl Sub<[<u $bits>]> for $metric {
            type Output = Self;
            fn sub(self, rhs: [<u $bits>]) -> Self::Output { let mut s = self.clone(); s.set(self.raw() - rhs as MetricsInternalType); s }
        }
        impl Div<[<u $bits>]> for $metric {
            type Output = Self;
            fn div(self, rhs: [<u $bits>]) -> Self::Output { let mut s = self.clone(); s.set(self.raw() / rhs as MetricsInternalType); s }
        }
        impl Mul<[<u $bits>]> for $metric {
            type Output = Self;
            fn mul(self, rhs: [<u $bits>]) -> Self::Output { let mut s = self.clone(); s.set(self.raw() * rhs as MetricsInternalType); s }
        }
        impl Mul<$metric> for [<u $bits>] {
            type Output = $metric;
            fn mul(self, rhs: $metric) -> Self::Output { rhs * self as MetricsInternalType }
        }
        // Signatures for all!
        impl Add<[<i $bits>]> for $metric {
            type Output = Self;
            fn add(self, rhs: [<i $bits>]) -> Self::Output { let mut s = self.clone(); s.set(self.raw() + rhs as MetricsInternalType); s }
        }
        impl Sub<[<i $bits>]> for $metric {
            type Output = Self;
            fn sub(self, rhs: [<i $bits>]) -> Self::Output { let mut s = self.clone(); s.set(self.raw() - rhs as MetricsInternalType); s }
        }
        impl Div<[<i $bits>]> for $metric {
            type Output = Self;
            fn div(self, rhs: [<i $bits>]) -> Self::Output { let mut s = self.clone(); s.set(self.raw() / rhs as MetricsInternalType); s }
        }
        impl Mul<[<i $bits>]> for $metric {
            type Output = Self;
            fn mul(self, rhs: [<i $bits>]) -> Self::Output { let mut s = self.clone(); s.set(self.raw() * rhs as MetricsInternalType); s }
        }
        impl Mul<$metric> for [<i $bits>] {
            type Output = $metric;
            fn mul(self, rhs: $metric) -> Self::Output { rhs * self as MetricsInternalType }
        }
    )*}};
}

/// Define `$trait` for a metric along with the associated `$fn`.
#[macro_export]
macro_rules! define_ops_for_metric {
    ( [ $( ($trait:ident, $fn:ident) ),* ]; $metric:ident ) => {$(
            // The root of all not-so-evil…
            impl $trait<&$metric> for &$metric {
                type Output = $metric;
                fn $fn(self, rhs: &$metric) -> Self::Output { self.clone().$fn(rhs.raw()) }
            }
            impl $trait for $metric {
                type Output = $metric;
                fn $fn(self, rhs: Self) -> Self::Output {<&$metric as $trait<&$metric>>::$fn(&self, &rhs)}
            }
            impl $trait<$metric> for &$metric {
                type Output = $metric;
                fn $fn(self, rhs: $metric) -> Self::Output {<&$metric as $trait<&$metric>>::$fn(&self, &rhs)}
            }
            impl $trait<&$metric> for $metric {
                type Output = $metric;
                fn $fn(self, rhs: &$metric) -> Self::Output {<&$metric as $trait<&$metric>>::$fn(&self, rhs)}
            }
    )*};
}

/// Generic ratio calc.
const fn ratio(num: MetricsInternalType, denom: MetricsInternalType) -> MetricsInternalType {
    num / denom
}
