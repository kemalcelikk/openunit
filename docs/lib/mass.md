# Mass Module

The `mass` module provides functions for converting between different units of mass measurements, such as kilograms, grams, pounds, ounces, milligrams, and tonnes.

**Functions**

## `kilogram_to_gram(kg: f64) -> f64`

Converts kilograms to grams.

### Parameters
- `kg`: The mass value in kilograms to be converted to grams.

### Returns
- `f64`: The equivalent value in grams.

### Formula
`g = kg * 1000`

## `gram_to_kilogram(g: f64) -> f64`

Converts grams to kilograms.

### Parameters
- `g`: The mass value in grams to be converted to kilograms.

### Returns
- `f64`: The equivalent value in kilograms.

### Formula
`kg = g / 1000`

## `kilogram_to_milligram(kg: f64) -> f64`

Converts kilograms to milligrams.

### Parameters
- `kg`: The mass value in kilograms to be converted to milligrams.

### Returns
- `f64`: The equivalent value in milligrams.

### Formula
`mg = kg * 1e+6`

## `milligram_to_kilogram(mg: f64) -> f64`

Converts milligrams to kilograms.

### Parameters
- `mg`: The mass value in milligrams to be converted to kilograms.

### Returns
- `f64`: The equivalent value in kilograms.

### Formula
`kg = mg / 1e+6`

## `kilogram_to_tonne(kg: f64) -> f64`

Converts kilograms to tonnes.

### Parameters
- `kg`: The mass value in kilograms to be converted to tonnes.

### Returns
- `f64`: The equivalent value in tonnes.

### Formula
`t = kg / 1000`

## `tonne_to_kilogram(t: f64) -> f64`

Converts tonnes to kilograms.

### Parameters
- `t`: The mass value in tonnes to be converted to kilograms.

### Returns
- `f64`: The equivalent value in kilograms.

### Formula
`kg = t * 1000`

## `pound_to_kilogram(lb: f64) -> f64`

Converts pounds to kilograms.

### Parameters
- `lb`: The mass value in pounds to be converted to kilograms.

### Returns
- `f64`: The equivalent value in kilograms.

### Formula
`kg = lb * 0.45359237`

## `kilogram_to_pound(kg: f64) -> f64`

Converts kilograms to pounds.

### Parameters
- `kg`: The mass value in kilograms to be converted to pounds.

### Returns
- `f64`: The equivalent value in pounds.

### Formula
`lb = kg / 0.45359237`

## `ounce_to_gram(oz: f64) -> f64`

Converts ounces to grams.

### Parameters
- `oz`: The mass value in ounces to be converted to grams.

### Returns
- `f64`: The equivalent value in grams.

### Formula
`g = oz * 28.3495231`

## `gram_to_ounce(g: f64) -> f64`

Converts grams to ounces.

### Parameters
- `g`: The mass value in grams to be converted to ounces.

### Returns
- `f64`: The equivalent value in ounces.

### Formula
`oz = g / 28.3495231`

## `kilogram_to_tonne(kg: f64) -> f64`

Converts kilograms to tonnes.

### Parameters
- `kg`: The mass value in kilograms to be converted to tonnes.

### Returns
- `f64`: The equivalent value in tonnes.

### Formula
`t = kg / 1000`

## `tonne_to_kilogram(t: f64) -> f64`

Converts tonnes to kilograms.

### Parameters
- `t`: The mass value in tonnes to be converted to kilograms.

### Returns
- `f64`: The equivalent value in kilograms.

### Formula
`kg = t * 1000`

## `pound_to_kilogram(lb: f64) -> f64`

Converts pounds to kilograms.

### Parameters
- `lb`: The mass value in pounds to be converted to kilograms.

### Returns
- `f64`: The equivalent value in kilograms.

### Formula
`kg = lb * 0.45359237`

## `kilogram_to_pound(kg: f64) -> f64`

Converts kilograms to pounds.

### Parameters
- `kg`: The mass value in kilograms to be converted to pounds.

### Returns
- `f64`: The equivalent value in pounds.

### Formula
`lb = kg / 0.45359237`

## `ounce_to_gram(oz: f64) -> f64`

Converts ounces to grams.

### Parameters
- `oz`: The mass value in ounces to be converted to grams.

### Returns
- `f64`: The equivalent value in grams.

### Formula
`g = oz * 28.3495231`

## `gram_to_ounce(g: f64) -> f64`

Converts grams to ounces.

### Parameters
- `g`: The mass value in grams to be converted to ounces.

### Returns
- `f64`: The equivalent value in ounces.

### Formula
`oz = g / 28.3495231`

## `pound_to_ounce(lb: f64) -> f64`

Converts pounds to ounces.

### Parameters
- `lb`: The mass value in pounds to be converted to ounces.

### Returns
- `f64`: The equivalent value in ounces.

### Formula
`oz = lb * 16`

## `ounce_to_pound(oz: f64) -> f64`

Converts ounces to pounds.

### Parameters
- `oz`: The mass value in ounces to be converted to pounds.

### Returns
- `f64`: The equivalent value in pounds.

### Formula
`lb = oz / 16`

## `gram_to_milligram(g: f64) -> f64`

Converts grams to milligrams.

### Parameters
- `g`: The mass value in grams to be converted to milligrams.

### Returns
- `f64`: The equivalent value in milligrams.

### Formula
`mg = g * 1000`

## `milligram_to_gram(mg: f64) -> f64`

Converts milligrams to grams.

### Parameters
- `mg`: The mass value in milligrams to be converted to grams.

### Returns
- `f64`: The equivalent value in grams.

### Formula
`g = mg / 1000`

## `tonne_to_pound(t: f64) -> f64`

Converts tonnes to pounds.

### Parameters
- `t`: The mass value in tonnes to be converted to pounds.

### Returns
- `f64`: The equivalent value in pounds.

### Formula
`lb = t * 2204.62`

## `pound_to_tonne(lb: f64) -> f64`

Converts pounds to tonnes.

### Parameters
- `lb`: The mass value in pounds to be converted to tonnes.

### Returns
- `f64`: The equivalent value in tonnes.

### Formula
`t = lb / 2204.62`

## `milligram_to_pound(mg: f64) -> f64`

Converts milligrams to pounds.

### Parameters
- `mg`: The mass value in milligrams to be converted to pounds.

### Returns
- `f64`: The equivalent value in pounds.

### Formula
`lb = mg * 0.00000220462`

## `pound_to_milligram(lb: f64) -> f64`

Converts pounds to milligrams.

### Parameters
- `lb`: The mass value in pounds to be converted to milligrams.

### Returns
- `f64`: The equivalent value in milligrams.

### Formula
`mg = lb / 0.00000220462`

## `milligram_to_ounce(mg: f64) -> f64`

Converts milligrams to ounces.

### Parameters
- `mg`: The mass value in milligrams to be converted to ounces.

### Returns
- `f64`: The equivalent value in ounces.

### Formula
`oz = mg * 0.000035274`

## `ounce_to_milligram(oz: f64) -> f64`

Converts ounces to milligrams.

### Parameters
- `oz`: The mass value in ounces to be converted to milligrams.

### Returns
- `f64`: The equivalent value in milligrams.

### Formula
`mg = oz / 0.000035274`
