/// Converts square meters to square feet.
///
/// Formula: ft^2 = m^2 * 10.764
///
/// # Examples
///
/// ```
/// let square_meters = 1.0;
/// let square_feet = square_meters_to_square_feet(square_meters);
/// assert_eq!(square_feet, 10.764);
/// ```
pub fn square_meters_to_square_feet(m: f64) -> f64 {
    return m * 10.764;
}

/// Converts square feet to square meters.
///
/// Formula: m^2 = ft^2 / 10.764
///
/// # Examples
///
/// ```
/// let square_feet = 10.764;
/// let square_meters = square_feet_to_square_meters(square_feet);
/// assert_eq!(square_meters, 1.0);
/// ```
pub fn square_feet_to_square_meters(ft: f64) -> f64 {
    return ft / 10.764;
}

/// Converts square kilometers to square miles.
///
/// Formula: mi^2 = km^2 / 2.59
///
/// # Examples
///
/// ```
/// let square_kilometers = 1.0;
/// let square_miles = square_kilometers_to_square_miles(square_kilometers);
/// assert_eq!(square_miles, 0.386);
/// ```
pub fn square_kilometers_to_square_miles(km: f64) -> f64 {
    return km / 2.59;
}

/// Converts square miles to square kilometers.
///
/// Formula: km^2 = mi^2 * 2.59
///
/// # Examples
///
/// ```
/// let square_miles = 1.0;
/// let square_kilometers = square_miles_to_square_kilometers(square_miles);
/// assert_eq!(square_kilometers, 2.59);
/// ```
pub fn square_miles_to_square_kilometers(mi: f64) -> f64 {
    return mi * 2.59;
}

/// Converts square millimeters to square centimeters.
///
/// Formula: cm^2 = mm^2 / 100
///
/// # Examples
///
/// ```
/// let square_millimeters = 100.0;
/// let square_centimeters = square_millimeters_to_square_centimeters(square_millimeters);
/// assert_eq!(square_centimeters, 10.0);
/// ```
pub fn square_millimeters_to_square_centimeters(mm: f64) -> f64 {
    return mm / 100.0;
}

/// Converts square centimeters to square millimeters.
///
/// Formula: mm^2 = cm^2 * 100
///
/// # Examples
///
/// ```
/// let square_centimeters = 10.0;
/// let square_millimeters = square_centimeters_to_square_millimeters(square_centimeters);
/// assert_eq!(square_millimeters, 1000.0);
/// ```
pub fn square_centimeters_to_square_millimeters(cm: f64) -> f64 {
    return cm * 100.0;
}

/// Converts hectares to acres.
///
/// Formula: acres = hectares * 2.471
///
/// # Examples
///
/// ```
/// let hectares = 1.0;
/// let acres = hectares_to_acres(hectares);
/// assert_eq!(acres, 2.471);
/// ```
pub fn hectares_to_acres(hectares: f64) -> f64 {
    return hectares * 2.471;
}

/// Converts acres to hectares.
///
/// Formula: hectares = acres / 2.471
///
/// # Examples
///
/// ```
/// let acres = 1.0;
/// let hectares = acres_to_hectares(acres);
/// assert_eq!(hectares, 0.405);
/// ```
pub fn acres_to_hectares(acres: f64) -> f64 {
    return acres / 2.471;
}

/// Converts square yards to square feet.
///
/// Formula: ft^2 = yd^2 * 9
///
/// # Examples
///
/// ```
/// let square_yards = 1.0;
/// let square_feet = square_yards_to_square_feet(square_yards);
/// assert_eq!(square_feet, 9.0);
/// ```
pub fn square_yards_to_square_feet(yd: f64) -> f64 {
    return yd * 9.0;
}

/// Converts square feet to square yards.
///
/// Formula: yd^2 = ft^2 / 9
///
/// # Examples
///
/// ```
/// let square_feet = 9.0;
/// let square_yards = square_feet_to_square_yards(square_feet);
/// assert_eq!(square_yards, 1.0);
/// ```
pub fn square_feet_to_square_yards(ft: f64) -> f64 {
    return ft / 9.0;
}
