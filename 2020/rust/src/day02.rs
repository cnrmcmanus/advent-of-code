use regex::Regex;
use utils::*;

struct Policy {
    c: char,
    x: usize,
    y: usize,
    passcode: String,
}

impl Policy {
    fn new(re: &Regex, text: &str) -> Option<Policy> {
        let m = re.captures(text)?;
        Some(Policy {
            x: m.get(1)?.as_str().parse().ok()?,
            y: m.get(2)?.as_str().parse().ok()?,
            c: m.get(3)?.as_str().chars().nth(0)?,
            passcode: m.get(4)?.as_str().to_string(),
        })
    }

    fn match_range(&self) -> bool {
        let n = self.passcode.chars().filter(|&c| c == self.c).count();
        n >= self.x && n <= self.y
    }

    fn match_indexes(&self) -> bool {
        let a = self.passcode.chars().nth(self.x - 1);
        let b = self.passcode.chars().nth(self.y - 1);
        (a == Some(self.c)) ^ (b == Some(self.c))
    }
}

pub fn main() {
    let re = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
    let policies: Vec<Policy> = stdin_lines()
        .filter_map(|line| Policy::new(&re, &line))
        .collect();
    println!("{}", policies.iter().filter(|p| p.match_range()).count());
    println!("{}", policies.iter().filter(|p| p.match_indexes()).count());
}
