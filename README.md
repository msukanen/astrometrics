# Astrometrics

Some astrometrics in Rust.

## Float Types

By default **astrometrics** uses 64-bit floats, but one can change this with feature flags…

- `retro-32-bit` for 32-bit floats in case some hardware cannot operate with 64-bit.
- `f128-stable` to plug-and-play `f128` whenever it's stable enough for general public.
- `f256-exists` to plug-and-play `f256`, if/when *that* gets implemented…

## Metrics

All metrics support e.g. `Add`, `Sub`, `Mul` and `Div`, as owned, borrowed, and a mixture of.
`Mul` in general is "symmetric", unlike the other ops. `PartialOrd` and `PartialEq` are around
in various forms (owned, borrowed, mixed…).

### Traits

#### `Squared`

Implemented for:

- `Temperature`
- `SpatialUnit`

#### `Cubed`

Implemented for:

- `SpatialUnit`
- `Mass`

### Distance Related…

Part of `SpatialUnit` enum.

- m
- AU
- ly
- pc

#### Coversion/Usage

```rust
// e.g.:
let a = 70_000.au();
let b = 0.7.ly();
let x = a < b; // false...
let y = (a + b); // y is in AU
let z = (b + a); // z is in ly
let o = y.km(); // an absurd number, …but here we go.
```

### Mass Related…

Part of `Mass` enum.

- g
- kg
- M⊕ - Earth masses
- M♃ - Jupiter/Jovian masses
- M☉ - Solar masses

### Radii Related…

Part of `SpatialUnit` enum.

- R⊕
- R☉

#### Megastructures…

Part of `Megastructure` enum.

- GR - galactic radii; a trio of ranges - visible disk, arms, and halo.

### Temperature Related…

Part of `Temperature` enum.

- C - Celsius
- K - Kelvin
- D - White Dwarf, a fixed approx. value.
- N - Neutrol stars. A fixed approx. value.
- X - Black Holes due their peculiarity…
