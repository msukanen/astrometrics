# Astrometrics

Some astrometrics in Rust.

## Metrics

All metrics support e.g. `Add`, `Sub`, `Mul` and `Div`, as owned, borrowed, and a mixture of.
`Mul` in general is "symmetric", unlike the other ops. `PartialOrd` and `PartialEq` are around
in various forms (owned, borrowed, mixed…).

### Distance Related

Part of `SpatialUnit` enum.

* m
* au
* ly
* pc

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

#### Megastructures

Part of `Megastructure` enum.

* GR - galactic radii; a trio of ranges - visible disk, arms, and halo.

### Temperature Related

Part of `Temperature` enum.

* C - Celsius
* K - Kelvin
* D - White Dwarf, a fixed approx. value.
* N - Neutrol stars. A fixed approx. value.
* X - Black Holes due their peculiarity…
