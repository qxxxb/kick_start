use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::str;
use std::time::Instant;

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

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    /// This function should be marked unsafe, but nobody has time for that in a programming
    /// contest.
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn do_read(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(filename)?;
    let mut scanner = Scanner::new(io::BufReader::new(file));

    let n = scanner.token::<i32>();
    for _ in 0..n {
        let _ = scanner.token::<i32>();
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments");
    }
    let filename = &args[1];

    let instant = Instant::now();
    if let Err(error) = do_read(filename) {
        eprintln!("{:?}", error);
    }

    println!(
        "Time elapsed: {}",
        instant.elapsed().as_micros() as f64 / 1e6
    );
}
