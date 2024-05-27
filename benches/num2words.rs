use std::time::Duration;

use criterion::measurement::WallTime;
use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion,
};
use num2words::{Currency, Num2Words};
use num_bigfloat::BigFloat;
// Criterion's quick start guide https://bheisler.github.io/criterion.rs/book/getting_started.html
fn group_with_config(bench_group: &mut BenchmarkGroup<'_, WallTime>) {
    bench_group.measurement_time(Duration::from_secs(20));
    bench_group.sample_size(1000);
    bench_group.confidence_level(0.99);
    bench_group.warm_up_time(Duration::from_secs(5));
}
pub fn criterion_benchmark(c: &mut Criterion) {
    let nonillion = BigFloat::from(121_121_121_121_121_121_121_121_121_121_121u128);
    let quadrillions = BigFloat::from(121_121_121_121_121_121u64);
    let trillions = BigFloat::from(121_121_121_121_121u64);
    let billions = BigFloat::from(121_121_121_121u64);
    let millions = BigFloat::from(121_121_121);
    let thousands = BigFloat::from(121_121);
    let hundreds = BigFloat::from(121);
    // from lowest to biggest
    let numbers = [hundreds, thousands, millions, billions, trillions, quadrillions, nonillion];
    let numbers_name =
        ["hundreds", "thousands", "millions", "billions", "trillions", "quadrillions", "nonillion"];
    use num2words::Lang::*;
    let numbers_and_name = numbers.iter().zip(numbers_name.iter()).collect::<Vec<_>>();
    let languages = [
        // (Spanish, "ES"),
        (English, "EN"),
        (French, "FR"),
        (French_BE, "FR_BE"),
        (French_CH, "FR_CH"),
        (Ukrainian, "UK"),
    ];
    let mut group = c.benchmark_group("Num2Words to_cardinal()");
    group_with_config(&mut group);

    for row_data in languages.iter().copied() {
        let (lang, language) = row_data;
        for (num, number) in numbers_and_name.iter() {
            let id = BenchmarkId::new(language, number);
            group.bench_with_input(id, *num, |b, num| {
                b.iter(|| {
                    let driver = Num2Words::new(*num).lang(lang);
                    black_box(driver.cardinal().to_words().unwrap());
                })
            });
        }
    }
    group.finish();

    // let mut group = c.benchmark_group("Num2Words to_ordinal()");
    // group_with_config(&mut group);

    // for row_data in languages.iter().copied() {
    //     let (lang, language) = row_data;
    //     for (num, number) in numbers_and_name.iter() {
    //         let id = BenchmarkId::new(language, number);
    //         group.bench_with_input(id, *num, |b, num| {
    //             b.iter(|| {
    //                 let driver = Num2Words::new(*num).lang(lang);
    //                 black_box(driver.ordinal().to_words().unwrap());
    //             })
    //         });
    //     }
    // }
    // group.finish();

    // // to currency
    // let mut group = c.benchmark_group("Num2Words to_currency(Currency::USD)");
    // group_with_config(&mut group);

    // for row_data in languages.iter().copied() {
    //     let (lang, language) = row_data;
    //     for (num, number) in numbers_and_name.iter() {
    //         let id = BenchmarkId::new(language, number);
    //         group.bench_with_input(id, *num, |b, num| {
    //             b.iter(|| {
    //                 let driver = Num2Words::new(*num).lang(lang);
    //                 black_box(driver.currency(Currency::USD).to_words().unwrap());
    //             })
    //         });
    //     }
    // }
    // group.finish();
}
criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
