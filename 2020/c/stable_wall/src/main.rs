use std::clone::Clone;
use std::collections::HashSet;
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

struct Wall {
    data: Vec<String>,
    polyominos: HashSet<u8>,
    placed: HashSet<u8>,
    unplaced: HashSet<u8>,
}

impl Wall {
    fn new(rows: usize) -> Self {
        Wall {
            data: Vec::with_capacity(rows),
            polyominos: HashSet::new(),
            placed: HashSet::new(),
            unplaced: HashSet::new(),
        }
    }

    fn add_row(&mut self, row: String) {
        for x in row.bytes() {
            self.polyominos.insert(x);
        }
        self.data.push(row);
    }

    fn is_polyomino_supported_at_row(&self, polyomino: u8, row_index: usize) -> bool {
        let row = &self.data[row_index];
        let row_below = &self.data[row_index + 1];
        for (i, x) in row.bytes().enumerate() {
            if x == polyomino {
                if row_below.as_bytes()[i] != polyomino {
                    if !self.placed.contains(&row_below.as_bytes()[i]) {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    fn is_polyomino_supported(&self, polyomino: u8) -> bool {
        for r in (0..(self.data.len() - 1)).rev() {
            if !&self.is_polyomino_supported_at_row(polyomino, r) {
                return false;
            }
        }

        return true;
    }

    fn find_supported_polyomino_to_place(&self) -> Option<u8> {
        for polyomino in &self.unplaced {
            if (&self).is_polyomino_supported(*polyomino) {
                return Some(*polyomino);
            }
        }

        return None;
    }

    fn polyomino_placement_order(&mut self) -> Option<String> {
        self.unplaced = self.polyominos.clone();
        self.placed.clear();
        let mut result = String::new();

        loop {
            let p = &self.find_supported_polyomino_to_place();
            match p {
                &None => {
                    if self.unplaced.is_empty() {
                        return Some(result);
                    } else {
                        return None;
                    }
                }
                &Some(p) => {
                    self.placed.insert(p);
                    self.unplaced.remove(&p);
                    result.push(p as char);
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    let t: i32 = scanner.token();
    for ti in 0..t {
        // TODO
        let r: i32 = scanner.token();
        let _c: i32 = scanner.token();

        let mut wall = Wall::new(r as usize);

        for _ri in 0..r {
            let row: String = scanner.token();
            wall.add_row(row);
        }

        print!("Case #{}: ", ti + 1);

        let ps = wall.polyomino_placement_order();
        match ps {
            None => println!("-1"),
            Some(ps) => println!("{}", ps),
        }
    }
}
