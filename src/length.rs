/// Converts Meter to Centimeter.
///
/// Formula: cm = m * 100
///
/// # Examples
///
/// ```
/// let meter = 1;
/// let centimeter = meter_to_centimeter(meter);
/// assert_eq!(centimeter, 100);
/// ```
pub fn meter_to_centimeter(m: f64) -> f64 {
    return m * 100.0;
}

/// Converts Centimeter to Meter.
///
/// Formula: m = cm / 100
///
/// # Examples
///
/// ```
/// let centimeter = 100;
/// let meter = centimeter_to_meter(centimeter);
/// assert_eq!(meter, 1);
/// ```
pub fn centimeter_to_meter(cm: f64) -> f64 {
    return cm / 100.0;
}

/// Converts Meter to Kilometer.
///
/// Formula: km = m / 1000
///
/// # Examples
///
/// ```
/// let meter = 1000;
/// let kilometer = meter_to_kilometer(meter);
/// assert_eq!(kilometer, 1);
/// ```
pub fn meter_to_kilometer(m: f64) -> f64 {
    return m / 1000.0;
}

/// Converts Kilometer to Meter.
///
/// Formula: m = km * 1000
///
/// # Examples
///
/// ```
/// let kilometer = 1;
/// let meter = kilometer_to_meter(kilometer);
/// assert_eq!(meter, 1000);
/// ```
pub fn kilometer_to_meter(km: f64) -> f64 {
    return km * 1000.0;
}

/// Converts Meter to Millimeter.
///
/// Formula: mm = m * 1000
///
/// # Examples
///
/// ```
/// let meter = 1;
/// let millimeter = meter_to_millimeter(meter);
/// assert_eq!(millimeter, 1000);
/// ```
pub fn meter_to_millimeter(m: f64) -> f64 {
    return m * 1000.0;
}

/// Converts Millimeter to Meter.
///
/// Formula: m = mm / 1000
///
/// # Examples
///
/// ```
/// let millimeter = 1000;
/// let meter = millimeter_to_meter(millimeter);
/// assert_eq!(meter, 1);
/// ```
pub fn millimeter_to_meter(mm: f64) -> f64 {
    return mm / 1000.0;
}

/// Converts Meter to Mile.
///
/// Formula: mi = m / 1609.344
///
/// # Examples
///
/// ```
/// let meter = 1609.344;
/// let mile = meter_to_mile(meter);
/// assert_eq!(mile, 1);
/// ```
pub fn meter_to_mile(m: f64) -> f64 {
    return m / 1609.344;
}

/// Converts Mile to Meter.
///
/// Formula: m = mi * 1609.344
///
/// # Examples
///
/// ```
/// let mile = 1;
/// let meter = mile_to_meter(mile);
/// assert_eq!(meter, 1609.344);
/// ```
pub fn mile_to_meter(mi: f64) -> f64 {
    return mi * 1609.344;
}

/// Converts Meter to Yard.
///
/// Formula: yd = m * 1.0936133
///
/// # Examples
///
/// ```
/// let meter = 1;
/// let yard = meter_to_yard(meter);
/// assert_eq!(yard, 1.0936133);
/// ```
pub fn meter_to_yard(m: f64) -> f64 {
    return m * 1.0936133;
}

/// Converts Yard to Meter.
///
/// Formula: m = yd / 1.0936133
///
/// # Examples
///
/// ```
/// let yard = 1;
/// let meter = yard_to_meter(yard);
/// assert_eq!(meter, 0.9144);
/// ```
pub fn yard_to_meter(yd: f64) -> f64 {
    return yd / 1.0936133;
}

/// Converts Meter to Foot.
///
/// Formula: ft = m * 3.2808399
///
/// # Examples
///
/// ```
/// let meter = 1;
/// let foot = meter_to_foot(meter);
/// assert_eq!(foot, 3.2808399);
/// ```
pub fn meter_to_foot(m: f64) -> f64 {
    return m * 3.2808399;
}

/// Converts Foot to Meter.
///
/// Formula: m = ft / 3.2808399
///
/// # Examples
///
/// ```
/// let foot = 1;
/// let meter = foot_to_meter(foot);
/// assert_eq!(meter, 0.3048);
/// ```
pub fn foot_to_meter(ft: f64) -> f64 {
    return ft / 3.2808399;
}

/// Converts Meter to Inch.
///
/// Formula: in = m * 39.3700787
///
/// # Examples
///
/// ```
/// let meter = 1;
/// let inch = meter_to_inch(meter);
/// assert_eq!(inch, 39.3700787);
/// ```
pub fn meter_to_inch(m: f64) -> f64 {
    return m * 39.3700787;
}

/// Converts Inch to Meter.
///
/// Formula: m = in / 39.3700787
///
/// # Examples
///
/// ```
/// let inch = 1;
/// let meter = inch_to_meter(inch);
/// assert_eq!(meter, 0.0254);
/// ```
pub fn inch_to_meter(inch: f64) -> f64 {
    return inch / 39.3700787;
}

/// Converts Meter to Nautical Mile.
///
/// Formula: nmi = m / 1852
///
/// # Examples
///
/// ```
/// let meter = 1852;
/// let nautical_mile = meter_to_nautical_mile(meter);
/// assert_eq!(nautical_mile, 1);
/// ```
pub fn meter_to_nautical_mile(m: f64) -> f64 {
    return m / 1852.0;
}

