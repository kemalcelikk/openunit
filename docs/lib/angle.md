# Angle Module

The `angle` module provides functions for converting between different angular units, including degrees, radians, and gradians.

**Functions**

## `degrees_to_radians(deg: f64) -> f64`

Converts degrees to radians.

### Parameters
- `deg`: The value in degrees to be converted to radians.

### Returns
- `f64`: The equivalent value in radians.

### Formula
`rad = deg * PI / 180`

## `radians_to_degrees(rad: f64) -> f64`

Converts radians to degrees.

### Parameters
- `rad`: The value in radians to be converted to degrees.

### Returns
- `f64`: The equivalent value in degrees.

### Formula
`deg = rad * 180 / PI`

## `degrees_to_gradians(deg: f64) -> f64`

Converts degrees to gradians.

### Parameters
- `deg`: The value in degrees to be converted to gradians.

### Returns
- `f64`: The equivalent value in gradians.

### Formula
`grad = deg * 200 / 180`

## `gradians_to_degrees(grad: f64) -> f64`

Converts gradians to degrees.

### Parameters
- `grad`: The value in gradians to be converted to degrees.

### Returns
- `f64`: The equivalent value in degrees.

### Formula
`deg = grad * 180 / 200`

## `radians_to_gradians(rad: f64) -> f64`

Converts radians to gradians.

### Parameters
- `rad`: The value in radians to be converted to gradians.

### Returns
- `f64`: The equivalent value in gradians.

### Formula
`grad = rad * 200 / PI`

## `gradians_to_radians(grad: f64) -> f64`

Converts gradians to radians.

### Parameters
- `grad`: The value in gradians to be converted to radians.

### Returns
- `f64`: The equivalent value in radians.

### Formula
`rad = grad * PI / 200`

## `radians_to_turns(rad: f64) -> f64`

Converts radians to turns.

### Parameters
- `rad`: The value in radians to be converted to turns.

### Returns
- `f64`: The equivalent value in turns.

### Formula
`turn = rad / (2 * PI)`

## `turns_to_radians(turn: f64) -> f64`

Converts turns to radians.

### Parameters
- `turn`: The value in turns to be converted to radians.

### Returns
- `f64`: The equivalent value in radians.

### Formula
`rad = turn * 2 * PI`

## `degrees_to_turns(deg: f64) -> f64`

Converts degrees to turns.

### Parameters
- `deg`: The value in degrees to be converted to turns.

### Returns
- `f64`: The equivalent value in turns.

### Formula
`turn = deg / 360`

## `turns_to_degrees(turn: f64) -> f64`

Converts turns to degrees.

### Parameters
- `turn`: The value in turns to be converted to degrees.

### Returns
- `f64`: The equivalent value in degrees.

### Formula
`deg = turn * 360`

## `gradians_to_turns(grad: f64) -> f64`

Converts gradians to turns.

### Parameters
- `grad`: The value in gradians to be converted to turns.

### Returns
- `f64`: The equivalent value in turns.

### Formula
`turn = grad / 400`

## `turns_to_gradians(turn: f64) -> f64`

Converts turns to gradians.

### Parameters
- `turn`: The value in turns to be converted to gradians.

### Returns
- `f64`: The equivalent value in gradians.

### Formula
`grad = turn * 400`
