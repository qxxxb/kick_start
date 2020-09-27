use std::cmp;
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

fn hidden(n: usize, a_desired: usize, b_desired: usize, c: usize) -> Option<usize> {
    let hidden = (n as i32) - (a_desired as i32) - (b_desired as i32) + (c as i32);
    if hidden < 0 {
        None
    } else {
        Some(hidden as usize)
    }
}

fn calc(n: usize, a_desired: usize, b_desired: usize, c: usize) -> Option<Vec<u32>> {
    let hidden = hidden(n, a_desired, b_desired, c)?;

    let mut xs = vec![1; n];

    let n_maxes = c;

    let a = n_maxes;
    let b = n_maxes;

    let a_to_add = a_desired - a;
    let b_to_add = b_desired - b;

    let bookends = n - hidden - c;
    if bookends == 0 && hidden > 0 {
        return None;
    }

    let hide_on_a = a_to_add >= b_to_add;

    let mut a_start_i = 0;

    for _ in 0..a_to_add {
        xs[a_start_i] = (n as u32) - 1;
        a_start_i += 1;
    }

    if hide_on_a {
        for _ in 0..hidden {
            xs[a_start_i] = 1;
            a_start_i += 1;
        }
    }

    let c_start_i = a_start_i;
    for i in 0..n_maxes {
        let index = i + c_start_i;
        xs[index] = n as u32;
    }

    let mut b_start_i = c_start_i + n_maxes;

    if !hide_on_a {
        for _ in 0..hidden {
            xs[b_start_i] = 1;
            b_start_i += 1;
        }
    }

    for _ in 0..b_to_add {
        xs[b_start_i] = (n as u32) - 1;
        b_start_i += 1;
    }

    Some(xs)
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    let t: u32 = scanner.token();

    for ti in 0..t {
        let n: usize = scanner.token();
        let a_desired: usize = scanner.token();
        let b_desired: usize = scanner.token();
        let c: usize = scanner.token();

        let result = calc(n, a_desired, b_desired, c);

        print!("Case #{}: ", ti + 1);

        match result {
            Some(xs) => {
                for (i, x) in xs.iter().enumerate() {
                    print!("{}", x);
                    if i < ((n - 1) as usize) {
                        print!(" ");
                    }
                }
                println!();
            }
            None => {
                println!("IMPOSSIBLE");
            }
        }
    }
}

fn check(n: usize, a_desired: usize, b_desired: usize, c: usize, xs: Option<Vec<u32>>) -> bool {
    let should_be_impossible = {
        // TODO
        let hidden = (n as i32) - (a_desired as i32) - (b_desired as i32) + (c as i32);
        hidden < 0
    };

    match xs {
        None => should_be_impossible,
        Some(xs) => {
            // TODO
            let mut max = xs[0];
            let mut n_maxes = 1;
            for x in xs {
                if x > max {
                    n_maxes = 1;
                } else if x == max {
                    n_maxes += 1;
                }
            }

            true
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn check_calc() {
        for n in 1..6 {
            for c in 1..(n + 1) {
                for a in c..(n + 1) {
                    for b in c..(n + 1) {
                        let xs = calc(n, a, b, c);
                        assert!(check(n, a, b, c, xs));
                    }
                }
            }
        }
    }
}
