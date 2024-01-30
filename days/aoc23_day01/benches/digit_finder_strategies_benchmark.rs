use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc23_day01::strategries::{iterator_digit_finder, two_pointer_digit_finder, DigitFinder};

fn digit_finder_strategies_benchmark(c: &mut Criterion) {
    let short = "three4two4rnnslsvxmsbcpvnbpfseveneightwokcn";
    let iterator = iterator_digit_finder::IteratorDigitFinder {};
    let two_pointer = two_pointer_digit_finder::TwoPointersDigitFinder {};

    c.bench_function("[Short Text] Iterator Strategy", |b| b.iter(|| iterator.find_digits(black_box(short))));
    c.bench_function("[Short Text] Two Pointers Strategy", |b| b.iter(|| two_pointer.find_digits(black_box(short))));

    let pattern = "threetwornnslsvxmsbcpvnbpfeveneightwokcn";
    let long = pattern.repeat(100) + "1" + &pattern.repeat(100);

    c.bench_function("[Long Message] Iterator Strategy", |b| b.iter(|| iterator.find_digits(black_box(&long))));
    c.bench_function("[Long Message] Two Pointers Strategy", |b| b.iter(|| two_pointer.find_digits(black_box(&long))));
}

criterion_group!(benches, digit_finder_strategies_benchmark);
criterion_main!(benches);
