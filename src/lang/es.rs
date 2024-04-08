use core::fmt::{self, Formatter};
use std::fmt::Display;
use std::str::FromStr;

use num_bigfloat::BigFloat;

use super::Language;
use crate::Num2Err;
// Reference that can hopefully be implemented seamlessly: https://es.wikipedia.org/wiki/Anexo:Nombres_de_los_n%C3%BAmeros_en_espa%C3%B1ol
const UNIDADES: [&str; 10] =
    ["", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve"];
// Decenas que son entre 11 y 19
const DIECIS: [&str; 10] = [
    "diez", // Needed for cases like 10, 10_000 and 10_000_000
    "once",
    "doce",
    "trece",
    "catorce",
    "quince",
    "dieciséis",
    "diecisiete",
    "dieciocho",
    "diecinueve",
];
// Saltos en decenas
const DECENAS: [&str; 10] = [
    "",
    "", // This actually never gets called, but if so, it probably should be "diez"
    "veinte",
    "treinta",
    "cuarenta",
    "cincuenta",
    "sesenta",
    "setenta",
    "ochenta",
    "noventa",
];
// Saltos en decenas
// Binary size might see a dozen bytes improvement if we append "ientos" at CENTENAS's callsites
const CENTENAS: [&str; 10] = [
    "",
    "ciento",
    "doscientos",
    "trescientos",
    "cuatrocientos",
    "quinientos",
    "seiscientos",
    "setecientos",
    "ochocientos",
    "novecientos",
];
// To ensure both arrays doesn't desync
const MILLAR_SIZE: usize = 22;
/// from source: https://es.wikipedia.org/wiki/Anexo:Nombres_de_los_n%C3%BAmeros_en_espa%C3%B1ol
/// Based on https://en.wikipedia.org/wiki/Names_of_large_numbers, each thousands is from the Short Scales,
/// which each thousands can be defined as 10^(3n+3) magnitude, where n is replaced by the index of
/// the Array. For example 10^3 = Thousands (starts at n=1 here)
/// 10^6 = Millions
/// 10^9 = Billions
/// 10^33 = Decillion
// Saltos en Millares
const MILLARES: [&str; MILLAR_SIZE] = [
    "",
    "mil",
    "millones",
    "billones",
    "trillones",
    "cuatrillones",
    "quintillones",
    "sextillones",
    "septillones",
    "octillones",
    "nonillones",
    "decillones",
    "undecillones",
    "duodecillones",
    "tredecillones",
    "cuatrodecillones",
    "quindeciollones",
    "sexdecillones",
    "septendecillones",
    "octodecillones",
    "novendecillones",
    "vigintillones",
];
// Saltos en Millar
const MILLAR: [&str; MILLAR_SIZE] = [
    "",
    "mil",
    "millón",
    "billón",
    "trillón",
    "cuatrillón",
    "quintillón",
    "sextillón",
    "septillón",
    "octillón",
    "nonillón",
    "decillón",
    "undecillón",
    "duodecillón",
    "tredecillón",
    "cuatrodecillón",
    "quindeciollón",
    "sexdecillón",
    "septendecillón",
    "octodecillón",
    "novendecillón",
    "vigintillón",
];
pub mod ordinal {
    // Gender must be added at callsite
    pub(super) const UNIDADES: [&str; 10] =
        ["", "primer", "segund", "tercer", "cuart", "quint", "sext", "séptim", "octav", "noven"];
    pub(super) const DECENAS: [&str; 10] = [
        "",
        "", // expects DIECIS to be called instead
        "vigésim",
        "trigésim",
        "cuadragésim",
        "quincuagésim",
        "sexagésim",
        "septuagésim",
        "octogésim",
        "nonagésim",
    ];
    // Gender must be added at callsite
    pub(super) const DIECIS: [&str; 10] = [
        "décim",
        "undécim",  // `decimoprimero` is a valid word
        "duodécim", // `décimosegundo` is a valid word
        "decimotercer",
        "decimocuart",
        "decimoquint",
        "decimosext",
        "decimoséptim",
        "decimooctav",
        "decimonoven",
    ];
    pub(super) const CENTENAS: [&str; 10] = [
        "",
        "centésim",
        "ducentésim",
        "tricentésim",
        "cuadringentésim",
        "quingentésim",
        "sexcentésim",
        "septingentésim",
        "octingentésim",
        "noningentésim",
    ];
    pub(super) const MILLARES: [&str; 22] = [
        "",
        "milésim",
        "millonésim",
        "billonésim",
        "trillonésim",
        "cuatrillonésim",
        "quintillonésim",
        "sextillonésim",
        "septillonésim",
        "octillonésim",
        "nonillonésim",
        "decillonésim",
        "undecillonésim",
        "duodecillonésim",
        "tredecillonésim",
        "cuatrodecillonésim",
        "quindeciollonésim",
        "sexdecillonésim",
        "septendecillonésim",
        "octodecillonésim",
        "novendecillonésim",
        "vigintillonésim",
    ];
}
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Spanish {
    /// Negative flavour like "bajo cero", "menos", "negativo"
    neg_flavour: NegativeFlavour,
    // Writes the number as "veinte y ocho" instead of "veintiocho" in case of true
    prefer_veinte: bool,
    decimal_char: DecimalChar,
    // Gender for ordinal numbers
    feminine: bool,
    // Plural for ordinal numbers
    plural: bool,
}
#[allow(unused)]
impl Spanish {
    #[inline(always)]
    pub fn new(decimal_char: DecimalChar, feminine: bool) -> Self {
        Self { decimal_char, feminine, ..Default::default() }
    }

    #[inline(always)]
    pub fn set_feminine(&mut self, feminine: bool) -> &mut Self {
        self.feminine = feminine;
        self
    }

    #[inline(always)]
    pub fn with_feminine(self, feminine: bool) -> Self {
        Self { feminine, ..self }
    }

    #[inline(always)]
    pub fn set_plural(&mut self, plural: bool) -> &mut Self {
        self.plural = plural;
        self
    }

    #[inline(always)]
    pub fn with_plural(self, plural: bool) -> Self {
        Self { plural, ..self }
    }

    #[inline(always)]
    pub fn set_veinte(&mut self, veinte: bool) -> &mut Self {
        self.prefer_veinte = veinte;
        self
    }

    #[inline(always)]
    pub fn with_veinte(self, veinte: bool) -> Self {
        Self { prefer_veinte: veinte, ..self }
    }

    #[inline(always)]
    pub fn set_neg_flavour(&mut self, flavour: NegativeFlavour) -> &mut Self {
        self.neg_flavour = flavour;
        self
    }

    #[inline(always)]
    pub fn with_neg_flavour(self, flavour: NegativeFlavour) -> Self {
        Self { neg_flavour: flavour, ..self }
    }

    #[inline(always)]
    pub fn set_decimal_char(&mut self, decimal_char: DecimalChar) -> &mut Self {
        self.decimal_char = decimal_char;
        self
    }

    #[inline(always)]
    pub fn with_decimal_char(self, decimal_char: DecimalChar) -> Self {
        Self { decimal_char, ..self }
    }

    #[inline(always)]
    // Converts Integer BigFloat to a vector of u64
    fn en_miles(&self, mut num: BigFloat) -> Vec<u64> {
        // Doesn't check if BigFloat is Integer only
        let mut thousands = Vec::new();
        let mil = 1000.into();
        num = num.abs();
        while !num.int().is_zero() {
            // Insertar en Low Endian
            thousands.push((num % mil).to_u64().expect("triplet not under 1000"));
            num /= mil; // DivAssign
        }
        thousands
    }

    // Only should be called if you're sure the number has no fraction
    fn int_to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        // Don't convert a number with fraction, NaN or Infinity
        if !num.frac().is_zero() || num.is_nan() || num.is_inf() {
            return Err(Num2Err::CannotConvert);
        }

        if num.is_zero() {
            return Ok(String::from("cero"));
        }

        let mut words = vec![];
        for (i, triplet) in self.en_miles(num.int()).into_iter().enumerate().rev() {
            let hundreds = ((triplet / 100) % 10) as usize;
            let tens = ((triplet / 10) % 10) as usize;
            let units = (triplet % 10) as usize;

            if hundreds > 0 {
                match triplet {
                    // Edge case when triplet is a hundred
                    100 => words.push(String::from("cien")),
                    _ => words.push(String::from(CENTENAS[hundreds])),
                }
            }

            if tens != 0 || units != 0 {
                let unit_word = match (units, i) {
                    // case `1_100` => `mil cien` instead of `un mil cien`
                    // case `1_001_000` => `un millón mil` instead of `un millón un mil`
                    // Explanation: Second triplet is always read as thousand, so we
                    // don't need to say "un mil"
                    (_, 1) if triplet == 1 => "",
                    // case `001_001_100...` => `un billón un millón cien mil...` instead of
                    // `uno billón uno millón cien mil...`
                    // All `triplets == 1`` can can be named as "un". except for first or second
                    // triplet
                    (_, index) if index != 0 && triplet == 1 => "un",
                    _ => UNIDADES[units],
                };

                match tens {
                    // case `?_102` => `? ciento dos`
                    0 => words.push(String::from(unit_word)),
                    // case `?_119` => `? ciento diecinueve`
                    // case `?_110` => `? ciento diez`
                    1 => words.push(String::from(DIECIS[units])),
                    2 if self.prefer_veinte && units != 0 => {
                        let unit_word = if units == 1 && i != 0 { "un" } else { unit_word };
                        words.push(format!("veinte y {unit_word}"));
                    }
                    2 => words.push(match units {
                        0 => String::from(DECENAS[tens]),
                        // case `021_...` => `? veintiún...`
                        1 if i != 0 => String::from("veintiún"),
                        // case `?_021` => `? veintiuno`
                        _ => format!("veinti{unit_word}"),
                    }),
                    _ => {
                        // case `?_142 => `? ciento cuarenta y dos`
                        let ten = DECENAS[tens];
                        words.push(match units {
                            0 => String::from(ten),
                            _ => format!("{ten} y {unit_word}"),
                        });
                    }
                }
            }

            // Add the next Milliard if there's any.
            if i != 0 && triplet != 0 {
                if i > MILLARES.len() - 1 {
                    return Err(Num2Err::CannotConvert);
                }
                // Boolean that checks if next Milliard is plural
                let plural = triplet != 1;
                match plural {
                    false => words.push(String::from(MILLAR[i])),
                    true => words.push(String::from(MILLARES[i])),
                }
            }
        }
        // flavour the text when negative
        if let (flavour, true) = (&self.neg_flavour, num.is_negative()) {
            self.flavourize_with_negative(&mut words, *flavour)
        }

        Ok(words.into_iter().filter(|word| !word.is_empty()).collect::<Vec<String>>().join(" "))
    }

    fn float_to_cardinal(&self, num: &BigFloat) -> Result<String, Num2Err> {
        let mut words = vec![];
        let is_negative = num.is_negative();
        let num = num.abs();
        let positive_int_word = self.int_to_cardinal(num.int())?;
        words.push(positive_int_word);

        let mut fraction_part = num.frac();
        if !fraction_part.is_zero() {
            // Inserts decimal separator
            words.push(self.decimal_char.to_word().to_string());
        }

        while !fraction_part.is_zero() {
            let digit = (fraction_part * BigFloat::from(10)).int();
            fraction_part = (fraction_part * BigFloat::from(10)).frac();
            words.push(match digit.to_u64().unwrap() {
                0 => String::from("cero"),
                i => String::from(UNIDADES[i as usize]),
            });
        }
        if is_negative {
            self.flavourize_with_negative(&mut words, self.neg_flavour);
        }
        Ok(words.join(" "))
    }

    #[inline(always)]
    fn inf_to_cardinal(&self, num: &BigFloat) -> Result<String, Num2Err> {
        if !num.is_inf() {
            Err(Num2Err::CannotConvert)
        } else if num.is_inf_pos() {
            Ok(String::from("infinito"))
        } else {
            let word = match self.neg_flavour {
                NegativeFlavour::Prepended => "{} infinito",
                NegativeFlavour::Appended => "infinito {}",
                // Defaults to `menos` because it doesn't make sense to call `infinito bajo cero`
                NegativeFlavour::BelowZero => "menos infinito",
            }
            .replace("{}", self.neg_flavour.as_str());
            Ok(word)
        }
    }

    #[inline(always)]
    fn flavourize_with_negative(&self, words: &mut Vec<String>, flavour: NegativeFlavour) {
        use NegativeFlavour::*;
        let string = flavour.to_string();
        match flavour {
            Prepended => words.insert(0, string),
            Appended => words.push(string),
            BelowZero => words.push(string),
        }
    }
}
impl Language for Spanish {
    /// Converts a BigFloat to a cardinal number in Spanish
    /// ```rust
    /// use num2words::{Lang, Num2Words};
    /// use num_bigfloat::BigFloat;
    ///
    /// let words = Num2Words::new(-123_456.789)
    ///     .lang(Lang::Spanish)
    ///     .cardinal()
    ///     .prefer("negativo")
    ///     .to_words()
    ///     .unwrap();
    /// assert_eq!(
    ///     words,
    ///     "ciento veintitres mil cuatrocientos cincuenta y seis punto siete ocho nueve negativo"
    /// );
    /// ```
    fn to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        if num.is_nan() {
            Err(Num2Err::CannotConvert)
        } else if num.is_inf() {
            self.inf_to_cardinal(&num)
        } else if num.frac().is_zero() {
            self.int_to_cardinal(num)
        } else {
            self.float_to_cardinal(&num)
        }
    }

    /// Ordinal numbers above 10 are unnatural for Spanish speakers. Don't rely on these to convey
    /// meanings
    /// ```rust
    /// use num2words::{Lang, Num2Words};
    /// use num_bigfloat::BigFloat;
    ///
    /// let words = Num2Words::new(11).lang(Lang::Spanish).ordinal().to_words().unwrap();
    /// assert_eq!(words, "undécimo");
    /// ```
    fn to_ordinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        // Important to keep so it doesn't conflict with the main module's constants
        use ordinal::{CENTENAS, DECENAS, DIECIS, MILLARES, UNIDADES};
        match (num.is_inf(), num.is_negative(), num.frac().is_zero()) {
            _ if num.is_nan() => return Err(Num2Err::CannotConvert),
            (true, _, _) => return Err(Num2Err::InfiniteOrdinal),
            (_, true, _) => return Err(Num2Err::NegativeOrdinal),
            (_, _, false) => return Err(Num2Err::FloatingOrdinal),
            _ => (), /* Nothing Happens */
        }
        let mut words = vec![];
        let gender = || -> &'static str { if self.feminine { "a" } else { "o" } };
        for (i, triplet) in self
            .en_miles(num.int())
            .into_iter()
            .enumerate()
            .rev()
            .filter(|(_, triplet)| *triplet != 0)
        {
            let hundreds = ((triplet / 100) % 10) as usize;
            let tens = ((triplet / 10) % 10) as usize;
            let units = (triplet % 10) as usize;

            if hundreds > 0 {
                // case `500` => `quingentesim@`
                words.push(String::from(CENTENAS[hundreds]) + gender());
            }

            if tens != 0 || units != 0 {
                let unit_word = UNIDADES[units];
                match tens {
                    // case `?_001` => `? primer`
                    0 if triplet == 1 && i > 0 => words.push(String::from("primer")),
                    0 => words.push(String::from(unit_word) + gender()),
                    // case `?_119` => `? centésim@ decimonoven@`
                    // case `?_110` => `? centésim@ decim@`
                    1 => words.push(String::from(DIECIS[units]) + gender()),
                    _ => {
                        let ten = DECENAS[tens];
                        let word = match units {
                            // case `?_120 => `? centésim@ vigésim@`
                            0 => String::from(ten),
                            // case `?_122 => `? centésim@ vigésim@ segund@`
                            _ => format!("{ten}{} {unit_word}", gender()),
                        };

                        words.push(word + gender());
                    }
                }
            }
            // Add the next Milliard if there's any.
            if i != 0 && triplet != 0 {
                if i > MILLARES.len() - 1 {
                    return Err(Num2Err::CannotConvert);
                }
                words.push(String::from(MILLARES[i]) + gender());
            }
        }
        if self.plural {
            if let Some(word) = words.last_mut() {
                word.push('s');
            }
        }
        Ok(words.into_iter().filter(|word| !word.is_empty()).collect::<Vec<String>>().join(" "))
    }

    /// A numeric number which has a `ª` or `º` appended at the end
    /// ```rust
    /// use num2words::{Lang, Num2Words};
    /// use num_bigfloat::BigFloat;
    ///
    /// let words = Num2Words::new(8).lang(Lang::Spanish).ordinal_num().to_words().unwrap();
    /// assert_eq!(words, "8º", "some mismatch");
    ///
    /// let words = Num2Words::new(8).lang(Lang::Spanish).ordinal_num().prefer("femenino");
    /// assert_eq!(words.to_words().unwrap(), "8ª", "some mismatch2");
    /// ```
    fn to_ordinal_num(&self, num: BigFloat) -> Result<String, Num2Err> {
        match (num.is_inf(), num.is_negative(), num.frac().is_zero()) {
            _ if num.is_nan() => return Err(Num2Err::CannotConvert),
            (true, _, _) => return Err(Num2Err::InfiniteOrdinal),
            (_, true, _) => return Err(Num2Err::NegativeOrdinal),
            (_, _, false) => return Err(Num2Err::FloatingOrdinal),
            _ => (), /* Nothing Happens */
        }

        let mut word = num.to_i128().ok_or(Num2Err::CannotConvert)?.to_string();
        word.push(if self.feminine { 'ª' } else { 'º' });
        Ok(word)
    }

    /// A year is just a Cardinal number. When the BigFloat input is negative, it appends "a.C." to
    /// the positive Cardinal representation
    /// ```rust
    /// use num2words::{Lang, Num2Words};
    /// use num_bigfloat::BigFloat;
    ///
    /// let words = Num2Words::new(2021).lang(Lang::Spanish).year().to_words().unwrap();
    /// assert_eq!(words, "dos mil veintiuno");
    ///
    /// let words = Num2Words::new(-2021).lang(Lang::Spanish).year().to_words().unwrap();
    /// assert_eq!(words, "dos mil veintiuno a. C.");
    /// ```
    fn to_year(&self, num: BigFloat) -> Result<String, Num2Err> {
        match (num.is_inf(), num.frac().is_zero(), num.int().is_zero()) {
            _ if num.is_nan() => return Err(Num2Err::CannotConvert),
            (true, _, _) => return Err(Num2Err::InfiniteYear),
            (_, false, _) => return Err(Num2Err::FloatingYear),
            (_, _, true) => return Err(Num2Err::CannotConvert), // Year 0 is not a thing
            _ => (/* Nothing Happens */),
        }

        let mut num = num;

        let suffix = if num.is_negative() {
            num = num.inv_sign();
            " a. C."
        } else {
            ""
        };

        // Years in spanish are read the same as cardinal numbers....(?)
        // src:https://twitter.com/RAEinforma/status/1761725275736334625?lang=en
        let year_word = self.int_to_cardinal(num)?;
        Ok(format!("{}{}", year_word, suffix))
    }

    /// A Cardinal number which then the currency word representation is appended at the end.
    /// Any cardinal that ends in "uno" is the only exception to the rule. For example 41, 21 and 1
    /// The extra decimals are truncated instead of rounded
    /// ```rust
    /// use num2words::{Currency, Lang, Num2Words};
    /// use num_bigfloat::BigFloat;
    ///
    /// let words =
    ///     Num2Words::new(-2021).lang(Lang::Spanish).currency(Currency::USD).to_words().unwrap();
    /// assert_eq!(words, "menos dos mil veintiún US dollars");
    ///
    /// let words =
    ///     Num2Words::new(81.21).lang(Lang::Spanish).currency(Currency::USD).to_words().unwrap();
    /// assert_eq!(words, "ochenta y un US dollars con veintiún centavos");
    ///
    /// let words =
    ///     Num2Words::new(1.01).lang(Lang::Spanish).currency(Currency::USD).to_words().unwrap();
    /// assert_eq!(words, "un US dollar con un centavo");
    ///
    /// let words = Num2Words::new(1).lang(Lang::Spanish).currency(Currency::USD).to_words().unwrap();
    /// assert_eq!(words, "un US dollar");
    /// ```
    fn to_currency(&self, num: BigFloat, currency: crate::Currency) -> Result<String, Num2Err> {
        let strip_uno_into_un = |string: String| -> String {
            let len = string.len();
            if string.ends_with("iuno") {
                string[..len - 3].to_string() + "ún"
            } else if string.ends_with("uno") {
                string[..len - 1].to_string()
            } else {
                string
            }
        };
        if num.is_nan() {
            Err(Num2Err::CannotConvert)
        } else if num.is_inf() {
            let currency = currency.default_string(true);
            let inf = self.inf_to_cardinal(&num)? + "de {}";
            let word = inf.replace("{}", &currency);
            return Ok(word);
        } else if num.frac().is_zero() {
            let is_plural = num.int() != 1.into();
            let currency = currency.default_string(is_plural);
            let cardinal = strip_uno_into_un(self.int_to_cardinal(num)?);
            return Ok(format!("{cardinal} {currency}"));
        } else {
            let hundred: BigFloat = 100.into();
            let (integral, cents) = (num.int(), num.mul(&hundred).int().rem(&hundred));
            let cents_is_plural = cents != 1.into();
            let (int_words, cent_words) = (
                self.to_currency(integral, currency)?,
                strip_uno_into_un(self.int_to_cardinal(cents)?),
            );
            let cents_suffix = currency.default_subunit_string("centavo{}", cents_is_plural);

            if cents.is_zero() {
                return Ok(int_words);
            } else if integral.is_zero() {
                return Ok(format!("{cent_words} {cents_suffix}"));
            } else {
                return Ok(format!("{} con {} {cents_suffix}", int_words, cent_words));
            }
        }
    }
}
// TODO: Remove Copy trait if enums can store data
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum NegativeFlavour {
    #[default]
    Prepended, // -1 => menos uno
    Appended,  // -1 => uno negativo
    BelowZero, // -1 => uno bajo cero
}
impl NegativeFlavour {
    pub fn as_str(&self) -> &'static str {
        match self {
            NegativeFlavour::Prepended => "menos",
            NegativeFlavour::Appended => "negativo",
            NegativeFlavour::BelowZero => "bajo cero",
        }
    }
}
impl FromStr for NegativeFlavour {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "prepended" => NegativeFlavour::Prepended,
            "appended" => NegativeFlavour::Appended,
            "menos" => NegativeFlavour::Prepended,
            "negativo" => NegativeFlavour::Appended,
            "bajo cero" => NegativeFlavour::BelowZero,
            _ => return Err(()),
        };
        Ok(result)
    }
}