/// Converts Nautical Mile to Meter.
///
/// Formula: m = nmi * 1852
///
/// # Examples
///
/// ```
/// let nautical_mile = 1;
/// let meter = nautical_mile_to_meter(nautical_mile);
/// assert_eq!(meter, 1852);
/// ```
pub fn nautical_mile_to_meter(nmi: f64) -> f64 {
    return nmi * 1852.0;
}

/// Converts Centimeter to Millimeter.
///
/// Formula: mm = cm * 10
///
/// # Examples
///
/// ```
/// let centimeter = 1;
/// let millimeter = centimeter_to_millimeter(centimeter);
/// assert_eq!(millimeter, 10);
/// ```
pub fn centimeter_to_millimeter(cm: f64) -> f64 {
    return cm * 10.0;
}

/// Converts Millimeter to Centimeter.
///
/// Formula: cm = mm / 10
///
/// # Examples
///
/// ```
/// let millimeter = 10;
/// let centimeter = millimeter_to_centimeter(millimeter);
/// assert_eq!(centimeter, 1);
/// ```
pub fn millimeter_to_centimeter(mm: f64) -> f64 {
    return mm / 10.0;
}

/// Converts Centimeter to Inch.
///
/// Formula: in = cm / 2.54
///
/// # Examples
///
/// ```
/// let centimeter = 2.54;
/// let inch = centimeter_to_inch(centimeter);
/// assert_eq!(inch, 1);
/// ```
pub fn centimeter_to_inch(cm: f64) -> f64 {
    return cm / 2.54;
}

/// Converts Inch to Centimeter.
///
/// Formula: cm = in * 2.54
///
/// # Examples
///
/// ```
/// let inch = 1;
/// let centimeter = inch_to_centimeter(inch);
/// assert_eq!(centimeter, 2.54);
/// ```
pub fn inch_to_centimeter(inch: f64) -> f64 {
    return inch * 2.54;
}

/// Converts Inch to Foot.
///
/// Formula: ft = in / 12
///
/// # Examples
///
/// ```
/// let inch = 12;
/// let foot = inch_to_foot(inch);
/// assert_eq!(foot, 1);
/// ```
pub fn inch_to_foot(inch: f64) -> f64 {
    return inch / 12.0;
}

/// Converts Foot to Inch.
///
/// Formula: in = ft * 12
///
/// # Examples
///
/// ```
/// let foot = 1;
/// let inch = foot_to_inch(foot);
/// assert_eq!(inch, 12);
/// ```
pub fn foot_to_inch(foot: f64) -> f64 {
    return foot * 12.0;
}

/// Converts Inch to Yard.
///
/// Formula: yd = in / 36
///
/// # Examples
///
/// ```
/// let inch = 36;
/// let yard = inch_to_yard(inch);
/// assert_eq!(yard, 1);
/// ```
pub fn inch_to_yard(inch: f64) -> f64 {
    return inch / 36.0;
}

/// Converts Yard to Inch.
///
/// Formula: in = yd * 36
///
/// # Examples
///
/// ```
/// let yard = 1;
/// let inch = yard_to_inch(yard);
/// assert_eq!(inch, 36);
/// ```
pub fn yard_to_inch(yd: f64) -> f64 {
    return yd * 36.0;
}

/// Converts Foot to Yard.
///
/// Formula: yd = ft / 3
///
/// # Examples
///
/// ```
/// let foot = 3;
/// let yard = foot_to_yard(foot);
/// assert_eq!(yard, 1);
/// ```
pub fn foot_to_yard(ft: f64) -> f64 {
    return ft / 3.0;
}

/// Converts Yard to Foot.
///
/// Formula: ft = yd * 3
///
/// # Examples
///
/// ```
/// let yard = 1;
/// let foot = yard_to_foot(yard);
/// assert_eq!(foot, 3);
/// ```
pub fn yard_to_foot(yd: f64) -> f64 {
    return yd * 3.0;
}

/// Converts Mile to Kilometer.
///
/// Formula: km = mi * 1.609344
///
/// # Examples
///
/// ```
/// let mile = 1;
/// let kilometer = mile_to_kilometer(mile);
/// assert_eq!(kilometer, 1.609344);
/// ```
pub fn mile_to_kilometer(mi: f64) -> f64 {
    return mi * 1.609344;
}

/// Converts Kilometer to Mile.
///
/// Formula: mi = km / 1.609344
///
/// # Examples
///
/// ```
/// let kilometer = 1;
/// let mile = kilometer_to_mile(kilometer);
/// assert_eq!(mile, 0.6213712);
/// ```
pub fn kilometer_to_mile(km: f64) -> f64 {
    return km / 1.609344;
}

/// Converts Nautical Mile to Kilometer.
///
/// Formula: km = nmi * 1.852
///
/// # Examples
///
/// ```
/// let nautical_mile = 1;
/// let kilometer = nautical_mile_to_kilometer(nautical_mile);
/// assert_eq!(kilometer, 1.852);
/// ```
pub fn nautical_mile_to_kilometer(nmi: f64) -> f64 {
    return nmi * 1.852;
}

/// Converts Kilometer to Nautical Mile.
///
/// Formula: nmi = km / 1.852
///
/// # Examples
///
/// ```
/// let kilometer = 1;
/// let nautical_mile = kilometer_to_nautical_mile(kilometer);
/// assert_eq!(nautical_mile, 0.5399568);
/// ```
pub fn kilometer_to_nautical_mile(km: f64) -> f64 {
    return km / 1.852;
}
