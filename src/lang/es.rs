// Reference that can hopefully be implemented seamlessly: https://es.wikipedia.org/wiki/Anexo:Nombres_de_los_n%C3%BAmeros_en_espa%C3%B1ol
const UNIDADES: [&str; 10] = [
    "", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve",
];
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
    "diez",
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
const MILLAR_SIZE: usize = 21; 
/// from source: https://es.wikipedia.org/wiki/Anexo:Nombres_de_los_n%C3%BAmeros_en_espa%C3%B1ol
/// Based on https://en.wikipedia.org/wiki/Names_of_large_numbers, each thousands is from the Short Scales,
/// which each thousands can be defined as 10^(3n+3) magnitude, where n is replaced by the index of the Array. For example
/// 10^3 = Thousands (starts at n=1 here)
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
pub struct Spanish {}
impl Spanish {
    fn en_miles(&self, mut num: i32) -> Vec<u64> {
        let mut thousands = Vec::new();
        let mil = 1000;

        while num != 0 {
            // Insertar en big-endian
            thousands.push((num % mil) as u64);
            num /= mil; // DivAssign
        }
        thousands
    }

    pub fn to_cardinal(&self, num: i32) -> Result<String, String> {
        match num {
            0 => return Ok(String::from("cero")),
            _ => (),
        }

        let mut words = vec![];
        for (i, triplet) in self.en_miles(num).iter().enumerate().rev() {
            let hundreds = (triplet / 100 % 10) as usize;
            let tens = (triplet / 10 % 10) as usize;
            let units = (triplet % 10) as usize;

            if hundreds > 0 {
                match triplet {
                    // Edge case when triplet is a hundred
                    100 => words.push(String::from("cien")),
                    _ => words.push(String::from(CENTENAS[hundreds])),
                }
            }

            if tens != 0 || units != 0 {
                // for edge case when unit value is 1 and is not the last triplet
                let unit_word = if units == 1 && i != 0 {
                    "un"
                } else {
                    UNIDADES[units]
                };
                match tens {
                    // case ?_102 => ? ciento dos
                    0 => words.push(String::from(unit_word)),
                    // case `?_119` => `? ciento diecinueve`
                    // case `?_110` => `? ciento diez`
                    1 => words.push(String::from(DIECIS[units])),
                    _ => {
                        // case 142 => CENTENAS[x] forty-two
                        let ten = DECENAS[tens];
                        words.push(match units {
                            0 => String::from(ten),
                            _ => format!("{ten} y {unit_word}"),
                        });
                    }
                }
            }

            if i != 0 && triplet != &0 {
                if i > (MILLARES.len() - 1) {
                    return Err(format!(
                        "Número demasiado grande: {} - Maximo: {}",
                        num,
                        i32::MAX
                    ));
                }
                // Boolean that checks if next MEGA/MILES is plural
                let plural = !(hundreds == 0 && tens == 0 && units == 1);
                match plural {
                    false => words.push(String::from(MILLAR[i])),
                    true => words.push(String::from(MILLARES[i])),
                }
            }
        }
        Ok(words.join(" "))
    }
}

pub fn main() {
    let es = Spanish {};
    println!("Resultado {:?}", es.to_cardinal(dbg!(1_002_002_031)));
    println!("Resultado {:?}", es.to_cardinal(dbg!(1_012_002_031)));
    println!("Resultado {:?}", es.to_cardinal(dbg!(1_011_002_031)));
}
