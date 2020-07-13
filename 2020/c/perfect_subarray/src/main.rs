use std::io;
use std::str;

/// Reads white-space separated tokens one at a time.
pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    /// Use "turbofish" syntax token::<T>() to select data type of next token.
    ///
    /// # Panics
    ///
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

const LOG_TABLE_U32: [u32; 32] = [
    0, 9, 1, 10, 13, 21, 2, 29, 11, 14, 16, 18, 22, 25, 3, 30, 8, 12, 20, 28, 15, 17, 24, 7, 19,
    27, 23, 6, 26, 5, 4, 31,
];

fn log2_u32(x: u32) -> u32 {
    // Based on http://guihaire.com/code/?p=414
    let mut value = x;
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    return LOG_TABLE_U32[(value.wrapping_mul(0x07C4ACDD) >> 27) as usize];
}

fn guess_square_root(x: i32) -> i32 {
    // Based on https://en.wikipedia.org/wiki/Methods_of_computing_square_roots.
    // We shift off the floor of half the bits.
    let x = x as u32;
    let n = log2_u32(x);
    return 1 << (n / 2);
}

fn sqrt_i32(a: i32) -> Option<i32> {
    if a == 0 { return Some(0); }
    match a % 10 {
        2 | 3 | 7 | 8 => return None,
        _ => {}
    }

    // Newton's method where f(x) = x * x - a.
    // Based on https://math.stackexchange.com/a/41355.
    // Also see https://en.wikipedia.org/wiki/Integer_square_root.
    let mut x_n = guess_square_root(a);
    loop {
        let x_n_1: i32 = (x_n * x_n + a) / (2 * x_n);
        let guess = x_n_1 * x_n_1;

        if guess == a {
            return Some(x_n_1);
        } else if (x_n_1 - x_n).abs() <= 1 {
            return None;
        }

        x_n = x_n_1;
    }
}

fn sqrt_binary_search_i64(a: i64) -> Option<i64> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn log2_u32_low() {
        assert_eq!(log2_u32(1), 0);
        assert_eq!(log2_u32(2), 1);
        assert_eq!(log2_u32(3), 1);
        assert_eq!(log2_u32(4), 2);
        assert_eq!(log2_u32(5), 2);
        assert_eq!(log2_u32(6), 2);
        assert_eq!(log2_u32(7), 2);
        assert_eq!(log2_u32(8), 3);
        assert_eq!(log2_u32(9), 3);
        assert_eq!(log2_u32(15), 3);
        assert_eq!(log2_u32(16), 4);
        assert_eq!(log2_u32(31), 4);
        assert_eq!(log2_u32(32), 5);
    }

    #[test]
    fn guess_square_root_low() {
        assert_eq!(guess_square_root(1), 1);
        assert_eq!(guess_square_root(2), 1);
        assert_eq!(guess_square_root(3), 1);
        assert_eq!(guess_square_root(4), 2);
        assert_eq!(guess_square_root(5), 2);
        assert_eq!(guess_square_root(6), 2);
        assert_eq!(guess_square_root(7), 2);
        assert_eq!(guess_square_root(8), 2);
        assert_eq!(guess_square_root(9), 2);
        assert_eq!(guess_square_root(15), 2);
        assert_eq!(guess_square_root(16), 4);
        assert_eq!(guess_square_root(31), 4);
        assert_eq!(guess_square_root(32), 4);
        assert_eq!(guess_square_root(63), 4);
        assert_eq!(guess_square_root(64), 8);
    }

    #[test]
    fn sqrt_i32_low() {
        for x in 0..512 {
            let result = sqrt_i32(x);
            match result {
                Some(sqrt) => assert_eq!(sqrt * sqrt, x),
                None => assert_ne!((x as f32).sqrt().fract(), 0.0_f32),
            }
        }
    }

    #[test]
    fn sqrt_i32_mid() {
        for x in 512..1024 {
            let result = sqrt_i32(x);
            match result {
                Some(sqrt) => assert_eq!(sqrt * sqrt, x),
                None => assert_ne!((x as f32).sqrt().fract(), 0.0_f32),
            }
        }
    }

    #[test]
    fn sqrt_i32_high_true() {
        assert_eq!(sqrt_i32(30858025), Some(5555));
        assert_eq!(sqrt_i32(44435556), Some(6666));
        assert_eq!(sqrt_i32(100000000), Some(10000));
        // assert_eq!(sqrt_i32(987656329), Some(31427));
        // assert_eq!(sqrt_i32(152399025), Some(12345));
    }

    #[test]
    fn sqrt_i32_high_false() {
        assert_eq!(sqrt_i32(30858026), None);
        assert_eq!(sqrt_i32(30823412), None);
        assert_eq!(sqrt_i32(59109310), None);
        // assert_eq!(sqrt_i32(123456789), None);
        // assert_eq!(sqrt_i32(987654321), None);
    }

    #[test] #[ignore]
    fn sqrt_i32_benchmark() {
        // This should be run in release mode, otherwise it takes around 12
        // seconds
        let instant = Instant::now();
        for x in 0..100000000 {
            sqrt_i32(x);
        }

        println!(
            "Time elapsed: {}",
            instant.elapsed().as_micros() as f64 / 1e6
        );
    }

    #[test]
    fn sqrt_binary_search_i64_low() {
        for x in 0..512 {
            let result = sqrt_binary_search_i64(x);
            match result {
                Some(sqrt) => assert_eq!(sqrt * sqrt, x),
                None => assert_ne!((x as f32).sqrt().fract(), 0.0_f32),
            }
        }
    }

    #[test] #[ignore]
    fn sqrt_binary_search_i64_benchmark() {
        // Currently the loop gets completely optimized out, which is not good
        // for benchmarking.
        // TODO: Use test::bench
        let instant = Instant::now();
        for x in 0..65536 {
            sqrt_binary_search_i64(x);
        }

        println!(
            "Time elapsed: {}",
            instant.elapsed().as_micros() as f64 / 1e6
        );
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    let t: i32 = scanner.token();
    for ti in 0..t {
        let n: i32 = scanner.token();

        let mut n_perfect = 0;
        let mut arr: Vec<i64> = Vec::with_capacity(n as usize);

        for _ in 0..n {
            let x: i64 = scanner.token();

            // Check them now while we have them
            if let Some(_) = sqrt_binary_search_i64(x) {
                n_perfect += 1;
            }

            arr.push(x);
        }

        let mut prev_sub_sums = arr.clone();

        for sub_size in 2..(n + 1) {
            let n_subs = n - sub_size + 1;
            let mut sub_sums: Vec<i64> = Vec::with_capacity(n_subs as usize);

            for i in 0..n_subs {
                let sub_sum = prev_sub_sums[i as usize] + arr[(i + sub_size - 1) as usize];
                if let Some(_) = sqrt_binary_search_i64(sub_sum) {
                    n_perfect += 1;
                }

                sub_sums.push(sub_sum);
            }

            prev_sub_sums = sub_sums;
        }

        println!("Case #{}: {}", ti + 1, n_perfect);
    }
}
