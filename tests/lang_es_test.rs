use num2words::{Currency, Lang, Num2Err, Num2Words};
use num_bigfloat::BigFloat;
enum Outputs {
    Cardinal,
    Ordinal,
    OrdinalNum,
    Year,
    Currency,
}
fn to_words(num: BigFloat, output: Outputs, preference: &[&str]) -> Result<String, Num2Err> {
    let mut driver = Num2Words::new(num).lang(Lang::Spanish);
    for preference in preference.iter() {
        driver = driver.prefer(preference.to_string());
    }
    let driver = match output {
        Outputs::Cardinal => driver.cardinal(),
        Outputs::Ordinal => driver.ordinal(),
        Outputs::OrdinalNum => driver.ordinal_num(),
        Outputs::Year => driver.year(),
        Outputs::Currency => driver.currency(Currency::USD),
    };
    driver.to_words()
}
#[test]
fn test_lang_es() {
    let prefs_basics =
        ["negativo" /* , "veinte", "menos", "prepended", "appended", "bajo cero" */];
    let prefs_for_ordinals = vec!["femenino" /* "f", "feminine", */, "plural"];
    let prefs_for_decimal_char = vec!["coma"];

    let driver = |output: Outputs, num: BigFloat| {
        to_words(
            num,
            output,
            prefs_basics
                .iter()
                .chain(&prefs_for_decimal_char)
                .copied()
                .collect::<Vec<_>>()
                .as_slice(),
        )
    };
    let word = driver(Outputs::Cardinal, BigFloat::from(-821_442_524.69)).unwrap();
    assert_eq!(
        word,
        "ochocientos veintiún millones cuatrocientos cuarenta y dos mil quinientos veinticuatro \
         coma seis nueve negativo"
    );
    let word = driver(Outputs::Ordinal, BigFloat::from(-484));
    assert!(word.is_err()); // You can't get the ordinal of a negative number

    let driver =
        |output: Outputs, num: BigFloat| to_words(num, output, prefs_for_ordinals.as_slice());

    // let driver = to_language(Lang::Spanish, prefs_for_ordinals.clone());
    // let word = ;
    assert_eq!(driver(Outputs::Ordinal, BigFloat::from(14)).unwrap(), "decimocuartas");
    assert_eq!(driver(Outputs::Ordinal, BigFloat::from(1)).unwrap(), "primeras");
    assert_eq!(driver(Outputs::Ordinal, BigFloat::from(2)).unwrap(), "segundas");

    let driver = |output: Outputs, num: BigFloat| to_words(num, output, &[]);
    let word = driver(Outputs::Ordinal, BigFloat::from(141_100_211_021u64)).unwrap();
    assert_eq!(
        word,
        "centésimo cuadragésimo primero billonésimo centésimo millonésimo ducentésimo undécimo \
         milésimo vigésimo primero"
    );
    assert_eq!(driver(Outputs::Ordinal, BigFloat::from(14)).unwrap(), "decimocuarto");
    assert_eq!(driver(Outputs::Ordinal, BigFloat::from(1)).unwrap(), "primero");
    assert_eq!(driver(Outputs::Ordinal, BigFloat::from(2)).unwrap(), "segundo");
    assert_eq!(driver(Outputs::Ordinal, BigFloat::from(3)).unwrap(), "tercero");

    let word = to_words(BigFloat::from(14), Outputs::OrdinalNum, &["f"]);
    assert_eq!(word.unwrap(), "14ª");
    let word = to_words(BigFloat::from(14), Outputs::OrdinalNum, &[]);
    assert_eq!(word.unwrap(), "14º");

    let word = to_words(BigFloat::from(2021), Outputs::Year, &[]);
    assert_eq!(word.unwrap(), "dos mil veintiuno");
    let word = to_words(BigFloat::from(-2021), Outputs::Year, &[]);
    assert_eq!(word.unwrap(), "dos mil veintiuno a. C.");

    let word = to_words(BigFloat::from(21_001.21), Outputs::Currency, &[]);
    assert_eq!(word.unwrap(), "veintiún mil un US dollars con veintiún centavos");

    let word = to_words(BigFloat::from(21.01), Outputs::Currency, &[]);
    assert_eq!(word.unwrap(), "veintiún US dollars con un centavo");
}
