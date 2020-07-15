use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn sqrt_binary_search_i64(a: i64) -> Option<i64> {
    let mut bottom = 0;
    let mut top = a + 1;
    loop {
        let guess = (top + bottom) / 2;
        let guess_squared = guess * guess;
        if guess_squared == a {
            return Some(guess);
        } else {
            if top - bottom <= 1 {
                return None;
            } else if guess_squared < a {
                bottom = guess;
            } else if guess_squared > a {
                top = guess;
            }
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sqrt 20", |b| {
        b.iter(|| sqrt_binary_search_i64(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
