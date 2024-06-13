/// Converts Hertz to Kilohertz.
///
/// Formula: kHz = Hz / 1000
///
/// # Examples
///
/// ```
/// let hertz = 1000;
/// let kilohertz = hertz_to_kilohertz(hertz);
/// assert_eq!(kilohertz, 1);
/// ```
pub fn hertz_to_kilohertz(hz: usize) -> usize {
    return hz / 1000;
}

/// Converts Kilohertz to Hertz.
///
/// Formula: Hz = kHz * 1000
///
/// # Examples
///
/// ```
/// let kilohertz = 1;
/// let hertz = kilohertz_to_hertz(kilohertz);
/// assert_eq!(hertz, 1000);
/// ```
pub fn kilohertz_to_hertz(khz: usize) -> usize {
    return khz * 1000;
}

/// Converts Hertz to Megahertz.
///
/// Formula: MHz = Hz / 1,000,000
///
/// # Examples
///
/// ```
/// let hertz = 1_000_000;
/// let megahertz = hertz_to_megahertz(hertz);
/// assert_eq!(megahertz, 1);
/// ```
pub fn hertz_to_megahertz(hz: usize) -> usize {
    return hz / 1_000_000;
}

/// Converts Megahertz to Hertz.
///
/// Formula: Hz = MHz * 1,000,000
///
/// # Examples
///
/// ```
/// let megahertz = 1;
/// let hertz = megahertz_to_hertz(megahertz);
/// assert_eq!(hertz, 1_000_000);
/// ```
pub fn megahertz_to_hertz(mhz: usize) -> usize {
    return mhz * 1_000_000;
}

/// Converts Hertz to Gigahertz.
///
/// Formula: GHz = Hz / 1,000,000,000
///
/// # Examples
///
/// ```
/// let hertz = 1_000_000_000;
/// let gigahertz = hertz_to_gigahertz(hertz);
/// assert_eq!(gigahertz, 1);
/// ```
pub fn hertz_to_gigahertz(hz: usize) -> usize {
    return hz / 1_000_000_000;
}

/// Converts Gigahertz to Hertz.
///
/// Formula: Hz = GHz * 1,000,000,000
///
/// # Examples
///
/// ```
/// let gigahertz = 1;
/// let hertz = gigahertz_to_hertz(gigahertz);
/// assert_eq!(hertz, 1_000_000_000);
/// ```
pub fn gigahertz_to_hertz(ghz: usize) -> usize {
    return ghz * 1_000_000_000;
}

/// Converts Kilohertz to Megahertz.
///
/// Formula: MHz = kHz / 1000
///
/// # Examples
///
/// ```
/// let kilohertz = 1000;
/// let megahertz = kilohertz_to_megahertz(kilohertz);
/// assert_eq!(megahertz, 1);
/// ```
pub fn kilohertz_to_megahertz(khz: usize) -> usize {
    return khz / 1000;
}

/// Converts Megahertz to Kilohertz.
///
/// Formula: kHz = MHz * 1000
///
/// # Examples
///
/// ```
/// let megahertz = 1;
/// let kilohertz = megahertz_to_kilohertz(megahertz);
/// assert_eq!(kilohertz, 1000);
/// ```
pub fn megahertz_to_kilohertz(mhz: usize) -> usize {
    return mhz * 1000;
}

/// Converts Kilohertz to Gigahertz.
///
/// Formula: GHz = kHz / 1,000,000
///
/// # Examples
///
/// ```
/// let kilohertz = 1_000_000;
/// let gigahertz = kilohertz_to_gigahertz(kilohertz);
/// assert_eq!(gigahertz, 1);
/// ```
pub fn kilohertz_to_gigahertz(khz: usize) -> usize {
    return khz / 1_000_000;
}

/// Converts Gigahertz to Kilohertz.
///
/// Formula: kHz = GHz * 1,000,000
///
/// # Examples
///
/// ```
/// let gigahertz = 1;
/// let kilohertz = gigahertz_to_kilohertz(gigahertz);
/// assert_eq!(kilohertz, 1_000_000);
/// ```
pub fn gigahertz_to_kilohertz(ghz: usize) -> usize {
    return ghz * 1_000_000;
}

/// Converts Megahertz to Gigahertz.
///
/// Formula: GHz = MHz / 1000
///
/// # Examples
///
/// ```
/// let megahertz = 1000;
/// let gigahertz = megahertz_to_gigahertz(megahertz);
/// assert_eq!(gigahertz, 1);
/// ```
pub fn megahertz_to_gigahertz(mhz: usize) -> usize {
    return mhz / 1000;
}

/// Converts Gigahertz to Megahertz.
///
/// Formula: MHz = GHz * 1000
///
/// # Examples
///
/// ```
/// let gigahertz = 1;
/// let megahertz = gigahertz_to_megahertz(gigahertz);
/// assert_eq!(megahertz, 1000);
/// ```
pub fn gigahertz_to_megahertz(ghz: usize) -> usize {
    return ghz * 1000;
}
