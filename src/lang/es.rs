use core::fmt::{self, Formatter};
use std::convert::TryInto;
use std::fmt::Display;
// Reference that can hopefully be implemented seamlessly: https://es.wikipedia.org/wiki/Anexo:Nombres_de_los_n%C3%BAmeros_en_espa%C3%B1ol
const UNIDADES: [&str; 10] =
    ["", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve"];
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
const MILLAR_SIZE: usize = 22;
/// from source: https://es.wikipedia.org/wiki/Anexo:Nombres_de_los_n%C3%BAmeros_en_espa%C3%B1ol
/// Based on https://en.wikipedia.org/wiki/Names_of_large_numbers, each thousands is from the Short Scales,
/// which each thousands can be defined as 10^(3n+3) magnitude, where n is replaced by the index of
/// the Array. For example 10^3 = Thousands (starts at n=1 here)
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
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Spanish {
    neg_flavour: NegativeFlavour,
}
#[allow(dead_code)]
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum NegativeFlavour {
    #[default]
    Prepended, // -1 => menos uno
    Appended,  // -1 => uno negativo
    BelowZero, // -1 => uno bajo cero
}

impl Display for NegativeFlavour {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            NegativeFlavour::Prepended => write!(f, "menos"),
            NegativeFlavour::Appended => write!(f, "negativo"),
            NegativeFlavour::BelowZero => write!(f, "bajo cero"),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Spanish {
    neg_flavour: NegativeFlavour,
}
#[allow(dead_code)]
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum NegativeFlavour {
    #[default]
    Prepended, // -1 => menos uno
    Appended,  // -1 => uno negativo
    BelowZero, // -1 => uno bajo cero
}

impl Display for NegativeFlavour {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            NegativeFlavour::Prepended => write!(f, "menos"),
            NegativeFlavour::Appended => write!(f, "negativo"),
            NegativeFlavour::BelowZero => write!(f, "bajo cero"),
        }
    }
}

impl Spanish {
    pub fn set_neg_flavour(&mut self, flavour: NegativeFlavour) {
        self.neg_flavour = flavour;
    }

