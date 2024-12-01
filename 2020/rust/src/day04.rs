use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;
use utils::*;

type Passport = HashMap<String, String>;
type Validation = fn(&str) -> bool;

static RE_PAIR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\w+):([a-z0-9#]+)").unwrap());
static RE_HEIGHT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)(cm|in)$").unwrap());
static RE_HAIR_COLOR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^#[0-9a-z]{6}$").unwrap());
static RE_EYE_COLOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap());
static RE_PID: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{9}$").unwrap());

static COMPLEX_VALIDATIONS: LazyLock<HashMap<&str, Validation>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("byr", (|value| between(value, 1920, 2002)) as Validation);
    m.insert("iyr", (|value| between(value, 2010, 2020)) as Validation);
    m.insert("eyr", (|value| between(value, 2020, 2030)) as Validation);
    m.insert("hgt", height);
    m.insert("hcl", (|value| RE_HAIR_COLOR.is_match(value)) as Validation);
    m.insert("ecl", (|value| RE_EYE_COLOR.is_match(value)) as Validation);
    m.insert("pid", (|value| RE_PID.is_match(value)) as Validation);
    m
});

fn passport_new(line: &str) -> Option<Passport> {
    let mut passport = HashMap::new();
    for m in RE_PAIR.captures_iter(line) {
        let field = m.get(1)?.as_str().to_string();
        let value = m.get(2)?.as_str().to_string();
        passport.insert(field, value);
    }
    Some(passport)
}

fn between(value: &str, min: u32, max: u32) -> bool {
    value
        .parse()
        .ok()
        .filter(|&n: &u32| n >= min && n <= max)
        .is_some()
}

fn height(value: &str) -> bool {
    fn parse(value: &str) -> Option<(&str, u32)> {
        let m = RE_HEIGHT.captures(value)?;
        let value = m.get(1)?.as_str().parse().ok()?;
        let unit = m.get(2)?.as_str();
        Some((unit, value))
    }
    match parse(value) {
        Some(("cm", value)) if (150..=193).contains(&value) => true,
        Some(("in", value)) if (59..=76).contains(&value) => true,
        _ => false,
    }
}

fn simple_validation(passport: &Passport) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .all(|field| passport.contains_key(field))
}

fn complex_validation(passport: &Passport) -> bool {
    COMPLEX_VALIDATIONS.iter().all(|(&field, &validator)| {
        let Some(value) = passport.get(field) else {
            return false;
        };
        validator(value)
    })
}

pub fn main() {
    let passports: Vec<_> = stdin_lines_by(r"(\r\n\r\n|\n\n)")
        .into_iter()
        .filter_map(|line| passport_new(&line))
        .collect();
    println!(
        "{}",
        passports.iter().filter(|p| simple_validation(p)).count()
    );
    println!(
        "{}",
        passports.iter().filter(|p| complex_validation(p)).count()
    );
}
