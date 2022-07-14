use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wcore::{evaluator::Evaluate, models::*};

fn factorial(n: f64) -> f64 {
    let code = format!(
        "
factorial <-|
  $0 2 < -> 1
  $0 $0 1 sub factorial mul

{} factorial
",
        n
    );
    let mut state = State::new();
    match state.apply(&code)[0][0] {
        Token::Value(x) => x,
        _ => panic!("Invalid response!"),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("factorial", |b| b.iter(|| factorial(black_box(20.))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
