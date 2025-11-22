mod mass;
pub use mass::Mass;

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
