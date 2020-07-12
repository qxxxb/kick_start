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

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    let t: i32 = scanner.token();

    for ti in 0..t {
        let n: i32 = scanner.token();
        let k: i32 = scanner.token();

        let mut a_i_prev = scanner.token();
        let mut n_countdowns = 0;
        let mut in_countdown = false;

        for _ in 1..n {
            let a_i: i32 = scanner.token();

            if a_i_prev == k {
                in_countdown = true;
            }

            if in_countdown {
                if a_i == a_i_prev - 1 {
                    if a_i == 1 {
                        n_countdowns += 1;
                        in_countdown = false;
                    }
                } else {
                    in_countdown = false;
                }
            }

            a_i_prev = a_i;
        }

        println!("Case #{}: {}", ti + 1, n_countdowns);
    }
}
