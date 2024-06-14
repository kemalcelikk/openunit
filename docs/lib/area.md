# Area Module

The `area` module provides functions for converting between different units of area, such as square meters, square feet, square kilometers, and square miles.

**Functions**

## `square_meters_to_square_feet(m: f64) -> f64`

Converts square meters to square feet.

### Parameters
- `m`: The area value in square meters to be converted to square feet.

### Returns
- `f64`: The equivalent value in square feet.

### Formula
`ft^2 = m^2 * 10.764`

## `square_feet_to_square_meters(ft: f64) -> f64`

Converts square feet to square meters.

### Parameters
- `ft`: The area value in square feet to be converted to square meters.

### Returns
- `f64`: The equivalent value in square meters.

### Formula
`m^2 = ft^2 / 10.764`

## `square_kilometers_to_square_miles(km: f64) -> f64`

Converts square kilometers to square miles.

### Parameters
- `km`: The area value in square kilometers to be converted to square miles.

### Returns
- `f64`: The equivalent value in square miles.

### Formula
`mi^2 = km^2 / 2.59`

## `square_miles_to_square_kilometers(mi: f64) -> f64`

Converts square miles to square kilometers.

### Parameters
- `mi`: The area value in square miles to be converted to square kilometers.

### Returns
- `f64`: The equivalent value in square kilometers.

### Formula
`km^2 = mi^2 * 2.59`

## `square_millimeters_to_square_centimeters(mm: f64) -> f64`

Converts square millimeters to square centimeters.

### Parameters
- `mm`: The area value in square millimeters to be converted to square centimeters.

### Returns
- `f64`: The equivalent value in square centimeters.

### Formula
`cm^2 = mm^2 / 100`

## `square_centimeters_to_square_millimeters(cm: f64) -> f64`

Converts square centimeters to square millimeters.

### Parameters
- `cm`: The area value in square centimeters to be converted to square millimeters.

### Returns
- `f64`: The equivalent value in square millimeters.

### Formula
`mm^2 = cm^2 * 100`

## `hectares_to_acres(hectares: f64) -> f64`

Converts hectares to acres.

### Parameters
- `hectares`: The area value in hectares to be converted to acres.

### Returns
- `f64`: The equivalent value in acres.

### Formula
`acres = hectares * 2.471`

## `acres_to_hectares(acres: f64) -> f64`

Converts acres to hectares.

### Parameters
- `acres`: The area value in acres to be converted to hectares.

### Returns
- `f64`: The equivalent value in hectares.

### Formula
`hectares = acres / 2.471`

## `square_yards_to_square_feet(yd: f64) -> f64`

Converts square yards to square feet.

### Parameters
- `yd`: The area value in square yards to be converted to square feet.

### Returns
- `f64`: The equivalent value in square feet.

### Formula
`ft^2 = yd^2 * 9`

## `square_feet_to_square_yards(ft: f64) -> f64`

Converts square feet to square yards.

### Parameters
- `ft`: The area value in square feet to be converted to square yards.

### Returns
- `f64`: The equivalent value in square yards.

### Formula
`yd^2 = ft^2 / 9`
