use std::cmp;
use std::io;
use std::ops::{Index, IndexMut};
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

struct PartialSums {
    data: Vec<i32>,
    offset: usize,
}

impl PartialSums {
    fn get_min_max_partial_sum(xs: &Vec<i32>) -> (i32, i32) {
        let mut min = xs[0];
        let mut max = xs[0];
        let mut partial_sum = 0;
        for x in xs {
            partial_sum += x;
            min = cmp::min(partial_sum, min);
            max = cmp::max(partial_sum, max);
        }

        (min, max)
    }

    pub fn new(xs: &Vec<i32>) -> Self {
        let (min, max) = Self::get_min_max_partial_sum(&xs);
        let offset = -cmp::min(min, 0) as usize;

        let initial_size = ((max + (offset as i32)) + 1) as usize;

        Self {
            data: vec![0; initial_size],
            offset,
        }
    }

    pub fn real_index(&self, index: i32) -> usize {
        (index + (self.offset as i32)) as usize
    }
}

impl Index<i32> for PartialSums {
    type Output = i32;
    fn index(&self, index: i32) -> &Self::Output {
        &self.data[self.real_index(index)]
    }
}

impl IndexMut<i32> for PartialSums {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        let real_index = self.real_index(index);
        if real_index >= self.data.len() {
            &mut self.data.resize(real_index + 1, 0);
        }
        &mut self.data[real_index]
    }
}

fn generate_squares(max_square: i32) -> Vec<i32> {
    // Based on `generate_squares_bench`, we can determine the capacity
    let capacity = (max_square as f32).sqrt().ceil() as usize + 1;
    let mut result: Vec<i32> = Vec::with_capacity(capacity);
    println!("capacity: {}", capacity);

    for i in 0..(capacity as i32) {
        let square = i * i;
        result.push(square);
    }

    result
}

fn count_squares(partial_sum: i32, squares: &Vec<i32>, partial_sums: &PartialSums) -> i32 {
    let mut result = 0;
    for square in squares {
        let index = partial_sum - square;

        if index == 0 {
            result += 1;
        }

        if index + (partial_sums.offset as i32) < 0 {
            break;
        }

        let n_occurrences = partial_sums[index];
        result += n_occurrences;
    }

    result
}

fn count_squares_in_subarrays(xs: &Vec<i32>, squares: &Vec<i32>) -> i32 {
    let mut partial_sums = PartialSums::new(&xs);

    let mut result = 0;

    let mut partial_sum = 0;
    for x in xs {
        // Count number of subarrays that end at `i` and have a sum that is
        // a perfect square

        partial_sum += x;

        result += count_squares(partial_sum, &squares, &partial_sums);

        partial_sums[partial_sum] += 1;
    }

    result
}

