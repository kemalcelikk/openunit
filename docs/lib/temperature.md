# Temperature Module

The `temperature` module provides functions for converting between different units of temperature measurements.

**Functions**

## `celsius_to_fahrenheit(c: usize) -> usize`

Converts Celsius to Fahrenheit.

## `fahrenheit_to_celsius(f: usize) -> usize`

Converts Fahrenheit to Celsius.

## `kelvin_to_celsius(k: f64) -> usize`

Converts Kelvin to Celsius.

## `kelvin_to_fahrenheit(k: f64) -> usize`

Converts Kelvin to Fahrenheit.

## `celsius_to_kelvin(c: f64) -> usize`

Converts Celsius to Kelvin.

## `fahrenheit_to_kelvin(f: f64) -> usize`

Converts Fahrenheit to Kelvin.

## `fahrenheit_to_rankine(f: f64) -> f64`

Converts Fahrenheit to Rankine using the formula: `R = F + 459.67`.

## `rankine_to_fahrenheit(r: f64) -> f64`

Converts Rankine to Fahrenheit using the formula: `F = R - 459.67`.

## `celsius_to_rankine(c: f64) -> f64`

Converts Celsius to Rankine using the formula: `R = (C + 273.15) * 9/5`.

## `rankine_to_celsius(r: f64) -> f64`

Converts Rankine to Celsius using the formula: `C = (R - 491.67) * 5/9`.

## `kelvin_to_rankine(k: f64) -> f64`

Converts Kelvin to Rankine using the formula: `R = K * 9/5`.

## `rankine_to_kelvin(r: f64) -> f64`

Converts Rankine to Kelvin using the formula: `K = R * 5/9`.