# Electricity Module

The `electricity` module provides functions for converting between different units of electric current, such as amperes, milliamperes, microamperes, kilopicoamperes, kiloamperes, and megaamperes.

**Functions**

## `amperes_to_milliamperes(a: f64) -> f64`

Converts amperes to milliamperes.

### Parameters
- `a`: The current value in amperes to be converted to milliamperes.

### Returns
- `f64`: The equivalent value in milliamperes.

### Formula
`mA = A * 1000`

## `milliamperes_to_amperes(ma: f64) -> f64`

Converts milliamperes to amperes.

### Parameters
- `ma`: The current value in milliamperes to be converted to amperes.

### Returns
- `f64`: The equivalent value in amperes.

### Formula
`A = mA / 1000`

## `amperes_to_microamperes(a: f64) -> f64`

Converts amperes to microamperes.

### Parameters
- `a`: The current value in amperes to be converted to microamperes.

### Returns
- `f64`: The equivalent value in microamperes.

### Formula
`μA = A * 1_000_000`

## `microamperes_to_amperes(ua: f64) -> f64`

Converts microamperes to amperes.

### Parameters
- `ua`: The current value in microamperes to be converted to amperes.

### Returns
- `f64`: The equivalent value in amperes.

### Formula
`A = μA / 1_000_000`

## `amperes_to_kilopicoamperes(a: f64) -> f64`

Converts amperes to kilopicoamperes.

### Parameters
- `a`: The current value in amperes to be converted to kilopicoamperes.

### Returns
- `f64`: The equivalent value in kilopicoamperes.

### Formula
`kA = A * 1e12`

## `kilopicoamperes_to_amperes(ka: f64) -> f64`

Converts kilopicoamperes to amperes.

### Parameters
- `ka`: The current value in kilopicoamperes to be converted to amperes.

### Returns
- `f64`: The equivalent value in amperes.

### Formula
`A = kA / 1e12`

## `amperes_to_kiloamperes(a: f64) -> f64`

Converts amperes to kiloamperes.

### Parameters
- `a`: The current value in amperes to be converted to kiloamperes.

### Returns
- `f64`: The equivalent value in kiloamperes.

### Formula
`kA = A / 1000`

## `kiloamperes_to_amperes(ka: f64) -> f64`

Converts kiloamperes to amperes.

### Parameters
- `ka`: The current value in kiloamperes to be converted to amperes.

### Returns
- `f64`: The equivalent value in amperes.

### Formula
`A = kA * 1000`

## `amperes_to_megaamperes(a: f64) -> f64`

Converts amperes to megaamperes.

### Parameters
- `a`: The current value in amperes to be converted to megaamperes.

### Returns
- `f64`: The equivalent value in megaamperes.

### Formula
`MA = A / 1e6`

## `megaamperes_to_amperes(ma: f64) -> f64`

Converts megaamperes to amperes.

### Parameters
- `ma`: The current value in megaamperes to be converted to amperes.

### Returns
- `f64`: The equivalent value in amperes.

### Formula
`A = MA * 1e6`