fn main() {
    let squares = generate_squares(1_000_000 * 100);

    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    let t: i32 = scanner.token();

    for ti in 0..t {
        let n: i32 = scanner.token();

        let mut xs = Vec::with_capacity(n as usize);

        for _ in 0..n {
            let x: i32 = scanner.token();
            xs.push(x);
        }

        let result = count_squares_in_subarrays(&xs, &squares);
        println!("Case #{}: {}", ti + 1, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn get_min_max_partial_sum_1() {
        let xs = vec![1, 2, 3, 4, 5];
        assert_eq!(
            PartialSums::get_min_max_partial_sum(&xs),
            (1, 1 + 2 + 3 + 4 + 5)
        );
    }

    #[test]
    fn get_min_max_partial_sum_2() {
        let xs = vec![1, 2, 3, -4, -5, -6];
        assert_eq!(PartialSums::get_min_max_partial_sum(&xs), (-9, 1 + 2 + 3));
    }

    #[test]
    fn get_min_max_partial_sum_3() {
        let xs = vec![-1, -2, -3];
        assert_eq!(PartialSums::get_min_max_partial_sum(&xs), (-6, -1));
    }

    #[test]
    #[ignore]
    fn generate_squares_bench() {
        let instant = Instant::now();
        let squares = generate_squares(1_000_000 * 100);
        println!(
            "Time elapsed: {}",
            instant.elapsed().as_micros() as f64 / 1e6
        );
        println!("squares.len(): {}", squares.len());
        assert_eq!(squares[0], 0);
        assert_eq!(squares[1], 1);
        assert_eq!(squares[2], 4);
        assert_eq!(squares[3], 9);
        assert_eq!(squares[squares.len() - 1], 10_000 * 10_000);
        assert_eq!(squares[squares.len() - 2], 9999 * 9999);
        assert_eq!(squares[squares.len() - 3], 9998 * 9998);
    }

    #[test]
    fn main_single() {
        let squares = generate_squares(100);
        let xs = vec![-9];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 0);
        let xs = vec![-4];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 0);
        let xs = vec![-3];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 0);
        let xs = vec![-2];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 0);
        let xs = vec![-1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 0);
        let xs = vec![0];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);
        let xs = vec![1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);
        let xs = vec![2];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 0);
        let xs = vec![3];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 0);
        let xs = vec![4];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);
        let xs = vec![5];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 0);
        let xs = vec![9];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);
    }

    #[test]
    fn main_positive_easy() {
        let squares = generate_squares(100);
        let xs = vec![2, 2, 6];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);
        let xs = vec![30, 30, 9, 1, 30];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 3);
        let xs = vec![4, 0, 0, 16];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 9);
        let xs = vec![2, 3, 1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 2);
        let xs = vec![4, 4];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 2);
    }

    #[test]
    fn main_repeated_zeroes() {
        let squares = generate_squares(100);
        let xs = vec![0, 0];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 3);
        let xs = vec![0, 0, 0];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 6);
        let xs = vec![0, 0, 0, 0];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 10);
        let xs = vec![0, 0, 0, 0, 0];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 15);
    }

    #[test]
    fn main_repeated_ones() {
        let squares = generate_squares(100);
        let xs = vec![1, 1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 2);
        let xs = vec![1, 1, 1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 3);
        let xs = vec![1, 1, 1, 1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 5);
        let xs = vec![1, 1, 1, 1, 1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 7);
        let xs = vec![1, 1, 1, 1, 1, 1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 6 + 3);
    }

    #[test]
    fn main_small_sum_zero() {
        let squares = generate_squares(100);

        let xs = vec![1, -1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 2);
        let xs = vec![2, -2];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);
        let xs = vec![3, -3];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);

        let xs = vec![-1, 1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 2);
        let xs = vec![-2, 2];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);
        let xs = vec![-3, 3];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);

        let xs = vec![1, 0, -1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1 + 2 + 1);
        let xs = vec![2, 0, -2];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1 + 1);
        let xs = vec![3, 0, -3];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1 + 1);

        let xs = vec![-1, 0, 1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1 + 2 + 1);
        let xs = vec![-2, 0, 2];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1 + 1);
        let xs = vec![-3, 0, 3];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1 + 1);

        let xs = vec![-2, 1, 1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 3);
        let xs = vec![-2, 0, 1, 1];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1 + 2 + 1 + 1);
        let xs = vec![5, -2, -3];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);
    }

    #[test]
    fn main_small_negatives() {
        let squares = generate_squares(100);
        let xs = vec![-1, -2];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 0);
        let xs = vec![-1, -2, -3];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 0);
        let xs = vec![-1, -2, -3, -4];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 0);
        let xs = vec![-1, -2, -3, -4, -5];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 0);
    }

    #[test]
    fn main_mixed() {
        let squares = generate_squares(100);
        let xs = vec![-2, 0];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);
        let xs = vec![-2, 0, 0];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 3);
        let xs = vec![-2, 0, 0, 2];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 4);
        let xs = vec![-2, 1, 1, 4];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 3 + 2);
        let xs = vec![-2, 0, 1, 1, 4];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1 + 2 + 1 + 1 + 2);

        let xs = vec![2, 0];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);
        let xs = vec![0, 2];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 1);
        let xs = vec![0, 0, 2];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 3);
        let xs = vec![4, 0];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 3);
        let xs = vec![0, 4];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 3);
        let xs = vec![0, 0, 4];
        assert_eq!(count_squares_in_subarrays(&xs, &squares), 3 + 2 + 1);
    }
}
