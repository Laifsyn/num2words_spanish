use num2words::lang::to_language;
use num2words::Lang;
use num_bigfloat::BigFloat;

#[test]
fn test_lang_es() {
    let prefs_basics: Vec<String> =
        vec!["negativo" /* , "veinte", "menos", "prepended", "appended", "bajo cero" */]
            .into_iter()
            .map(String::from)
            .collect();
    let prefs_for_ordinals: Vec<String> =
        vec!["femenino", /* "f", "feminine", */ "plural"].into_iter().map(String::from).collect();
    let prefs_for_decimal_char: Vec<String> = vec!["coma"].into_iter().map(String::from).collect();

    let driver = to_language(
        Lang::Spanish,
        prefs_basics.iter().chain(&prefs_for_decimal_char).cloned().collect(),
    );
    let word = driver.to_cardinal(BigFloat::from(-821_442_524.69)).unwrap();
    #[rustfmt::skip]
    assert_eq!(word, "ochocientos veintiún millones cuatrocientos cuarenta y dos mil quinientos veinticuatro coma seis nueve negativo");
    let word = driver.to_ordinal(BigFloat::from(-484));
    assert!(word.is_err()); // You can't get the ordinal of a negative number

    let driver = to_language(Lang::Spanish, prefs_for_ordinals.clone());
    assert_eq!(driver.to_ordinal(14.into()).unwrap(), "decimocuartas");
    assert_eq!(driver.to_ordinal(1.into()).unwrap(), "primeras");
    assert_eq!(driver.to_ordinal(2.into()).unwrap(), "segundas");

    let driver = to_language(Lang::Spanish, vec![]);
    assert_eq!(
        driver.to_ordinal(141_100_211_021u64.into()).unwrap(),
        "centésimo cuadragésimo primero billonésimo centésimo millonésimo ducentésimo undécimo \
         milésimo vigésimo primero"
    );
    assert_eq!(driver.to_ordinal(14.into()).unwrap(), "decimocuarto");
    assert_eq!(driver.to_ordinal(1.into()).unwrap(), "primero");
    assert_eq!(driver.to_ordinal(2.into()).unwrap(), "segundo");
    assert_eq!(driver.to_ordinal(3.into()).unwrap(), "tercero");
}
