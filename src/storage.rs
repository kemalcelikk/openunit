/// Converts bytes to kilobytes.
///
/// Formula: KB = bytes / 1024
///
/// # Examples
///
/// ```
/// let bytes = 1024;
/// let kilobytes = bytes_to_kilobytes(bytes);
/// assert_eq!(kilobytes, 1);
/// ```
pub fn bytes_to_kilobytes(bytes: f64) -> f64 {
    return bytes / 1024.0;
}

/// Converts kilobytes to bytes.
///
/// Formula: bytes = KB * 1024
///
/// # Examples
///
/// ```
/// let kilobytes = 1.0;
/// let bytes = kilobytes_to_bytes(kilobytes);
/// assert_eq!(bytes, 1024.0);
/// ```
pub fn kilobytes_to_bytes(kb: f64) -> f64 {
    return kb * 1024.0;
}

/// Converts kilobytes to megabytes.
///
/// Formula: MB = KB / 1024
///
/// # Examples
///
/// ```
/// let kilobytes = 1024.0;
/// let megabytes = kilobytes_to_megabytes(kilobytes);
/// assert_eq!(megabytes, 1.0);
/// ```
pub fn kilobytes_to_megabytes(kb: f64) -> f64 {
    return kb / 1024.0;
}

/// Converts megabytes to kilobytes.
///
/// Formula: KB = MB * 1024
///
/// # Examples
///
/// ```
/// let megabytes = 1.0;
/// let kilobytes = megabytes_to_kilobytes(megabytes);
/// assert_eq!(kilobytes, 1024.0);
/// ```
pub fn megabytes_to_kilobytes(mb: f64) -> f64 {
    return mb * 1024.0;
}

/// Converts megabytes to gigabytes.
///
/// Formula: GB = MB / 1024
///
/// # Examples
///
/// ```
/// let megabytes = 1024.0;
/// let gigabytes = megabytes_to_gigabytes(megabytes);
/// assert_eq!(gigabytes, 1.0);
/// ```
pub fn megabytes_to_gigabytes(mb: f64) -> f64 {
    return mb / 1024.0;
}

/// Converts gigabytes to megabytes.
///
/// Formula: MB = GB * 1024
///
/// # Examples
///
/// ```
/// let gigabytes = 1.0;
/// let megabytes = gigabytes_to_megabytes(gigabytes);
/// assert_eq!(megabytes, 1024.0);
/// ```
pub fn gigabytes_to_megabytes(gb: f64) -> f64 {
    return gb * 1024.0;
}

/// Converts gigabytes to terabytes.
///
/// Formula: TB = GB / 1024
///
/// # Examples
///
/// ```
/// let gigabytes = 1024.0;
/// let terabytes = gigabytes_to_terabytes(gigabytes);
/// assert_eq!(terabytes, 1.0);
/// ```
pub fn gigabytes_to_terabytes(gb: f64) -> f64 {
    return gb / 1024.0;
}

/// Converts terabytes to gigabytes.
///
/// Formula: GB = TB * 1024
///
/// # Examples
///
/// ```
/// let terabytes = 1.0;
/// let gigabytes = terabytes_to_gigabytes(terabytes);
/// assert_eq!(gigabytes, 1024.0);
/// ```
pub fn terabytes_to_gigabytes(tb: f64) -> f64 {
    return tb * 1024.0;
}

/// Converts terabytes to petabytes.
///
/// Formula: PB = TB / 1024
///
/// # Examples
///
/// ```
/// let terabytes = 1024.0;
/// let petabytes = terabytes_to_petabytes(terabytes);
/// assert_eq!(petabytes, 1.0);
/// ```
pub fn terabytes_to_petabytes(tb: f64) -> f64 {
    return tb / 1024.0;
}

/// Converts petabytes to terabytes.
///
/// Formula: TB = PB * 1024
///
/// # Examples
///
/// ```
/// let petabytes = 1.0;
/// let terabytes = petabytes_to_terabytes(petabytes);
/// assert_eq!(terabytes, 1024.0);
/// ```
pub fn petabytes_to_terabytes(pb: f64) -> f64 {
    return pb * 1024.0;
}

/// Converts petabytes to exabytes.
///
/// Formula: EB = PB / 1024
///
/// # Examples
///
/// ```
/// let petabytes = 1024.0;
/// let exabytes = petabytes_to_exabytes(petabytes);
/// assert_eq!(exabytes, 1.0);
/// ```
pub fn petabytes_to_exabytes(pb: f64) -> f64 {
    return pb / 1024.0;
}

/// Converts exabytes to petabytes.
///
/// Formula: PB = EB * 1024
///
/// # Examples
///
/// ```
/// let exabytes = 1.0;
/// let petabytes = exabytes_to_petabytes(exabytes);
/// assert_eq!(petabytes, 1024.0);
/// ```
pub fn exabytes_to_petabytes(eb: f64) -> f64 {
    return eb * 1024.0;
}