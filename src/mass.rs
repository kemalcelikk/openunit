/// Converts Kilogram to Gram.
///
/// Formula: g = kg * 1000
///
/// # Examples
///
/// ```
/// let kilogram = 1;
/// let gram = kilogram_to_gram(kilogram);
/// assert_eq!(gram, 1000);
/// ```
pub fn kilogram_to_gram(kg: f64) -> f64 {
    return kg * 1000.0;
}

/// Converts Gram to Kilogram.
///
/// Formula: kg = g / 1000
///
/// # Examples
///
/// ```
/// let gram = 1000;
/// let kilogram = gram_to_kilogram(gram);
/// assert_eq!(kilogram, 1);
/// ```
pub fn gram_to_kilogram(g: f64) -> f64 {
    return g / 1000.0;
}

/// Converts Kilogram to Milligram.
///
/// Formula: mg = kg * 1e+6
///
/// # Examples
///
/// ```
/// let kilogram = 1;
/// let milligram = kilogram_to_milligram(kilogram);
/// assert_eq!(milligram, 1_000_000);
/// ```
pub fn kilogram_to_milligram(kg: f64) -> f64 {
    return kg * 1_000_000.0;
}

/// Converts Milligram to Kilogram.
///
/// Formula: kg = mg / 1e+6
///
/// # Examples
///
/// ```
/// let milligram = 1_000_000;
/// let kilogram = milligram_to_kilogram(milligram);
/// assert_eq!(kilogram, 1);
/// ```
pub fn milligram_to_kilogram(mg: f64) -> f64 {
    return mg / 1_000_000.0;
}

/// Converts Kilogram to Tonne.
///
/// Formula: t = kg / 1000
///
/// # Examples
///
/// ```
/// let kilogram = 1000;
/// let tonne = kilogram_to_tonne(kilogram);
/// assert_eq!(tonne, 1);
/// ```
pub fn kilogram_to_tonne(kg: f64) -> f64 {
    return kg / 1000.0;
}

/// Converts Tonne to Kilogram.
///
/// Formula: kg = t * 1000
///
/// # Examples
///
/// ```
/// let tonne = 1;
/// let kilogram = tonne_to_kilogram(tonne);
/// assert_eq!(kilogram, 1000);
/// ```
pub fn tonne_to_kilogram(t: f64) -> f64 {
    return t * 1000.0;
}

/// Converts Pound to Kilogram.
///
/// Formula: kg = lb * 0.45359237
///
/// # Examples
///
/// ```
/// let pound = 1;
/// let kilogram = pound_to_kilogram(pound);
/// assert_eq!(kilogram, 0.45359237);
/// ```
pub fn pound_to_kilogram(lb: f64) -> f64 {
    return lb * 0.45359237;
}

/// Converts Kilogram to Pound.
///
/// Formula: lb = kg / 0.45359237
///
/// # Examples
///
/// ```
/// let kilogram = 1;
/// let pound = kilogram_to_pound(kilogram);
/// assert_eq!(pound, 2.204622621848776);
/// ```
pub fn kilogram_to_pound(kg: f64) -> f64 {
    return kg / 0.45359237;
}

/// Converts Ounce to Gram.
///
/// Formula: g = oz * 28.3495231
///
/// # Examples
///
/// ```
/// let ounce = 1;
/// let gram = ounce_to_gram(ounce);
/// assert_eq!(gram, 28.3495231);
/// ```
pub fn ounce_to_gram(oz: f64) -> f64 {
    return oz * 28.3495231;
}

/// Converts Gram to Ounce.
///
/// Formula: oz = g / 28.3495231
///
/// # Examples
///
/// ```
/// let gram = 28.3495231;
/// let ounce = gram_to_ounce(gram);
/// assert_eq!(ounce, 1);
/// ```
pub fn gram_to_ounce(g: f64) -> f64 {
    return g / 28.3495231;
}

/// Converts Gram to Milligram.
///
/// Formula: mg = g * 1000
///
/// # Examples
///
/// ```
/// let gram = 1;
/// let milligram = gram_to_milligram(gram);
/// assert_eq!(milligram, 1000);
/// ```
pub fn gram_to_milligram(g: f64) -> f64 {
    return g * 1000.0;
}

/// Converts Milligram to Gram.
///
/// Formula: g = mg / 1000
///
/// # Examples
///
/// ```
/// let milligram = 1000;
/// let gram = milligram_to_gram(milligram);
/// assert_eq!(gram, 1);
/// ```
pub fn milligram_to_gram(mg: f64) -> f64 {
    return mg / 1000.0;
}

/// Converts Tonne to Pound.
///
/// Formula: lb = t * 2204.62
///
/// # Examples
///
/// ```
/// let tonne = 1;
/// let pound = tonne_to_pound(tonne);
/// assert_eq!(pound, 2204.62);
/// ```
pub fn tonne_to_pound(t: f64) -> f64 {
    return t * 2204.62;
}

/// Converts Pound to Tonne.
///
/// Formula: t = lb / 2204.62
///
/// # Examples
///
/// ```
/// let pound = 2204.62;
/// let tonne = pound_to_tonne(pound);
/// assert_eq!(tonne, 1);
/// ```
pub fn pound_to_tonne(lb: f64) -> f64 {
    return lb / 2204.62;
}

/// Converts Pound to Ounce.
///
/// Formula: oz = lb * 16
///
/// # Examples
///
/// ```
/// let pound = 1;
/// let ounce = pound_to_ounce(pound);
/// assert_eq!(ounce, 16);
/// ```
pub fn pound_to_ounce(lb: f64) -> f64 {
    return lb * 16.0;
}

/// Converts Ounce to Pound.
///
/// Formula: lb = oz / 16
///
/// # Examples
///
/// ```
/// let ounce = 16;
/// let pound = ounce_to_pound(ounce);
/// assert_eq!(pound, 1);
/// ```
pub fn ounce_to_pound(oz: f64) -> f64 {
    return oz / 16.0;
}

/// Converts Milligram to Pound.
///
/// Formula: lb = mg * 0.00000220462
///
/// # Examples
///
/// ```
/// let milligram = 1000000;
/// let pound = milligram_to_pound(milligram);
/// assert_eq!(pound, 2.20462);
/// ```
pub fn milligram_to_pound(mg: f64) -> f64 {
    return mg * 0.00000220462;
}

/// Converts Pound to Milligram.
///
/// Formula: mg = lb / 0.00000220462
///
/// # Examples
///
/// ```
/// let pound = 2.20462;
/// let milligram = pound_to_milligram(pound);
/// assert_eq!(milligram, 1000000);
/// ```
pub fn pound_to_milligram(lb: f64) -> f64 {
    return lb / 0.00000220462;
}

/// Converts Milligram to Ounce.
///
/// Formula: oz = mg * 0.000035274
///
/// # Examples
///
/// ```
/// let milligram = 1000;
/// let ounce = milligram_to_ounce(milligram);
/// assert_eq!(ounce, 0.035274);
/// ```
pub fn milligram_to_ounce(mg: f64) -> f64 {
    return mg * 0.000035274;
}

/// Converts Ounce to Milligram.
///
/// Formula: mg = oz / 0.000035274
///
/// # Examples
///
/// ```
/// let ounce = 0.035274;
/// let milligram = ounce_to_milligram(ounce);
/// assert_eq!(milligram, 1000);
/// ```
pub fn ounce_to_milligram(oz: f64) -> f64 {
    return oz / 0.000035274;
}
