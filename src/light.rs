/// Converts Lux to Kilolux.
///
/// Formula: klx = lx / 1000
///
/// # Examples
///
/// ```
/// let lux = 1000;
/// let kilolux = lux_to_kilolux(lux);
/// assert_eq!(kilolux, 1);
/// ```
pub fn lux_to_kilolux(lx: f64) -> f64 {
    return lx / 1000.0;
}

/// Converts Kilolux to Lux.
///
/// Formula: lx = klx * 1000
///
/// # Examples
///
/// ```
/// let kilolux = 1;
/// let lux = kilolux_to_lux(kilolux);
/// assert_eq!(lux, 1000);
/// ```
pub fn kilolux_to_lux(klx: f64) -> f64 {
    return klx * 1000.0;
}

/// Converts Lux to Millilux.
///
/// Formula: mlx = lx * 1000
///
/// # Examples
///
/// ```
/// let lux = 1;
/// let millilux = lux_to_millilux(lux);
/// assert_eq!(millilux, 1000);
/// ```
pub fn lux_to_millilux(lx: f64) -> f64 {
    return lx * 1000.0;
}

/// Converts Millilux to Lux.
///
/// Formula: lx = mlx / 1000
///
/// # Examples
///
/// ```
/// let millilux = 1000;
/// let lux = millilux_to_lux(millilux);
/// assert_eq!(lux, 1);
/// ```
pub fn millilux_to_lux(mlx: f64) -> f64 {
    return mlx / 1000.0;
}

/// Converts Lux to Foot-candles.
///
/// Formula: fc = lx / 10.764
///
/// # Examples
///
/// ```
/// let lux = 10.764;
/// let foot_candles = lux_to_foot_candles(lux);
/// assert_eq!(foot_candles, 1);
/// ```
pub fn lux_to_foot_candles(lx: f64) -> f64 {
    return lx / 10.764;
}

/// Converts Foot-candles to Lux.
///
/// Formula: lx = fc * 10.764
///
/// # Examples
///
/// ```
/// let foot_candles = 1;
/// let lux = foot_candles_to_lux(foot_candles);
/// assert_eq!(lux, 10.764);
/// ```
pub fn foot_candles_to_lux(fc: f64) -> f64 {
    return fc * 10.764;
}

/// Converts Lux to Candlepower.
///
/// Formula: cp = lx * 0.001464
///
/// # Examples
///
/// ```
/// let lux = 1000;
/// let candlepower = lux_to_candlepower(lux);
/// assert_eq!(candlepower, 1.464);
/// ```
pub fn lux_to_candlepower(lx: f64) -> f64 {
    return lx * 0.001464;
}

/// Converts Candlepower to Lux.
///
/// Formula: lx = cp / 0.001464
///
/// # Examples
///
/// ```
/// let candlepower = 1.464;
/// let lux = candlepower_to_lux(candlepower);
/// assert_eq!(lux, 1000);
/// ```
pub fn candlepower_to_lux(cp: f64) -> f64 {
    return cp / 0.001464;
}
