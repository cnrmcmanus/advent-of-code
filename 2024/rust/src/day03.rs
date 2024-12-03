use utils::*;

fn run(contents: &str, dos: &[usize], donts: &[usize]) -> u32 {
    let re = regex(r"mul\((\d{1,3}),(\d{1,3})\)");
    re.captures_iter(contents).fold(0, |ans, mul| {
        let start = mul.get(0).unwrap().start();
        let prev_do = dos.iter().take_while(|&&d| d < start).last();
        let prev_dont = donts.iter().take_while(|&&d| d < start).last();
        let xy = mul[1].parse::<u32>().unwrap() * mul[2].parse::<u32>().unwrap();
        ans + xy * u32::from(prev_dont.is_none() || prev_do > prev_dont)
    })
}

pub fn main() {
    let contents = stdin_all();

    println!("{}", run(&contents, &[], &[]));

    let dos: Vec<_> = contents.match_indices("do()").map(|m| m.0).collect();
    let donts: Vec<_> = contents.match_indices("don't()").map(|m| m.0).collect();
    println!("{}", run(&contents, &dos, &donts));
}
