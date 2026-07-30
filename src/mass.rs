mod mass;
pub use mass::Mass;

use crate::DefoAble;

/// Trait for converting `self` to some specific [Mass]-type.
pub trait AsMass {
    /// self → M☉
    fn mo(&self) -> Mass;
    /// self → M♃
    fn mj(&self) -> Mass;
    /// self → M⊕
    fn me(&self) -> Mass;
    /// self → kg
    fn kg(&self) -> Mass;
    /// self → g
    fn g(&self) -> Mass;
}

// Some masquerading:
impl Mass {
    #[inline] pub fn as_mo(&self) -> Mass { self.raw().mo() }
    #[inline] pub fn as_mj(&self) -> Mass { self.raw().mj() }
    #[inline] pub fn as_me(&self) -> Mass { self.raw().me() }
    #[inline] pub fn as_kg(&self) -> Mass { self.raw().kg() }
    #[inline] pub fn as_g(&self)  -> Mass { self.raw().g()  }
}
