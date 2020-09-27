use std::io;
use std::str;
use std::cmp;

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

        let mut a_prev: i32 = 0;
        let mut jump: i32;
        let mut jump_prev: i32 = 0;
        let mut sub_size = 2;
        let mut max_sub_size = sub_size;

        for i in 0..n {
            let a_i: i32 = scanner.token();

            if i > 0 {
                jump = a_i - a_prev;

                if i > 1 {
                    if jump == jump_prev {
                        sub_size += 1;
                        max_sub_size = cmp::max(sub_size, max_sub_size);
                    } else {
                        sub_size = 2;
                    }
                }

                jump_prev = jump;
            }

            a_prev = a_i;
        }

        println!("Case #{}: {}", ti + 1, max_sub_size);
    }
}
