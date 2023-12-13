use std::io::BufRead;
use std::iter::Iterator;
use std::ops::Add;
use std::str::FromStr;

pub fn stdin_lines() -> impl Iterator<Item = String> {
    std::io::stdin().lock().lines().map_while(Result::ok)
}

pub fn line_chunks<I>(stream: I) -> Vec<Vec<String>>
where
    I: Iterator<Item = String>,
{
    let mut chunks = vec![];
    let mut buffer = vec![];
    for line in stream {
        if line.is_empty() {
            chunks.push(buffer);
            buffer = vec![];
        } else {
            buffer.push(line);
        }
    }
    chunks.push(buffer);
    chunks
}

pub fn str_to_vec<T: FromStr>(string: &str) -> Vec<T> {
    let c = if string.contains(' ') { ' ' } else { ',' };
    string.split(c).filter_map(|n| n.parse().ok()).collect()
}

pub trait TupleSum<A, B>: Iterator {
    fn sum_tuples(&mut self) -> (A, B);
}

impl<A, B, I> TupleSum<A, B> for I
where
    I: Iterator<Item = (A, B)>,
    A: Add<Output = A> + Default,
    B: Add<Output = B> + Default,
{
    fn sum_tuples(&mut self) -> (A, B) {
        self.fold(
            (Default::default(), Default::default()),
            |(sum_a, sum_b), (a, b)| (sum_a + a, sum_b + b),
        )
    }
}
