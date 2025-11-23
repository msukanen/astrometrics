# Astrometrics

Some astrometrics in Rust.

## Metrics

All metrics support `Add`, `Sub`, `Mul` and `Div`, as owned, borrowed, and a mixture of.

### Distance Related

Part of `SpatialUnit` enum.

* m
* au
* ly

### Mass Related

Part of `Mass` enum.

* g
* kg
* M⊕ - Earth masses
* M♃ - Jupiter/Jovian masses
* M☉ - Solar masses

### Radii Related

Part of `SpatialUnit` enum.

* R⊕
* R☉

### Temperature Related

Part of `Temperature` enum.

* K
* X  (specifically for Black Holes due their peculiarity…)
