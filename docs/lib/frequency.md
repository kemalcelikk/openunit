# Frequency Module

The `frequency` module provides functions for converting between different units of frequency, such as hertz, kilohertz, megahertz, and gigahertz.

**Functions**

## `hertz_to_kilohertz(hz: usize) -> usize`

Converts hertz to kilohertz.

### Parameters
- `hz`: The value in hertz to be converted to kilohertz.

### Returns
- `usize`: The equivalent value in kilohertz.

### Formula
`kHz = Hz / 1000`

## `kilohertz_to_hertz(khz: usize) -> usize`

Converts kilohertz to hertz.

### Parameters
- `khz`: The value in kilohertz to be converted to hertz.

### Returns
- `usize`: The equivalent value in hertz.

### Formula
`Hz = kHz * 1000`

## `hertz_to_megahertz(hz: usize) -> usize`

Converts hertz to megahertz.

### Parameters
- `hz`: The value in hertz to be converted to megahertz.

### Returns
- `usize`: The equivalent value in megahertz.

### Formula
`MHz = Hz / 1,000,000`

## `megahertz_to_hertz(mhz: usize) -> usize`

Converts megahertz to hertz.

### Parameters
- `mhz`: The value in megahertz to be converted to hertz.

### Returns
- `usize`: The equivalent value in hertz.

### Formula
`Hz = MHz * 1,000,000`

## `hertz_to_gigahertz(hz: usize) -> usize`

Converts hertz to gigahertz.

### Parameters
- `hz`: The value in hertz to be converted to gigahertz.

### Returns
- `usize`: The equivalent value in gigahertz.

### Formula
`GHz = Hz / 1,000,000,000`

## `gigahertz_to_hertz(ghz: usize) -> usize`

Converts gigahertz to hertz.

### Parameters
- `ghz`: The value in gigahertz to be converted to hertz.

### Returns
- `usize`: The equivalent value in hertz.

### Formula
`Hz = GHz * 1,000,000,000`

## `kilohertz_to_megahertz(khz: usize) -> usize`

Converts kilohertz to megahertz.

### Parameters
- `khz`: The value in kilohertz to be converted to megahertz.

### Returns
- `usize`: The equivalent value in megahertz.

### Formula
`MHz = kHz / 1000`

## `megahertz_to_kilohertz(mhz: usize) -> usize`

Converts megahertz to kilohertz.

### Parameters
- `mhz`: The value in megahertz to be converted to kilohertz.

### Returns
- `usize`: The equivalent value in kilohertz.

### Formula
`kHz = MHz * 1000`

## `kilohertz_to_gigahertz(khz: usize) -> usize`

Converts kilohertz to gigahertz.

### Parameters
- `khz`: The value in kilohertz to be converted to gigahertz.

### Returns
- `usize`: The equivalent value in gigahertz.

### Formula
`GHz = kHz / 1,000,000`

## `gigahertz_to_kilohertz(ghz: usize) -> usize`

Converts gigahertz to kilohertz.

### Parameters
- `ghz`: The value in gigahertz to be converted to kilohertz.

### Returns
- `usize`: The equivalent value in kilohertz.

### Formula
`kHz = GHz * 1,000,000`

## `megahertz_to_gigahertz(mhz: usize) -> usize`

Converts megahertz to gigahertz.

### Parameters
- `mhz`: The value in megahertz to be converted to gigahertz.

### Returns
- `usize`: The equivalent value in gigahertz.

### Formula
`GHz = MHz / 1000`

## `gigahertz_to_megahertz(ghz: usize) -> usize`

Converts gigahertz to megahertz.

### Parameters
- `ghz`: The value in gigahertz to be converted to megahertz.

### Returns
- `usize`: The equivalent value in megahertz.

### Formula
`MHz = GHz * 1000`
