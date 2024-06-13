/// Converts Seconds to Minutes.
///
/// Formula: min = s / 60
///
/// # Examples
///
/// ```
/// let seconds = 60;
/// let minutes = seconds_to_minutes(seconds);
/// assert_eq!(minutes, 1);
/// ```
pub fn seconds_to_minutes(s: f64) -> f64 {
    return s / 60.0;
}

/// Converts Minutes to Seconds.
///
/// Formula: s = min * 60
///
/// # Examples
///
/// ```
/// let minutes = 1;
/// let seconds = minutes_to_seconds(minutes);
/// assert_eq!(seconds, 60);
/// ```
pub fn minutes_to_seconds(min: f64) -> f64 {
    return min * 60.0;
}

/// Converts Seconds to Hours.
///
/// Formula: h = s / 3600
///
/// # Examples
///
/// ```
/// let seconds = 3600;
/// let hours = seconds_to_hours(seconds);
/// assert_eq!(hours, 1);
/// ```
pub fn seconds_to_hours(s: f64) -> f64 {
    return s / 3600.0;
}

/// Converts Hours to Seconds.
///
/// Formula: s = h * 3600
///
/// # Examples
///
/// ```
/// let hours = 1;
/// let seconds = hours_to_seconds(hours);
/// assert_eq!(seconds, 3600);
/// ```
pub fn hours_to_seconds(h: f64) -> f64 {
    return h * 3600.0;
}

/// Converts Seconds to Days.
///
/// Formula: d = s / 86400
///
/// # Examples
///
/// ```
/// let seconds = 86400;
/// let days = seconds_to_days(seconds);
/// assert_eq!(days, 1);
/// ```
pub fn seconds_to_days(s: f64) -> f64 {
    return s / 86400.0;
}

/// Converts Days to Seconds.
///
/// Formula: s = d * 86400
///
/// # Examples
///
/// ```
/// let days = 1;
/// let seconds = days_to_seconds(days);
/// assert_eq!(seconds, 86400);
/// ```
pub fn days_to_seconds(d: f64) -> f64 {
    return d * 86400.0;
}

/// Converts Seconds to Weeks.
///
/// Formula: w = s / 604800
///
/// # Examples
///
/// ```
/// let seconds = 604800;
/// let weeks = seconds_to_weeks(seconds);
/// assert_eq!(weeks, 1);
/// ```
pub fn seconds_to_weeks(s: f64) -> f64 {
    return s / 604800.0;
}

/// Converts Weeks to Seconds.
///
/// Formula: s = w * 604800
///
/// # Examples
///
/// ```
/// let weeks = 1;
/// let seconds = weeks_to_seconds(weeks);
/// assert_eq!(seconds, 604800);
/// ```
pub fn weeks_to_seconds(w: f64) -> f64 {
    return w * 604800.0;
}

/// Converts Seconds to Months.
///
/// Formula: mo = s / 2.628e+6
///
/// # Examples
///
/// ```
/// let seconds = 2.628e+6;
/// let months = seconds_to_months(seconds);
/// assert_eq!(months, 1);
/// ```
pub fn seconds_to_months(s: f64) -> f64 {
    return s / 2.628e+6;
}

/// Converts Months to Seconds.
///
/// Formula: s = mo * 2.628e+6
///
/// # Examples
///
/// ```
/// let months = 1;
/// let seconds = months_to_seconds(months);
/// assert_eq!(seconds, 2.628e+6);
/// ```
pub fn months_to_seconds(mo: f64) -> f64 {
    return mo * 2.628e+6;
}

/// Converts Seconds to Years.
///
/// Formula: y = s / 3.154e+7
///
/// # Examples
///
/// ```
/// let seconds = 3.154e+7;
/// let years = seconds_to_years(seconds);
/// assert_eq!(years, 1);
/// ```
pub fn seconds_to_years(s: f64) -> f64 {
    return s / 3.154e+7;
}

/// Converts Years to Seconds.
///
/// Formula: s = y * 3.154e+7
///
/// # Examples
///
/// ```
/// let years = 1;
/// let seconds = years_to_seconds(years);
/// assert_eq!(seconds, 3.154e+7);
/// ```
pub fn years_to_seconds(y: f64) -> f64 {
    return y * 3.154e+7;
}

