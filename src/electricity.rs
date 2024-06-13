/// Converts Amperes to Milliamperes.
///
/// Formula: mA = A * 1000
///
/// # Examples
///
/// ```
/// let amperes = 1;
/// let milliamperes = amperes_to_milliamperes(amperes);
/// assert_eq!(milliamperes, 1000);
/// ```
pub fn amperes_to_milliamperes(a: f64) -> f64 {
    return a * 1000.0;
}

/// Converts Milliamperes to Amperes.
///
/// Formula: A = mA / 1000
///
/// # Examples
///
/// ```
/// let milliamperes = 1000;
/// let amperes = milliamperes_to_amperes(milliamperes);
/// assert_eq!(amperes, 1);
/// ```
pub fn milliamperes_to_amperes(ma: f64) -> f64 {
    return ma / 1000.0;
}

/// Converts Amperes to Microamperes.
///
/// Formula: μA = A * 1_000_000
///
/// # Examples
///
/// ```
/// let amperes = 1;
/// let microamperes = amperes_to_microamperes(amperes);
/// assert_eq!(microamperes, 1_000_000);
/// ```
pub fn amperes_to_microamperes(a: f64) -> f64 {
    return a * 1_000_000.0;
}

/// Converts Microamperes to Amperes.
///
/// Formula: A = μA / 1_000_000
///
/// # Examples
///
/// ```
/// let microamperes = 1_000_000;
/// let amperes = microamperes_to_amperes(microamperes);
/// assert_eq!(amperes, 1);
/// ```
pub fn microamperes_to_amperes(ua: f64) -> f64 {
    return ua / 1_000_000.0;
}

/// Converts Amperes to Kilopicoamperes.
///
/// Formula: kA = A * 1e12
///
/// # Examples
///
/// ```
/// let amperes = 1;
/// let kilopicoamperes = amperes_to_kilopicoamperes(amperes);
/// assert_eq!(kilopicoamperes, 1e12);
/// ```
pub fn amperes_to_kilopicoamperes(a: f64) -> f64 {
    return a * 1e12;
}

/// Converts Kilopicoamperes to Amperes.
///
/// Formula: A = kA / 1e12
///
/// # Examples
///
/// ```
/// let kilopicoamperes = 1e12;
/// let amperes = kilopicoamperes_to_amperes(kilopicoamperes);
/// assert_eq!(amperes, 1);
/// ```
pub fn kilopicoamperes_to_amperes(ka: f64) -> f64 {
    return ka / 1e12;
}

/// Converts Amperes to Kiloamperes.
///
/// Formula: kA = A / 1000
///
/// # Examples
///
/// ```
/// let amperes = 1000;
/// let kiloamperes = amperes_to_kiloamperes(amperes);
/// assert_eq!(kiloamperes, 1);
/// ```
pub fn amperes_to_kiloamperes(a: f64) -> f64 {
    return a / 1000.0;
}

/// Converts Kiloamperes to Amperes.
///
/// Formula: A = kA * 1000
///
/// # Examples
///
/// ```
/// let kiloamperes = 1;
/// let amperes = kiloamperes_to_amperes(kiloamperes);
/// assert_eq!(amperes, 1000);
/// ```
pub fn kiloamperes_to_amperes(ka: f64) -> f64 {
    return ka * 1000.0;
}

/// Converts Amperes to Megaamperes.
///
/// Formula: MA = A / 1e6
///
/// # Examples
///
/// ```
/// let amperes = 1_000_000;
/// let megaamperes = amperes_to_megaamperes(amperes);
/// assert_eq!(megaamperes, 1);
/// ```
pub fn amperes_to_megaamperes(a: f64) -> f64 {
    return a / 1e6;
}

/// Converts Megaamperes to Amperes.
///
/// Formula: A = MA * 1e6
///
/// # Examples
///
/// ```
/// let megaamperes = 1;
/// let amperes = megaamperes_to_amperes(megaamperes);
/// assert_eq!(amperes, 1_000_000);
/// ```
pub fn megaamperes_to_amperes(ma: f64) -> f64 {
    return ma * 1e6;
}
