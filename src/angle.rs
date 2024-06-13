/// Converts Degrees to Radians.
///
/// Formula: rad = deg * PI / 180
///
/// # Examples
///
/// ```
/// let degrees = 180;
/// let radians = degrees_to_radians(degrees);
/// assert_eq!(radians, 3.141592653589793);
/// ```
pub fn degrees_to_radians(deg: f64) -> f64 {
    return deg * std::f64::consts::PI / 180.0;
}

/// Converts Radians to Degrees.
///
/// Formula: deg = rad * 180 / PI
///
/// # Examples
///
/// ```
/// let radians = std::f64::consts::PI;
/// let degrees = radians_to_degrees(radians);
/// assert_eq!(degrees, 180);
/// ```
pub fn radians_to_degrees(rad: f64) -> f64 {
    return rad * 180.0 / std::f64::consts::PI;
}

/// Converts Degrees to Gradians.
///
/// Formula: grad = deg * 200 / 180
///
/// # Examples
///
/// ```
/// let degrees = 180;
/// let gradians = degrees_to_gradians(degrees);
/// assert_eq!(gradians, 200);
/// ```
pub fn degrees_to_gradians(deg: f64) -> f64 {
    return deg * 200.0 / 180.0;
}

/// Converts Gradians to Degrees.
///
/// Formula: deg = grad * 180 / 200
///
/// # Examples
///
/// ```
/// let gradians = 200;
/// let degrees = gradians_to_degrees(gradians);
/// assert_eq!(degrees, 180);
/// ```
pub fn gradians_to_degrees(grad: f64) -> f64 {
    return grad * 180.0 / 200.0;
}

/// Converts Radians to Gradians.
///
/// Formula: grad = rad * 200 / PI
///
/// # Examples
///
/// ```
/// let radians = std::f64::consts::PI;
/// let gradians = radians_to_gradians(radians);
/// assert_eq!(gradians, 200);
/// ```
pub fn radians_to_gradians(rad: f64) -> f64 {
    return rad * 200.0 / std::f64::consts::PI;
}

/// Converts Gradians to Radians.
///
/// Formula: rad = grad * PI / 200
///
/// # Examples
///
/// ```
/// let gradians = 200;
/// let radians = gradians_to_radians(gradians);
/// assert_eq!(radians, std::f64::consts::PI);
/// ```
pub fn gradians_to_radians(grad: f64) -> f64 {
    return grad * std::f64::consts::PI / 200.0;
}

/// Converts Radians to Turns.
///
/// Formula: turn = rad / (2 * PI)
///
/// # Examples
///
/// ```
/// let radians = std::f64::consts::PI;
/// let turns = radians_to_turns(radians);
/// assert_eq!(turns, 0.5);
/// ```
pub fn radians_to_turns(rad: f64) -> f64 {
    return rad / (2.0 * std::f64::consts::PI);
}

/// Converts Turns to Radians.
///
/// Formula: rad = turn * 2 * PI
///
/// # Examples
///
/// ```
/// let turns = 0.5;
/// let radians = turns_to_radians(turns);
/// assert_eq!(radians, std::f64::consts::PI);
/// ```
pub fn turns_to_radians(turn: f64) -> f64 {
    return turn * 2.0 * std::f64::consts::PI;
}

/// Converts Degrees to Turns.
///
/// Formula: turn = deg / 360
///
/// # Examples
///
/// ```
/// let degrees = 360;
/// let turns = degrees_to_turns(degrees);
/// assert_eq!(turns, 1);
/// ```
pub fn degrees_to_turns(deg: f64) -> f64 {
    return deg / 360.0;
}

/// Converts Turns to Degrees.
///
/// Formula: deg = turn * 360
///
/// # Examples
///
/// ```
/// let turns = 1;
/// let degrees = turns_to_degrees(turns);
/// assert_eq!(degrees, 360);
/// ```
pub fn turns_to_degrees(turn: f64) -> f64 {
    return turn * 360.0;
}

/// Converts Gradians to Turns.
///
/// Formula: turn = grad / 400
///
/// # Examples
///
/// ```
/// let gradians = 400;
/// let turns = gradians_to_turns(gradians);
/// assert_eq!(turns, 1);
/// ```
pub fn gradians_to_turns(grad: f64) -> f64 {
    return grad / 400.0;
}

/// Converts Turns to Gradians.
///
/// Formula: grad = turn * 400
///
/// # Examples
///
/// ```
/// let turns = 1;
/// let gradians = turns_to_gradians(turns);
/// assert_eq!(gradians, 400);
/// ```
pub fn turns_to_gradians(turn: f64) -> f64 {
    return turn * 400.0;
}