/// Converts Minutes to Hours.
///
/// Formula: h = min / 60
///
/// # Examples
///
/// ```
/// let minutes = 60;
/// let hours = minutes_to_hours(minutes);
/// assert_eq!(hours, 1);
/// ```
pub fn minutes_to_hours(min: f64) -> f64 {
    return min / 60.0;
}

/// Converts Hours to Minutes.
///
/// Formula: min = h * 60
///
/// # Examples
///
/// ```
/// let hours = 1;
/// let minutes = hours_to_minutes(hours);
/// assert_eq!(minutes, 60);
/// ```
pub fn hours_to_minutes(h: f64) -> f64 {
    return h * 60.0;
}

/// Converts Minutes to Days.
///
/// Formula: d = min / 1440
///
/// # Examples
///
/// ```
/// let minutes = 1440;
/// let days = minutes_to_days(minutes);
/// assert_eq!(days, 1);
/// ```
pub fn minutes_to_days(min: f64) -> f64 {
    return min / 1440.0;
}

/// Converts Days to Minutes.
///
/// Formula: min = d * 1440
///
/// # Examples
///
/// ```
/// let days = 1;
/// let minutes = days_to_minutes(days);
/// assert_eq!(minutes, 1440);
/// ```
pub fn days_to_minutes(d: f64) -> f64 {
    return d * 1440.0;
}

/// Converts Hours to Days.
///
/// Formula: d = h / 24
///
/// # Examples
///
/// ```
/// let hours = 24;
/// let days = hours_to_days(hours);
/// assert_eq!(days, 1);
/// ```
pub fn hours_to_days(h: f64) -> f64 {
    return h / 24.0;
}

/// Converts Days to Hours.
///
/// Formula: h = d * 24
///
/// # Examples
///
/// ```
/// let days = 1;
/// let hours = days_to_hours(days);
/// assert_eq!(hours, 24);
/// ```
pub fn days_to_hours(d: f64) -> f64 {
    return d * 24.0;
}

/// Converts Weeks to Days.
///
/// Formula: d = w * 7
///
/// # Examples
///
/// ```
/// let weeks = 1;
/// let days = weeks_to_days(weeks);
/// assert_eq!(days, 7);
/// ```
pub fn weeks_to_days(w: f64) -> f64 {
    return w * 7.0;
}

/// Converts Days to Weeks.
///
/// Formula: w = d / 7
///
/// # Examples
///
/// ```
/// let days = 7;
/// let weeks = days_to_weeks(days);
/// assert_eq!(weeks, 1);
/// ```
pub fn days_to_weeks(d: f64) -> f64 {
    return d / 7.0;
}

/// Converts Weeks to Hours.
///
/// Formula: h = w * 168
///
/// # Examples
///
/// ```
/// let weeks = 1;
/// let hours = weeks_to_hours(weeks);
/// assert_eq!(hours, 168);
/// ```
pub fn weeks_to_hours(w: f64) -> f64 {
    return w * 168.0;
}

/// Converts Hours to Weeks.
///
/// Formula: w = h / 168
///
/// # Examples
///
/// ```
/// let hours = 168;
/// let weeks = hours_to_weeks(hours);
/// assert_eq!(weeks, 1);
/// ```
pub fn hours_to_weeks(h: f64) -> f64 {
    return h / 168.0;
}

/// Converts Weeks to Minutes.
///
/// Formula: min = w * 10080
///
/// # Examples
///
/// ```
/// let weeks = 1;
/// let minutes = weeks_to_minutes(weeks);
/// assert_eq!(minutes, 10080);
/// ```
pub fn weeks_to_minutes(w: f64) -> f64 {
    return w * 10080.0;
}

/// Converts Minutes to Weeks.
///
/// Formula: w = min / 10080
///
/// # Examples
///
/// ```
/// let minutes = 10080;
/// let weeks = minutes_to_weeks(minutes);
/// assert_eq!(weeks, 1);
/// ```
pub fn minutes_to_weeks(min: f64) -> f64 {
    return min / 10080.0;
}
