# Length Module

The `length` module provides functions for converting between different units of length, such as meters, centimeters, kilometers, miles, yards, feet, inches, and nautical miles.

**Functions**

## `meter_to_centimeter(m: f64) -> f64`

Converts meters to centimeters.

### Parameters
- `m`: The length value in meters to be converted to centimeters.

### Returns
- `f64`: The equivalent value in centimeters.

### Formula
`cm = m * 100`

## `centimeter_to_meter(cm: f64) -> f64`

Converts centimeters to meters.

### Parameters
- `cm`: The length value in centimeters to be converted to meters.

### Returns
- `f64`: The equivalent value in meters.

### Formula
`m = cm / 100`

## `meter_to_kilometer(m: f64) -> f64`

Converts meters to kilometers.

### Parameters
- `m`: The length value in meters to be converted to kilometers.

### Returns
- `f64`: The equivalent value in kilometers.

### Formula
`km = m / 1000`

## `kilometer_to_meter(km: f64) -> f64`

Converts kilometers to meters.

### Parameters
- `km`: The length value in kilometers to be converted to meters.

### Returns
- `f64`: The equivalent value in meters.

### Formula
`m = km * 1000`

## `meter_to_millimeter(m: f64) -> f64`

Converts meters to millimeters.

### Parameters
- `m`: The length value in meters to be converted to millimeters.

### Returns
- `f64`: The equivalent value in millimeters.

### Formula
`mm = m * 1000`

## `millimeter_to_meter(mm: f64) -> f64`

Converts millimeters to meters.

### Parameters
- `mm`: The length value in millimeters to be converted to meters.

### Returns
- `f64`: The equivalent value in meters.

### Formula
`m = mm / 1000`

## `meter_to_mile(m: f64) -> f64`

Converts meters to miles.

### Parameters
- `m`: The length value in meters to be converted to miles.

### Returns
- `f64`: The equivalent value in miles.

### Formula
`mi = m / 1609.344`

## `mile_to_meter(mi: f64) -> f64`

Converts miles to meters.

### Parameters
- `mi`: The length value in miles to be converted to meters.

### Returns
- `f64`: The equivalent value in meters.

### Formula
`m = mi * 1609.344`

## `meter_to_yard(m: f64) -> f64`

Converts meters to yards.

### Parameters
- `m`: The length value in meters to be converted to yards.

### Returns
- `f64`: The equivalent value in yards.

### Formula
`yd = m * 1.0936133`

## `yard_to_meter(yd: f64) -> f64`

Converts yards to meters.

### Parameters
- `yd`: The length value in yards to be converted to meters.

### Returns
- `f64`: The equivalent value in meters.

### Formula
`m = yd / 1.0936133`

## `meter_to_foot(m: f64) -> f64`

Converts meters to feet.

### Parameters
- `m`: The length value in meters to be converted to feet.

### Returns
- `f64`: The equivalent value in feet.

### Formula
`ft = m * 3.2808399`

## `foot_to_meter(ft: f64) -> f64`

Converts feet to meters.

### Parameters
- `ft`: The length value in feet to be converted to meters.

### Returns
- `f64`: The equivalent value in meters.

### Formula
`m = ft / 3.2808399`

## `meter_to_inch(m: f64) -> f64`

Converts meters to inches.

### Parameters
- `m`: The length value in meters to be converted to inches.

### Returns
- `f64`: The equivalent value in inches.

### Formula
`in = m * 39.3700787`

## `inch_to_meter(inch: f64) -> f64`

Converts inches to meters.

### Parameters
- `inch`: The length value in inches to be converted to meters.

### Returns
- `f64`: The equivalent value in meters.

### Formula
`m = inch / 39.3700787`

## `yard_to_foot(yd: f64) -> f64`

Converts yards to feet.

### Parameters
- `yd`: The length value in yards to be converted to feet.

### Returns
- `f64`: The equivalent value in feet.

### Formula
`ft = yd * 3`

## `foot_to_yard(ft: f64) -> f64`

Converts feet to yards.

### Parameters
- `ft`: The length value in feet to be converted to yards.

### Returns
- `f64`: The equivalent value in yards.

### Formula
`yd = ft / 3`
