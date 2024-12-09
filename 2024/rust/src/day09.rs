use utils::*;

fn score(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, n)| n.map_or(0, |n| i * n))
        .sum()
}

pub fn main() {
    let nums: Vec<u32> = stdin_all().chars().filter_map(|c| c.to_digit(10)).collect();

    let mut initial_disk = vec![];
    let mut info = HashMap::new();
    nums.chunks(2).for_each(|chunk| {
        let size = chunk[0];
        let id = info.len();
        info.insert(id, (initial_disk.len(), size as usize));
        for _ in 0..size {
            initial_disk.push(Some(id));
        }

        if chunk.len() > 1 {
            for _ in 0..chunk[1] {
                initial_disk.push(None);
            }
        }
    });

    let mut disk = initial_disk.clone();
    let mut last_insert = 0;
    for i in (0..disk.len()).rev() {
        if disk[i].is_none() {
            continue;
        }

        for j in last_insert..i {
            if disk[j].is_none() {
                disk.swap(i, j);
                last_insert = j + 1;
                break;
            }
        }
    }
    println!("{}", score(&disk));

    let mut disk = initial_disk;
    let mut slots: Vec<Option<usize>> = Vec::with_capacity(disk.len());
    for id in (0..info.len()).rev() {
        let (i, size) = info[&id];
        for j in 0..i {
            slots.truncate(0);
            slots.extend(disk.iter().cloned().skip(j).take(size));

            if slots.len() != size || !slots.iter().all(|item| item.is_none()) {
                continue;
            }

            disk[j..j + size].fill(Some(id));
            disk[i..i + size].fill(None);
            break;
        }
    }
    println!("{}", score(&disk));
}
