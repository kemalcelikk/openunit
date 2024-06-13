/// Converts Meters/s to Kilometers/h.
///
/// Formula: km/h = m/s * 3.6
///
/// # Examples
///
/// ```
/// let meters_per_second = 10.0;
/// let kilometers_per_hour = meters_per_second_to_kilometers_per_hour(meters_per_second);
/// assert_eq!(kilometers_per_hour, 36.0);
/// ```
pub fn meters_per_second_to_kilometers_per_hour(mps: f64) -> f64 {
    return mps * 3.6;
}

/// Converts Kilometers/h to Meters/s.
///
/// Formula: m/s = km/h / 3.6
///
/// # Examples
///
/// ```
/// let kilometers_per_hour = 36.0;
/// let meters_per_second = kilometers_per_hour_to_meters_per_second(kilometers_per_hour);
/// assert_eq!(meters_per_second, 10.0);
/// ```
pub fn kilometers_per_hour_to_meters_per_second(kph: f64) -> f64 {
    return kph / 3.6;
}

/// Converts Meters/s to Miles/h.
///
/// Formula: mph = m/s * 2.23694
///
/// # Examples
///
/// ```
/// let meters_per_second = 10.0;
/// let miles_per_hour = meters_per_second_to_miles_per_hour(meters_per_second);
/// assert_eq!(miles_per_hour, 22.3694);
/// ```
pub fn meters_per_second_to_miles_per_hour(mps: f64) -> f64 {
    return mps * 2.23694;
}

/// Converts Miles/h to Meters/s.
///
/// Formula: m/s = mph / 2.23694
///
/// # Examples
///
/// ```
/// let miles_per_hour = 22.3694;
/// let meters_per_second = miles_per_hour_to_meters_per_second(miles_per_hour);
/// assert_eq!(meters_per_second, 10.0);
/// ```
pub fn miles_per_hour_to_meters_per_second(mph: f64) -> f64 {
    return mph / 2.23694;
}

/// Converts Kilometers/h to Miles/h.
///
/// Formula: mph = km/h / 1.60934
///
/// # Examples
///
/// ```
/// let kilometers_per_hour = 36.0;
/// let miles_per_hour = kilometers_per_hour_to_miles_per_hour(kilometers_per_hour);
/// assert_eq!(miles_per_hour, 22.3694);
/// ```
pub fn kilometers_per_hour_to_miles_per_hour(kph: f64) -> f64 {
    return kph / 1.60934;
}

/// Converts Miles/h to Kilometers/h.
///
/// Formula: km/h = mph * 1.60934
///
/// # Examples
///
/// ```
/// let miles_per_hour = 22.3694;
/// let kilometers_per_hour = miles_per_hour_to_kilometers_per_hour(miles_per_hour);
/// assert_eq!(kilometers_per_hour, 36.0);
/// ```
pub fn miles_per_hour_to_kilometers_per_hour(mph: f64) -> f64 {
    return mph * 1.60934;
}

/// Converts Knot to Meters/s.
///
/// Formula: m/s = knot * 0.514444
///
/// # Examples
///
/// ```
/// let knot = 10.0;
/// let meters_per_second = knot_to_meters_per_second(knot);
/// assert_eq!(meters_per_second, 5.14444);
/// ```
pub fn knot_to_meters_per_second(knot: f64) -> f64 {
    return knot * 0.514444;
}

/// Converts Meters/s to Knot.
///
/// Formula: knot = m/s / 0.514444
///
/// # Examples
///
/// ```
/// let meters_per_second = 5.14444;
/// let knot = meters_per_second_to_knot(meters_per_second);
/// assert_eq!(knot, 10.0);
/// ```
pub fn meters_per_second_to_knot(mps: f64) -> f64 {
    return mps / 0.514444;
}

/// Converts Feet/s to Meters/s.
///
/// Formula: m/s = ft/s * 0.3048
///
/// # Examples
///
/// ```
/// let feet_per_second = 10.0;
/// let meters_per_second = feet_per_second_to_meters_per_second(feet_per_second);
/// assert_eq!(meters_per_second, 3.048);
/// ```
pub fn feet_per_second_to_meters_per_second(fps: f64) -> f64 {
    return fps * 0.3048;
}

/// Converts Meters/s to Feet/s.
///
/// Formula: ft/s = m/s / 0.3048
///
/// # Examples
///
/// ```
/// let meters_per_second = 3.048;
/// let feet_per_second = meters_per_second_to_feet_per_second(meters_per_second);
/// assert_eq!(feet_per_second, 10.0);
/// ```
pub fn meters_per_second_to_feet_per_second(mps: f64) -> f64 {
    return mps / 0.3048;
}

/// Converts Knot to Kilometers/h.
///
/// Formula: km/h = knot * 1.852
///
/// # Examples
///
/// ```
/// let knot = 10.0;
/// let kilometers_per_hour = knot_to_kilometers_per_hour(knot);
/// assert_eq!(kilometers_per_hour, 18.52);
/// ```
pub fn knot_to_kilometers_per_hour(knot: f64) -> f64 {
    return knot * 1.852;
}

/// Converts Kilometers/h to Knot.
///
/// Formula: knot = km/h / 1.852
///
/// # Examples
///
/// ```
/// let kilometers_per_hour = 18.52;
/// let knot = kilometers_per_hour_to_knot(kilometers_per_hour);
/// assert_eq!(knot, 10.0);
/// ```
pub fn kilometers_per_hour_to_knot(kph: f64) -> f64 {
    return kph / 1.852;
}

/// Converts Feet/s to Miles/h.
///
/// Formula: mph = ft/s * 0.681818
///
/// # Examples
///
/// ```
/// let feet_per_second = 10.0;
/// let miles_per_hour = feet_per_second_to_miles_per_hour(feet_per_second);
/// assert_eq!(miles_per_hour, 6.81818);
/// ```
pub fn feet_per_second_to_miles_per_hour(fps: f64) -> f64 {
    return fps * 0.681818;
}

/// Converts Miles/h to Feet/s.
///
/// Formula: ft/s = mph / 0.681818
///
/// # Examples
///
/// ```
/// let miles_per_hour = 6.81818;
/// let feet_per_second = miles_per_hour_to_feet_per_second(miles_per_hour);
/// assert_eq!(feet_per_second, 10.0);
/// ```
pub fn miles_per_hour_to_feet_per_second(mph: f64) -> f64 {
    return mph / 0.681818;
}