impl Display for NegativeFlavour {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{str}", str = self.as_str())
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecimalChar {
    #[default]
    Punto,
    Coma,
}
impl FromStr for DecimalChar {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "punto" => DecimalChar::Punto,
            "coma" => DecimalChar::Coma,
            _ => return Err(()),
        };
        debug_assert!(result.to_word() == s, "DecimalChar::from_str() is incorrect");
        Ok(result)
    }
}
impl DecimalChar {
    #[inline(always)]
    pub fn to_word(self) -> &'static str {
        match self {
            DecimalChar::Punto => "punto",
            DecimalChar::Coma => "coma",
        }
    }
}
#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;
    #[inline(always)]
    fn to<T: Into<BigFloat>>(input: T) -> BigFloat {
        input.into()
    }

    #[test]
    fn decimal_char_enum_integrity() {
        // Test if the enum can be converted to string and back
        assert_eq!(DecimalChar::from_str("punto").unwrap(), DecimalChar::Punto);
        assert_eq!(DecimalChar::from_str("coma").unwrap(), DecimalChar::Coma);
    }

    #[test]
    fn negative_flavour_enum_integrity() {
        // Test if the enum can be converted to string and back
        assert_eq!(NegativeFlavour::from_str("menos").unwrap(), NegativeFlavour::Prepended);
        assert_eq!(NegativeFlavour::from_str("negativo").unwrap(), NegativeFlavour::Appended);
        assert_eq!(NegativeFlavour::from_str("bajo cero").unwrap(), NegativeFlavour::BelowZero);
    }

    #[test]
    fn lang_es_sub_thousands() {
        let es = Spanish::default();
        assert_eq!(es.int_to_cardinal(to(000)).unwrap(), "cero");
        assert_eq!(es.int_to_cardinal(to(10)).unwrap(), "diez");
        assert_eq!(es.int_to_cardinal(to(100)).unwrap(), "cien");
        assert_eq!(es.int_to_cardinal(to(101)).unwrap(), "ciento uno");
        assert_eq!(es.int_to_cardinal(to(110)).unwrap(), "ciento diez");
        assert_eq!(es.int_to_cardinal(to(111)).unwrap(), "ciento once");
        assert_eq!(es.int_to_cardinal(to(141)).unwrap(), "ciento cuarenta y uno");
        assert_eq!(es.int_to_cardinal(to(142)).unwrap(), "ciento cuarenta y dos");
        assert_eq!(es.int_to_cardinal(to(800)).unwrap(), "ochocientos");
    }

    #[test]
    fn lang_es_thousands() {
        let es = Spanish::default();
        // When thousands triplet is 1
        assert_eq!(es.int_to_cardinal(to(1_000)).unwrap(), "mil");
        assert_eq!(es.int_to_cardinal(to(1_010)).unwrap(), "mil diez");
        assert_eq!(es.int_to_cardinal(to(1_100)).unwrap(), "mil cien");
        assert_eq!(es.int_to_cardinal(to(1_101)).unwrap(), "mil ciento uno");
        assert_eq!(es.int_to_cardinal(to(1_110)).unwrap(), "mil ciento diez");
        assert_eq!(es.int_to_cardinal(to(1_111)).unwrap(), "mil ciento once");
        assert_eq!(es.int_to_cardinal(to(1_141)).unwrap(), "mil ciento cuarenta y uno");
        // When thousands triplet isn't 1
        assert_eq!(es.int_to_cardinal(to(2_000)).unwrap(), "dos mil");
        assert_eq!(es.int_to_cardinal(to(12_010)).unwrap(), "doce mil diez");
        assert_eq!(es.int_to_cardinal(to(140_100)).unwrap(), "ciento cuarenta mil cien");
        assert_eq!(
            es.int_to_cardinal(to(141_101)).unwrap(),
            "ciento cuarenta y uno mil ciento uno"
        );
        assert_eq!(es.int_to_cardinal(to(142_002)).unwrap(), "ciento cuarenta y dos mil dos");
        assert_eq!(es.int_to_cardinal(to(142_000)).unwrap(), "ciento cuarenta y dos mil");
        assert_eq!(
            es.int_to_cardinal(to(888_111)).unwrap(),
            "ochocientos ochenta y ocho mil ciento once"
        );
        assert_eq!(es.int_to_cardinal(to(800_000)).unwrap(), "ochocientos mil");
    }

    #[test]
    fn lang_es_test_by_concept_to_cardinal_method() {
        // This might make other tests trivial
        let es = Spanish::default();
        // Triplet == 1 inserts following milliard in singular
        assert_eq!(es.int_to_cardinal(to(1_001_001_000)).unwrap(), "un billón un millón mil");
        // Triplet != 1 inserts following milliard in plural
        assert_eq!(
            es.int_to_cardinal(to(2_002_002_000)).unwrap(),
            "dos billones dos millones dos mil"
        );
        // Thousand's milliard is singular
        assert_eq!(es.int_to_cardinal(to(1_100)).unwrap(), "mil cien");
        // Thousand's milliard is plural
        assert_eq!(es.int_to_cardinal(to(2_100)).unwrap(), "dos mil cien");
        // Cardinal number ending in 1 always ends with "uno"
        assert!(es.int_to_cardinal(to(12_233_521_251.0)).unwrap().ends_with("uno"));
        // triplet with value "10"
        assert_eq!(es.int_to_cardinal(to(110_010_000)).unwrap(), "ciento diez millones diez mil");
        // Triplets ending in 1 but higher than 30, is "uno"
        // "un" is reserved for triplet == 1 in magnitudes higher than 10^3 like "un millón"
        // or "un trillón"
        assert_eq!(
            es.int_to_cardinal(to(171_031_041_031.0)).unwrap(),
            "ciento setenta y uno billones treinta y uno millones cuarenta y uno mil treinta y uno"
        );
    }

    #[test]
    fn lang_es_lang_trait_methods_fails_on() {
        let es = Spanish::default();
        let to_cardinal = Language::to_cardinal;
        assert_eq!(to_cardinal(&es, to(f64::NAN)).unwrap_err(), Num2Err::CannotConvert);
        // Vigintillion supposedly has 63 zeroes, so anything beyond ~66 digits should fail with
        // current impl
        let some_big_num = BigFloat::from_u8(2).pow(&BigFloat::from_u8(230));
        assert_eq!(to_cardinal(&es, to(some_big_num)).unwrap_err(), Num2Err::CannotConvert);

        let to_ordinal = Language::to_ordinal;
        assert_eq!(to_ordinal(&es, to(0.001)).unwrap_err(), Num2Err::FloatingOrdinal);
        assert_eq!(to_ordinal(&es, to(-0.01)).unwrap_err(), Num2Err::NegativeOrdinal);
        assert_eq!(to_ordinal(&es, to(f64::NAN)).unwrap_err(), Num2Err::CannotConvert);
        assert_eq!(to_ordinal(&es, to(f64::INFINITY)).unwrap_err(), Num2Err::InfiniteOrdinal);
        assert_eq!(to_ordinal(&es, to(f64::NEG_INFINITY)).unwrap_err(), Num2Err::InfiniteOrdinal);

        let to_ord_num = Language::to_ordinal_num;
        assert_eq!(to_ord_num(&es, to(0.001)).unwrap_err(), Num2Err::FloatingOrdinal);
        assert_eq!(to_ord_num(&es, to(-0.01)).unwrap_err(), Num2Err::NegativeOrdinal);
        assert_eq!(to_ord_num(&es, to(f64::NAN)).unwrap_err(), Num2Err::CannotConvert);
        assert_eq!(to_ord_num(&es, to(f64::INFINITY)).unwrap_err(), Num2Err::InfiniteOrdinal);
        assert_eq!(to_ord_num(&es, to(f64::NEG_INFINITY)).unwrap_err(), Num2Err::InfiniteOrdinal);

        // Year is the same as cardinal. Except when negative, it is appended with " a. C."
        let to_year = Language::to_year;
        assert_eq!(to_year(&es, to(0.001)).unwrap_err(), Num2Err::FloatingYear);
        assert_eq!(to_year(&es, to(f64::INFINITY)).unwrap_err(), Num2Err::InfiniteYear);
        assert_eq!(to_year(&es, to(f64::NEG_INFINITY)).unwrap_err(), Num2Err::InfiniteYear);
        assert_eq!(to_year(&es, to(f64::NAN)).unwrap_err(), Num2Err::CannotConvert);
        assert_eq!(to_year(&es, to(0)).unwrap_err(), Num2Err::CannotConvert); // Year 0 is not a thing afaik
    }

    #[test]
    fn lang_es_year_is_similar_to_cardinal() {
        let es = Spanish::default();

        assert_eq!(es.to_year(to(2021)).unwrap(), "dos mil veintiuno");
        assert_eq!(es.to_year(to(-2021)).unwrap(), "dos mil veintiuno a. C.");
        let two = BigFloat::from(2);
        for num in (3u64..).take(60).map(|num| two.pow(&to(num))) {
            assert_eq!(es.to_year(num).unwrap(), es.to_cardinal(num).unwrap())
        }
    }

    #[test]
    fn lang_es_un_is_for_single_unit() {
        // Triplets ending in 1 but higher than 30, is never "un"
        // consequently should never contain " un " as substring anywhere unless proven otherwise
        let es = Spanish::default();
        assert_eq!(
            es.int_to_cardinal(to(171_031_091_031.0)).unwrap(),
            "ciento setenta y uno billones treinta y uno millones noventa y uno mil treinta y uno",
        );
        assert!(!es.int_to_cardinal(to(171_031_091_031.0)).unwrap().contains(" un "));
    }
    #[test]
    fn lang_es_with_veinte_flavor() {
        // with veinte flavour
        let es = Spanish::default().with_veinte(true);
        assert_eq!(
            es.int_to_cardinal(to(21_021_321_021.0)).unwrap(),
            "veinte y un billones veinte y un millones trescientos veinte y un mil veinte y uno"
        );
        assert_eq!(es.int_to_cardinal(to(22_000_000)).unwrap(), "veinte y dos millones");
        assert_eq!(
            es.int_to_cardinal(to(20_020_020)).unwrap(),
            "veinte millones veinte mil veinte"
        );
    }

    #[test]
    fn lang_es_ordinal() {
        let es = Spanish::default().with_feminine(true);
        let ordinal = |num: i128| es.to_ordinal(to(num)).unwrap();
        assert_eq!(ordinal(1_101_001), "primer millonésima centésima primera milésima primera");
        assert_eq!(ordinal(2_001_022), "segunda millonésima primer milésima vigésima segunda");
        assert_eq!(
            ordinal(12_114_011),
            "duodécima millonésima centésima decimocuarta milésima undécima"
        );
        assert_eq!(
            ordinal(124_121_091),
            "centésima vigésima cuarta millonésima centésima vigésima primera milésima nonagésima \
             primera"
        );
        let es = Spanish::default().with_plural(true);
        let ordinal = |num: i128| es.to_ordinal(to(num)).unwrap();
        assert_eq!(
            ordinal(124_001_091),
            "centésimo vigésimo cuarto millonésimo primer milésimo nonagésimo primeros"
        );
    }

    #[test]
    fn lang_es_with_fraction() {
        use DecimalChar::{Coma, Punto};
        let es = Spanish::default().with_decimal_char(Punto);
        assert_eq!(
            es.to_cardinal(BigFloat::from(1.0123456789)).unwrap(),
            "uno punto cero uno dos tres cuatro cinco seis siete ocho nueve"
        );
        let es = es.with_decimal_char(Coma);
        assert_eq!(
            es.to_cardinal(BigFloat::from(0.0123456789)).unwrap(),
            "cero coma cero uno dos tres cuatro cinco seis siete ocho nueve"
        );
        // Negative flavours
        use NegativeFlavour::{Appended, BelowZero, Prepended};
        let es = es.with_neg_flavour(Appended);
        assert_eq!(
            es.to_cardinal(BigFloat::from(-0.0123456789)).unwrap(),
            "cero coma cero uno dos tres cuatro cinco seis siete ocho nueve negativo"
        );
        let es = es.with_neg_flavour(Prepended);
        assert_eq!(
            es.to_cardinal(BigFloat::from(-0.0123456789)).unwrap(),
            "menos cero coma cero uno dos tres cuatro cinco seis siete ocho nueve"
        );
        let es = es.with_neg_flavour(BelowZero);
        assert_eq!(
            es.to_cardinal(BigFloat::from(-0.0123456789)).unwrap(),
            "cero coma cero uno dos tres cuatro cinco seis siete ocho nueve bajo cero"
        );
    }

    #[test]
    fn lang_es_infinity_and_negatives() {
        use NegativeFlavour::*;
        let flavours: [NegativeFlavour; 3] = [Prepended, Appended, BelowZero];
        let neg = f64::NEG_INFINITY;
        let pos = f64::INFINITY;
        for flavour in flavours.iter().cloned() {
            let es = Spanish::default().with_neg_flavour(flavour);
            match flavour {
                Prepended => {
                    assert_eq!(es.to_cardinal(neg.into()).unwrap(), "menos infinito");
                    assert_eq!(es.to_cardinal(pos.into()).unwrap(), "infinito");
                }
                Appended => {
                    assert_eq!(es.to_cardinal(neg.into()).unwrap(), "infinito negativo");
                    assert_eq!(es.to_cardinal(pos.into()).unwrap(), "infinito");
                }
                BelowZero => {
                    assert_eq!(es.to_cardinal(neg.into()).unwrap(), "menos infinito");
                    assert_eq!(es.to_cardinal(pos.into()).unwrap(), "infinito");
                }
            }
        }
    }

    #[test]
    fn lang_es_millions() {
        let es = Spanish::default();
        // When thousands triplet is 1
        assert_eq!(es.int_to_cardinal(to(1_001_000)).unwrap(), "un millón mil");
        assert_eq!(es.int_to_cardinal(to(10_001_010)).unwrap(), "diez millones mil diez");
        assert_eq!(es.int_to_cardinal(to(19_001_010)).unwrap(), "diecinueve millones mil diez");
        assert_eq!(
            es.int_to_cardinal(to(801_001_001)).unwrap(),
            "ochocientos uno millones mil uno"
        );
        assert_eq!(es.int_to_cardinal(to(800_001_001)).unwrap(), "ochocientos millones mil uno");
        // when thousands triplet isn't 1
        assert_eq!(es.int_to_cardinal(to(1_002_010)).unwrap(), "un millón dos mil diez");
        assert_eq!(es.int_to_cardinal(to(10_002_010)).unwrap(), "diez millones dos mil diez");
        assert_eq!(
            es.int_to_cardinal(to(19_102_010)).unwrap(),
            "diecinueve millones ciento dos mil diez"
        );
        assert_eq!(
            es.int_to_cardinal(to(800_100_001)).unwrap(),
            "ochocientos millones cien mil uno"
        );
        assert_eq!(
            es.int_to_cardinal(to(801_021_001)).unwrap(),
            "ochocientos uno millones veintiún mil uno"
        );
        assert_eq!(es.int_to_cardinal(to(1_000_000)).unwrap(), "un millón");
        assert_eq!(es.int_to_cardinal(to(1_000_000_000)).unwrap(), "un billón");
        assert_eq!(
            es.int_to_cardinal(to(1_001_100_001)).unwrap(),
            "un billón un millón cien mil uno"
        );
    }

    #[test]
    fn lang_es_negative_prepended() {
        let mut es = Spanish::default();
        // Make sure no enums were accidentally missed in tests if flavour ever changes
        match es.neg_flavour {
            NegativeFlavour::Prepended => (),
            NegativeFlavour::Appended => (),
            NegativeFlavour::BelowZero => (),
        }

        use NegativeFlavour::*;
        es.set_neg_flavour(Appended);
        assert_eq!(es.int_to_cardinal((-1).into()).unwrap(), "uno negativo");
        assert_eq!(es.int_to_cardinal((-1_000_000).into()).unwrap(), "un millón negativo");
        assert_eq!(
            es.int_to_cardinal((-1_020_010_000).into()).unwrap(),
            "un billón veinte millones diez mil negativo"
        );

        es.set_neg_flavour(Prepended);
        assert_eq!(es.int_to_cardinal((-1).into()).unwrap(), "menos uno");
        assert_eq!(es.int_to_cardinal((-1_000_000).into()).unwrap(), "menos un millón");
        assert_eq!(
            es.int_to_cardinal((-1_020_010_000).into()).unwrap(),
            "menos un billón veinte millones diez mil"
        );

        es.set_neg_flavour(BelowZero);
        assert_eq!(es.int_to_cardinal((-1).into()).unwrap(), "uno bajo cero");
        assert_eq!(es.int_to_cardinal((-1_000_000).into()).unwrap(), "un millón bajo cero");
        assert_eq!(
            es.int_to_cardinal((-1_020_010_000).into()).unwrap(),
            "un billón veinte millones diez mil bajo cero"
        );
    }

    #[test]
    fn lang_es_positive_is_just_a_substring_of_negative_in_cardinal() {
        const VALUES: [i128; 3] = [-1, -1_000_000, -1_020_010_000];
        use NegativeFlavour::*;
        let mut es = Spanish::default();
        for flavour in [Prepended, Appended, BelowZero] {
            es.set_neg_flavour(flavour);
            for value in VALUES.iter().cloned() {
                let positive = es.int_to_cardinal(to(value).abs()).unwrap();
                let negative = es.int_to_cardinal(-to(value).abs()).unwrap();
                assert!(
                    negative.contains(positive.as_str()),
                    "{} !contains {}",
                    negative,
                    positive
                );
            }
        }
    }
}
