# Light Module

The `light` module provides functions for converting between different units of light measurements, such as illuminance, luminous flux, and luminous intensity.

**Functions**

## `lux_to_kilolux(lx: f64) -> f64`

Converts lux to kilolux.

### Parameters
- `lx`: The illuminance value in lux to be converted to kilolux.

### Returns
- `f64`: The equivalent value in kilolux.

### Formula
`klx = lx / 1000`

## `kilolux_to_lux(klx: f64) -> f64`

Converts kilolux to lux.

### Parameters
- `klx`: The illuminance value in kilolux to be converted to lux.

### Returns
- `f64`: The equivalent value in lux.

### Formula
`lx = klx * 1000`

## `lux_to_millilux(lx: f64) -> f64`

Converts lux to millilux.

### Parameters
- `lx`: The illuminance value in lux to be converted to millilux.

### Returns
- `f64`: The equivalent value in millilux.

### Formula
`mlx = lx * 1000`

## `millilux_to_lux(mlx: f64) -> f64`

Converts millilux to lux.

### Parameters
- `mlx`: The illuminance value in millilux to be converted to lux.

### Returns
- `f64`: The equivalent value in lux.

### Formula
`lx = mlx / 1000`

## `lux_to_foot_candles(lx: f64) -> f64`

Converts lux to foot-candles.

### Parameters
- `lx`: The illuminance value in lux to be converted to foot-candles.

### Returns
- `f64`: The equivalent value in foot-candles.

### Formula
`fc = lx / 10.764`

## `foot_candles_to_lux(fc: f64) -> f64`

Converts foot-candles to lux.

### Parameters
- `fc`: The illuminance value in foot-candles to be converted to lux.

### Returns
- `f64`: The equivalent value in lux.

### Formula
`lx = fc * 10.764`

## `lux_to_candlepower(lx: f64) -> f64`

Converts lux to candlepower.

### Parameters
- `lx`: The illuminance value in lux to be converted to candlepower.

### Returns
- `f64`: The equivalent value in candlepower.

### Formula
`cp = lx * 0.001464`

## `candlepower_to_lux(cp: f64) -> f64`

Converts candlepower to lux.

### Parameters
- `cp`: The illuminance value in candlepower to be converted to lux.

### Returns
- `f64`: The equivalent value in lux.

### Formula
`lx = cp / 0.001464`