    fn en_miles(&self, mut num: i128) -> Vec<u16> {
    pub fn set_neg_flavour(&mut self, flavour: NegativeFlavour) {
        self.neg_flavour = flavour;
    }

    fn en_miles(&self, mut num: i128) -> Vec<u16> {
        let mut thousands = Vec::new();
        let mil = 1000;
        num = num.abs();
        num = num.abs();
        while num != 0 {
            // Insertar en Low Endian
            thousands.push((num % mil).try_into().expect("triplet not under 1000"));
            // Insertar en Low Endian
            thousands.push((num % mil).try_into().expect("triplet not under 1000"));
            num /= mil; // DivAssign
        }
        thousands
    }

    pub fn to_cardinal(&self, num: i128) -> Result<String, String> {
        // for 0 case
        if num == 0 {
            return Ok(String::from("cero"));
    pub fn to_cardinal(&self, num: i128) -> Result<String, String> {
        // for 0 case
        if num == 0 {
            return Ok(String::from("cero"));
        }

        let mut words = vec![];
        for (i, triplet) in self.en_miles(num).iter().enumerate().rev() {
            let hundreds = ((triplet / 100) % 10) as usize;
            let tens = ((triplet / 10) % 10) as usize;
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
            'decenas: {
                if tens != 0 || units != 0 {
                    let unit_word = match (units, i) {
                        // case `1_100` => `mil cien` instead of `un mil un cien`
                        // case `1_001_000` => `un millón mil` instead of `un millón un mil`
                        (_, 1) if triplet == &1 => break 'decenas,
                        /*
                        // TODO: uncomment this Match Arm if it's more correct to say "un millón mil" for 1_001_000
                        (1, 1) => {
                            // Early break to avoid "un millón un mil" which personally sounds unnatural
                            break 'decenas;
                        }, */
                        // case `001_001_100...` => `un billón un millón cien mil...` instead of
                        // `uno billón uno millón cien mil...`
                        (_, index) if index != 0 && triplet == &1 => "un",
                        _ => UNIDADES[units],
                    };

                    match tens {
                        // case `?_102` => `? ciento dos`
                        0 => words.push(String::from(unit_word)),
                        // case `?_119` => `? ciento diecinueve`
                        // case `?_110` => `? ciento diez`
                        1 => words.push(String::from(DIECIS[units])),
                        _ => {
                            // case `?_142 => `? cuarenta y dos`
                            let ten = DECENAS[tens];
                            words.push(match units {
                                0 => String::from(ten),
                                _ => format!("{ten} y {unit_word}"),
                            });
                        }
                    }
                }
            }
            // Add the next Milliard if there's any.
            if i != 0 && triplet != &0 {
                if i > MILLARES.len() - 1 {
                    return Err(format!("Número demasiado grande: {} - Maximo: {}", num, i32::MAX));
                if i > MILLARES.len() - 1 {
                    return Err(format!("Número demasiado grande: {} - Maximo: {}", num, i32::MAX));
                }
                // Boolean that checks if next Milliard is plural
                let plural = *triplet != 1;
                // Boolean that checks if next Milliard is plural
                let plural = *triplet != 1;
                match plural {
                    false => words.push(String::from(MILLAR[i])),
                    true => words.push(String::from(MILLARES[i])),
                }
            }
        }
        // flavour the text when negative
        if let (flavour, true) = (&self.neg_flavour, num < 0) {
            use NegativeFlavour::*;
            let string = flavour.to_string();
            match flavour {
                Prepended => words.insert(0, string),
                Appended => words.push(string),
                BelowZero => words.push(string),
            }
        }

        // flavour the text when negative
        if let (flavour, true) = (&self.neg_flavour, num < 0) {
            use NegativeFlavour::*;
            let string = flavour.to_string();
            match flavour {
                Prepended => words.insert(0, string),
                Appended => words.push(string),
                BelowZero => words.push(string),
            }
        }

        Ok(words.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lang_es_sub_thousands() {
        let es = Spanish::default();
        assert_eq!(es.to_cardinal(000).unwrap(), "cero");
        assert_eq!(es.to_cardinal(10).unwrap(), "diez");
        assert_eq!(es.to_cardinal(100).unwrap(), "cien");
        assert_eq!(es.to_cardinal(101).unwrap(), "ciento uno");
        assert_eq!(es.to_cardinal(110).unwrap(), "ciento diez");
        assert_eq!(es.to_cardinal(111).unwrap(), "ciento once");
        assert_eq!(es.to_cardinal(141).unwrap(), "ciento cuarenta y uno");
        assert_eq!(es.to_cardinal(142).unwrap(), "ciento cuarenta y dos");
        assert_eq!(es.to_cardinal(800).unwrap(), "ochocientos");
    }

    #[test]
    fn lang_es_thousands() {
        let es = Spanish::default();
        // When thousands triplet is 1
        assert_eq!(es.to_cardinal(1_000).unwrap(), "mil");
        assert_eq!(es.to_cardinal(1_010).unwrap(), "mil diez");
        assert_eq!(es.to_cardinal(1_100).unwrap(), "mil cien");
        assert_eq!(es.to_cardinal(1_101).unwrap(), "mil ciento uno");
        assert_eq!(es.to_cardinal(1_110).unwrap(), "mil ciento diez");
        assert_eq!(es.to_cardinal(1_111).unwrap(), "mil ciento once");
        assert_eq!(es.to_cardinal(1_141).unwrap(), "mil ciento cuarenta y uno");
        // When thousands triplet isn't 1
        assert_eq!(es.to_cardinal(2_000).unwrap(), "dos mil");
        assert_eq!(es.to_cardinal(12_010).unwrap(), "doce mil diez");
        assert_eq!(es.to_cardinal(140_100).unwrap(), "ciento cuarenta mil cien");
        assert_eq!(es.to_cardinal(141_101).unwrap(), "ciento cuarenta y uno mil ciento uno");
        assert_eq!(es.to_cardinal(142_002).unwrap(), "ciento cuarenta y dos mil dos");
        assert_eq!(es.to_cardinal(142_000).unwrap(), "ciento cuarenta y dos mil");
        assert_eq!(es.to_cardinal(888_111).unwrap(), "ochocientos ochenta y ocho mil ciento once");
        assert_eq!(es.to_cardinal(800_000).unwrap(), "ochocientos mil");
    }

    #[test]
    fn lang_es_millions() {
        let es = Spanish::default();
        // When thousands triplet is 1
        assert_eq!(es.to_cardinal(1_001_000).unwrap(), "un millón mil");
        assert_eq!(es.to_cardinal(10_001_010).unwrap(), "diez millones mil diez");
        assert_eq!(es.to_cardinal(19_001_010).unwrap(), "diecinueve millones mil diez");
        assert_eq!(es.to_cardinal(801_001_001).unwrap(), "ochocientos uno millones mil uno");
        assert_eq!(es.to_cardinal(800_001_001).unwrap(), "ochocientos millones mil uno");
        // when thousands triplet isn't 1
        assert_eq!(es.to_cardinal(1_002_010).unwrap(), "un millón dos mil diez");
        assert_eq!(es.to_cardinal(10_002_010).unwrap(), "diez millones dos mil diez");
        assert_eq!(es.to_cardinal(19_102_010).unwrap(), "diecinueve millones ciento dos mil diez");
        assert_eq!(es.to_cardinal(800_100_001).unwrap(), "ochocientos millones cien mil uno");
        assert_eq!(
            es.to_cardinal(801_021_001).unwrap(),
            "ochocientos uno millones veinte y uno mil uno"
        );
        assert_eq!(es.to_cardinal(1_000_000).unwrap(), "un millón");
        assert_eq!(es.to_cardinal(1_000_000_000).unwrap(), "un billón");
        assert_eq!(es.to_cardinal(1_001_100_001).unwrap(), "un billón un millón cien mil uno");
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
        assert_eq!(es.to_cardinal(-1).unwrap(), "uno negativo");
        assert_eq!(es.to_cardinal(-1_000_000).unwrap(), "un millón negativo");
        assert_eq!(
            es.to_cardinal(-1_020_010_000).unwrap(),
            "un billón veinte millones diez mil negativo"
        );

        es.set_neg_flavour(Prepended);
        assert_eq!(es.to_cardinal(-1).unwrap(), "menos uno");
        assert_eq!(es.to_cardinal(-1_000_000).unwrap(), "menos un millón");
        assert_eq!(
            es.to_cardinal(-1_020_010_000).unwrap(),
            "menos un billón veinte millones diez mil"
        );

        es.set_neg_flavour(BelowZero);
        assert_eq!(es.to_cardinal(-1).unwrap(), "uno bajo cero");
        assert_eq!(es.to_cardinal(-1_000_000).unwrap(), "un millón bajo cero");
        assert_eq!(
            es.to_cardinal(-1_020_010_000).unwrap(),
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
                let positive = es.to_cardinal(value.abs()).unwrap();
                let negative = es.to_cardinal(-value.abs()).unwrap();
                assert!(
                    negative.contains(positive.as_str()),
                    "{} !contains {}",
                    negative,
                    positive
                );
            }
        }
    }

    #[test]
    fn lang_es_() {
        // unimplemented!()
    }
}