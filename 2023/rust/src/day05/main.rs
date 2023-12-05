use std::io::BufRead;

type Pair = (i64, i64);

fn str_to_int_vec(string: &str) -> Vec<i64> {
    string.split(' ').filter_map(|n| n.parse().ok()).collect()
}

pub fn split(
    (victim_first, victim_last): Pair,
    (cutter_first, cutter_last): Pair,
) -> ((Pair, Pair), Pair) {
    if cutter_first >= victim_first && cutter_first <= victim_last {
        let keep_start = if cutter_first == victim_first {
            (0, 0)
        } else {
            (victim_first, cutter_first - 1)
        };
        let keep_end = if cutter_last >= victim_last {
            (0, 0)
        } else {
            (cutter_last + 1, victim_last)
        };
        let cut = (cutter_first, std::cmp::min(cutter_last, victim_last));

        ((keep_start, keep_end), cut)
    } else if cutter_last >= victim_first && cutter_last <= victim_last {
        let keep_end = if cutter_last == victim_last {
            (0, 0)
        } else {
            (cutter_last + 1, victim_last)
        };
        let cut = (victim_first, cutter_last);

        (((0, 0), keep_end), cut)
    } else if cutter_first < victim_first && cutter_last > victim_last {
        (((0, 0), (0, 0)), (victim_first, victim_last))
    } else {
        (((victim_first, victim_last), (0, 0)), (0, 0))
    }
}

fn parse_mappings(lines: impl Iterator<Item = String>) -> Vec<Vec<(i64, i64, i64)>> {
    let lines = lines.filter(|line| !line.is_empty()).skip(1);
    let mut mappings = Vec::new();
    let mut buffer = Vec::new();

    for line in lines {
        match str_to_int_vec(&line).as_slice() {
            &[dst, src, len] => buffer.push((dst, src, len)),
            _ => {
                mappings.push(buffer);
                buffer = Vec::new();
            }
        }
    }

    mappings.push(buffer);
    mappings
}

fn crunch(mappings: &[Vec<(i64, i64, i64)>], ranges: Vec<Pair>) -> Vec<Pair> {
    mappings.iter().fold(ranges, |mut state, mapping| {
        let mut next = vec![];
        for &(dst, src, len) in mapping {
            let src_end = src + len - 1;
            let diff = dst - src;
            state = state
                .into_iter()
                .flat_map(|pair| {
                    let ((keep_start, keep_end), cut) = split(pair, (src, src_end));
                    let keep = vec![keep_start, keep_end]
                        .into_iter()
                        .filter(|&range| range != (0, 0))
                        .collect::<Vec<_>>();
                    if cut != (0, 0) {
                        next.push((cut.0 + diff, cut.1 + diff));
                    }
                    keep.into_iter()
                })
                .collect::<Vec<_>>();
        }
        next.append(&mut state);
        next
    })
}

pub fn main() {
    let mut lines = std::io::stdin().lock().lines().map_while(Result::ok);
    let initial = str_to_int_vec(&lines.next().unwrap()[7..]);
    let mappings = parse_mappings(lines);

    let mut parts = initial.clone().into_iter();
    let ranges = std::iter::from_fn(move || match (parts.next(), parts.next()) {
        (Some(a), Some(b)) => Some((a, a + b - 1)),
        _ => None,
    })
    .collect::<Vec<_>>();

    let results_1 = crunch(&mappings, initial.into_iter().map(|x| (x, x)).collect());
    let results_2 = crunch(&mappings, ranges);

    println!("{}", results_1.iter().map(|(x, _)| x).min().unwrap());
    println!("{}", results_2.iter().map(|(x, _)| x).min().unwrap());
}
