/// Converts Pascals to Kilopascals.
///
/// Formula: kPa = Pa / 1000
///
/// # Examples
///
/// ```
/// let pascals = 1000;
/// let kilopascals = pascals_to_kilopascals(pascals);
/// assert_eq!(kilopascals, 1);
/// ```
pub fn pascals_to_kilopascals(pa: f64) -> f64 {
    return pa / 1000.0;
}

/// Converts Kilopascals to Pascals.
///
/// Formula: Pa = kPa * 1000
///
/// # Examples
///
/// ```
/// let kilopascals = 1;
/// let pascals = kilopascals_to_pascals(kilopascals);
/// assert_eq!(pascals, 1000);
/// ```
pub fn kilopascals_to_pascals(kpa: f64) -> f64 {
    return kpa * 1000.0;
}

/// Converts Pascals to Atmospheres.
///
/// Formula: atm = Pa / 101325
///
/// # Examples
///
/// ```
/// let pascals = 101325;
/// let atmospheres = pascals_to_atmospheres(pascals);
/// assert_eq!(atmospheres, 1);
/// ```
pub fn pascals_to_atmospheres(pa: f64) -> f64 {
    return pa / 101325.0;
}

/// Converts Atmospheres to Pascals.
///
/// Formula: Pa = atm * 101325
///
/// # Examples
///
/// ```
/// let atmospheres = 1;
/// let pascals = atmospheres_to_pascals(atmospheres);
/// assert_eq!(pascals, 101325);
/// ```
pub fn atmospheres_to_pascals(atm: f64) -> f64 {
    return atm * 101325.0;
}

/// Converts Pascals to Bars.
///
/// Formula: bar = Pa / 100000
///
/// # Examples
///
/// ```
/// let pascals = 100000;
/// let bars = pascals_to_bars(pascals);
/// assert_eq!(bars, 1);
/// ```
pub fn pascals_to_bars(pa: f64) -> f64 {
    return pa / 100000.0;
}

/// Converts Bars to Pascals.
///
/// Formula: Pa = bar * 100000
///
/// # Examples
///
/// ```
/// let bars = 1;
/// let pascals = bars_to_pascals(bars);
/// assert_eq!(pascals, 100000);
/// ```
pub fn bars_to_pascals(bar: f64) -> f64 {
    return bar * 100000.0;
}

/// Converts Pascals to Pounds per Square Inch (PSI).
///
/// Formula: psi = Pa / 6895
///
/// # Examples
///
/// ```
/// let pascals = 6895;
/// let psi = pascals_to_psi(pascals);
/// assert_eq!(psi, 1);
/// ```
pub fn pascals_to_psi(pa: f64) -> f64 {
    return pa / 6895.0;
}

/// Converts Pounds per Square Inch (PSI) to Pascals.
///
/// Formula: Pa = psi * 6895
///
/// # Examples
///
/// ```
/// let psi = 1;
/// let pascals = psi_to_pascals(psi);
/// assert_eq!(pascals, 6895);
/// ```
pub fn psi_to_pascals(psi: f64) -> f64 {
    return psi * 6895.0;
}

/// Converts Kilopascals to Atmospheres.
///
/// Formula: atm = kPa / 101.325
///
/// # Examples
///
/// ```
/// let kilopascals = 101.325;
/// let atmospheres = kilopascals_to_atmospheres(kilopascals);
/// assert_eq!(atmospheres, 1);
/// ```
pub fn kilopascals_to_atmospheres(kpa: f64) -> f64 {
    return kpa / 101.325;
}

/// Converts Atmospheres to Kilopascals.
///
/// Formula: kPa = atm * 101.325
///
/// # Examples
///
/// ```
/// let atmospheres = 1;
/// let kilopascals = atmospheres_to_kilopascals(atmospheres);
/// assert_eq!(kilopascals, 101.325);
/// ```
pub fn atmospheres_to_kilopascals(atm: f64) -> f64 {
    return atm * 101.325;
}

/// Converts Bars to Atmospheres.
///
/// Formula: atm = bar / 1.01325
///
/// # Examples
///
/// ```
/// let bars = 1.01325;
/// let atmospheres = bars_to_atmospheres(bars);
/// assert_eq!(atmospheres, 1);
/// ```
pub fn bars_to_atmospheres(bar: f64) -> f64 {
    return bar / 1.01325;
}

/// Converts Atmospheres to Bars.
///
/// Formula: bar = atm * 1.01325
///
/// # Examples
///
/// ```
/// let atmospheres = 1;
/// let bars = atmospheres_to_bars(atmospheres);
/// assert_eq!(bars, 1.01325);
/// ```
pub fn atmospheres_to_bars(atm: f64) -> f64 {
    return atm * 1.01325;
}

/// Converts Pounds per Square Inch (PSI) to Kilopascals.
///
/// Formula: kPa = psi * 6.894757
///
/// # Examples
///
/// ```
/// let psi = 1;
/// let kilopascals = psi_to_kilopascals(psi);
/// assert_eq!(kilopascals, 6.894757);
/// ```
pub fn psi_to_kilopascals(psi: f64) -> f64 {
    return psi * 6.894757;
}

/// Converts Kilopascals to Pounds per Square Inch (PSI).
///
/// Formula: psi = kPa / 6.894757
///
/// # Examples
///
/// ```
/// let kilopascals = 6.894757;
/// let psi = kilopascals_to_psi(kilopascals);
/// assert_eq!(psi, 1);
/// ```
pub fn kilopascals_to_psi(kpa: f64) -> f64 {
    return kpa / 6.894757;
}
