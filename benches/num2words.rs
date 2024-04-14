use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num2words::Num2Words;
use num_bigfloat::BigFloat;
// Criterion's quick start guide https://bheisler.github.io/criterion.rs/book/getting_started.html

pub fn criterion_benchmark(c: &mut Criterion) {
    //rewrite consts so they are instantiated here instead
    let quadrillions = BigFloat::from(121_121_121_121_121u64);
    let trillions = BigFloat::from(121_121_121_121u64);
    let billions = BigFloat::from(121_121_121);
    let millions = BigFloat::from(121_121_121);
    let thousands = BigFloat::from(121_121);
    let hundreds = BigFloat::from(121);
    use num2words::Lang::*;
    for (language, name) in [English, French, French_BE, French_CH, Spanish, Ukrainian]
        .map(|lang| (lang, format!("{lang:?}")))
    {
        c.bench_function(name.as_str(), |b| {
            b.iter(|| {
                for num in [
                    quadrillions,
                    trillions,
                    billions,
                    millions,
                    thousands,
                    hundreds,
                ]
                .iter()
                {
                    let driver = Num2Words::new(*num).lang(language);
                    black_box(driver.cardinal().to_words()).unwrap();
                }
            })
        });
    }
}
criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
