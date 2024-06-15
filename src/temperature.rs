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
    return (f - 32) * 5 / 9;
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

/// Converts Fahrenheit to Rankine.
///
/// Formula: R = F + 459.67
///
/// # Examples
///
/// ```
/// let fahrenheit = 32.0;
/// let rankine = fahrenheit_to_rankine(fahrenheit);
/// assert_eq!(rankine, 491.67);
/// ```
pub fn fahrenheit_to_rankine(f: f64) -> f64 {
    return f + 459.67;
}

/// Converts Rankine to Fahrenheit.
///
/// Formula: F = R - 459.67
///
/// # Examples
///
/// ```
/// let rankine = 491.67;
/// let fahrenheit = rankine_to_fahrenheit(rankine);
/// assert_eq!(fahrenheit, 32.0);
/// ```
pub fn rankine_to_fahrenheit(r: f64) -> f64 {
    return r - 459.67;
}

/// Converts Celsius to Rankine.
///
/// Formula: R = (C + 273.15) * 9/5
///
/// # Examples
///
/// ```
/// let celsius = 0.0;
/// let rankine = celsius_to_rankine(celsius);
/// assert_eq!(rankine, 491.67);
/// ```
pub fn celsius_to_rankine(c: f64) -> f64 {
    return (c + 273.15) * 9.0 / 5.0;
}

/// Converts Rankine to Celsius.
///
/// Formula: C = (R - 491.67) * 5/9
///
/// # Examples
///
/// ```
/// let rankine = 491.67;
/// let celsius = rankine_to_celsius(rankine);
/// assert_eq!(celsius, 0.0);
/// ```
pub fn rankine_to_celsius(r: f64) -> f64 {
    return (r - 491.67) * 5.0 / 9.0;
}

/// Converts Kelvin to Rankine.
///
/// Formula: R = K * 9/5
///
/// # Examples
///
/// ```
/// let kelvin = 273.15;
/// let rankine = kelvin_to_rankine(kelvin);
/// assert_eq!(rankine, 491.67);
/// ```
pub fn kelvin_to_rankine(k: f64) -> f64 {
    return k * 9.0 / 5.0;
}

/// Converts Rankine to Kelvin.
///
/// Formula: K = R * 5/9
///
/// # Examples
///
/// ```
/// let rankine = 491.67;
/// let kelvin = rankine_to_kelvin(rankine);
/// assert_eq!(kelvin, 273.15);
/// ```
pub fn rankine_to_kelvin(r: f64) -> f64 {
    return r * 5.0 / 9.0;
}
