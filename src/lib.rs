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

#[macro_export]
/// `From<$metric>` for $some_primitive.
/// 
/// `PartialEq<$some_primitive>` for `$metric` and the other way 'round.
/// 
/// `PartialOrd<$some_primitive>` for `$metric` and the other way 'round.
/// 
/// `Add`, `Sub`, `Mul`, `Div` … for the whole family.
/// 
/// # Examples
/// 
/// ```text
/// defo!(SomeType; float [32, 64, 128], int [8, 16, 32, 64, 128, size]);
/// ```
macro_rules! defo {
    // Combo! Lets break it down…
    ($metric:ident; float [$($f_bits:tt),+], int [$($i_bits:tt),+]) => {
        defo!(@metric $metric);
        defo!(@floats [$($f_bits),+]; $metric);
        defo!(@ints [$($i_bits),+]; $metric);
    };

    // Floaty boats…
    (@floats [$($bits:tt),+]; $metric:ident) => {
        $(defo!(@float $bits; $metric);)*
    };
    (@float 128; $metric:ident) => {
        #[cfg(feature = "f128_stable")]
        defo!(@impl_f 128; $metric);
    };
    (@float $bits:tt; $metric:ident) => {
        defo!(@impl_f $bits; $metric);
    };
    (@impl_f $bits:tt; $metric:ident) => {paste!{
        defo!(@impl_it [<f $bits>]; $metric);
    }};

    // Integerishers…
    (@ints [$($bits:tt),+]; $metric:ident) => {paste!{$(
        defo!(@impl_it [<u $bits>]; $metric);
        defo!(@impl_it [<i $bits>]; $metric);
    )*}};

    // Calculus…
    (@calc $typ:ident [$(($trait:ident, $fn:ident)),+]; $metric:ident ) => {$(
        defo!(@calc_t $typ $trait $fn; $metric);
    )*};
    // route Mul and add Mul special case (for symmetry)
    (@calc_t $typ:ident Mul mul; $metric:ident) => {
        defo!(@calc_t_actual $typ Mul mul; $metric);
        impl Mul<&$metric> for &$typ {
            type Output = $metric;
            fn mul(self, rhs: &$metric) -> Self::Output {
                let mut lhs = *rhs;
                lhs.set((*self as MetricsInternalType).mul(rhs.raw()));
                lhs
            }
        }
        // …and to be full exhaust…
        impl Mul<$metric> for &$typ {
            type Output = $metric;
            fn mul(self, rhs: $metric) -> Self::Output {<&$typ as Mul<&$metric>>::mul(&self, &rhs)}
        }
        impl Mul<&$metric> for $typ {
            type Output = $metric;
            fn mul(self, rhs: &$metric) -> Self::Output {<&$typ as Mul<&$metric>>::mul(&self, &rhs)}
        }
        impl Mul<$metric> for $typ {
            type Output = $metric;
            fn mul(self, rhs: $metric) -> Self::Output {<&$typ as Mul<&$metric>>::mul(&self, &rhs)}
        }
    };
    // route Add, Sub, Div
    (@calc_t $typ:ident $trait:ident $fn:ident; $metric:ident) => {
        defo!(@calc_t_actual $typ $trait $fn; $metric);
    };
    // Actual impl of the calc traits.
    (@calc_t_actual $typ:ident $trait:ident $fn:ident; $metric:ident) => {
        // metric +-*/ primitive
        impl $trait<&$typ> for &$metric {
            type Output = $metric;
            fn $fn(self, rhs: &$typ) -> Self::Output {
                let mut s = *self;
                s.set(self.raw().$fn(*rhs as MetricsInternalType));
                s
            }
        }
        impl $trait<&$typ> for $metric {
            type Output = $metric;
            fn $fn(self, rhs: &$typ) -> Self::Output {<&$metric as $trait<&$typ>>::$fn(&self, rhs)}
        }
        impl $trait<$typ> for &$metric {
            type Output = $metric;
            fn $fn(self, rhs: $typ) -> Self::Output {<&$metric as $trait<&$typ>>::$fn(&self, &rhs)}
        }
        impl $trait<$typ> for $metric {
            type Output = $metric;
            fn $fn(self, rhs: $typ) -> Self::Output {<&$metric as $trait<&$typ>>::$fn(&self, &rhs)}
        }
    };

    // Define calc for core $metric itself
    (@metric $metric:ident) => {
        defo!(@calc_m [(Add, add), (Sub, sub), (Div, div), (Mul, mul)]; $metric);
    };
    (@calc_m [$(($trait:ident, $fn:ident)),+]; $metric:ident) => {
        $(defo!(@calc_m_t $trait $fn; $metric);)*
    };
    (@calc_m_t $trait:ident $fn:ident; $metric:ident) => {
        impl $trait<&$metric> for &$metric {
            type Output = $metric;
            fn $fn(self, rhs: &$metric) -> Self::Output {
                let mut s = *self;
                s.set(self.raw().$fn(rhs.cnv_into(&self).raw()));
                s
            }
        }
        impl $trait<$metric> for &$metric {
            type Output = $metric;
            fn $fn(self, rhs: $metric) -> Self::Output {<&$metric as $trait<&$metric>>::$fn(self, &rhs)}
        }
        impl $trait<&$metric> for $metric {
            type Output = $metric;
            fn $fn(self, rhs: &$metric) -> Self::Output {<&$metric as $trait<&$metric>>::$fn(&self, rhs)}
        }
        impl $trait<$metric> for $metric {
            type Output = $metric;
            fn $fn(self, rhs: $metric) -> Self::Output {<&$metric as $trait<&$metric>>::$fn(&self, &rhs)}
        }
    };

    //
    // Bolt it all together…
    //
    (@impl_it $typ:ident; $metric:ident) => {paste!{
        defo!(@calc $typ [(Add, add), (Sub, sub), (Div, div), (Mul, mul)]; $metric);
        
        impl From<&$metric> for $typ { fn from(value: &$metric) -> $typ {
            let v = value.raw();
            if v > $typ::MAX as MetricsInternalType {
                log::warn!("Contained raw value '{v}' of the {} exceeds the capacity of a `{}`!", stringify!($metric), stringify!($typ));
            }
            v as $typ
        }}
        impl From<$metric> for $typ { fn from(value: $metric) -> $typ {<$typ as From<&$metric>>::from(&value)}}

        impl PartialEq<$typ> for $metric { fn eq(&self, other: &$typ) -> bool {self.raw().total_cmp(&(*other as MetricsInternalType)) == Ordering::Equal}}
        impl PartialEq<$metric> for $typ { fn eq(&self, other: &$metric) -> bool { (*self as MetricsInternalType).total_cmp(&other.raw()) == Ordering::Equal }}

        impl PartialOrd<$typ> for $metric { fn partial_cmp(&self, other: &$typ) -> Option<Ordering> {
            self.raw().total_cmp(&(*other as MetricsInternalType)).into()
        }}
        impl PartialOrd<$metric> for $typ { fn partial_cmp(&self, other: &$metric) -> Option<Ordering> {
            (*self as MetricsInternalType).total_cmp(&other.raw()).into()
        }}
    }};
}

/// Generic ratio calc.
const fn ratio(num: MetricsInternalType, denom: MetricsInternalType) -> MetricsInternalType {
    num / denom
}

/// A *must-implement* trait for all metrics that utilize [defo] macro.
pub trait DefoAble {
    /// Get the raw underlying value.
    fn raw(&self) -> MetricsInternalType;
    /// Set internal value to `value`.
    fn set(&mut self, value: MetricsInternalType);
    /// Get `self` as the `other` variant.
    fn cnv_into(&self, other: &Self) -> Self;
}

/// A trait for anything that can be sensibly squared (x²).
pub trait Squared {
    /// Self squared…
    fn sq(&self) -> Self;
}