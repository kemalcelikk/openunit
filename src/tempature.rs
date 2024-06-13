/// Converts Celsius to Fahrenheit.
///
/// Formula: F = (C * 9/5) + 32
///
/// # Examples
///
/// ```
/// let celsius = 0;
/// let fahrenheit = celsius_to_fahrenheit(celsius);
/// assert_eq!(fahrenheit, 32);
/// ```
pub fn celsius_to_fahrenheit(c: usize) -> usize {
    return (c * 9 / 5) + 32;
}

/// Converts Fahrenheit to Celsius.
///
/// Formula: C = (F - 32) * 5/9
///
/// # Examples
///
/// ```
/// let fahrenheit = 32;
/// let celsius = fahrenheit_to_celsius(fahrenheit);
/// assert_eq!(celsius, 0);
/// ```
pub fn fahrenheit_to_celsius(f: usize) -> usize {
    return (f - 32) * 5/9;
}

/// Converts Kelvin to Celsius.
///
/// Formula: C = K - 273.15
///
/// # Examples
///
/// ```
/// let kelvin = 273.15;
/// let celsius = kelvin_to_celsius(kelvin);
/// assert_eq!(celsius, 0);
/// ```
pub fn kelvin_to_celsius(k: f64) -> usize {
    return (k - 273.15) as usize;
}

/// Converts Kelvin to Fahrenheit.
///
/// Formula: F = (K - 273.15) * 9/5 + 32
///
/// # Examples
///
/// ```
/// let kelvin = 273.15;
/// let fahrenheit = kelvin_to_fahrenheit(kelvin);
/// assert_eq!(fahrenheit, 32);
/// ```
pub fn kelvin_to_fahrenheit(k: f64) -> usize {
    let celsius = (k - 273.15) as usize;
    return celsius_to_fahrenheit(celsius);
}

/// Converts Celsius to Kelvin.
///
/// Formula: K = C + 273.15
///
/// # Examples
///
/// ```
/// let celsius = 0;
/// let kelvin = celsius_to_kelvin(celsius);
/// assert_eq!(kelvin, 273.15);
/// ```
pub fn celsius_to_kelvin(c: f64) -> usize {
    return (c + 273.15) as usize;
}

/// Converts Fahrenheit to Kelvin
/// 
/// Formula: C = (F - 32) * 5/9,
///          K = C + 273.15
/// 
/// # Examples
/// 
/// ```
/// let fahrenheit = 0;
/// let kelvin = fahrenheit_to_kelvin(fahrenheit);
/// assert_eq!(kelvin, 255.37);
/// ```
pub fn fahrenheit_to_kelvin(f: f64) -> usize {
    let celsius = fahrenheit_to_celsius(f as usize) as f64;
    return celsius_to_kelvin(celsius);
}