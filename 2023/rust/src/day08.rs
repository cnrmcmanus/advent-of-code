use crate::util::*;
use std::collections::HashMap;

struct Path {
    current: String,
    instructions: Vec<char>,
    index: usize,
    steps: u64,
}

impl Path {
    fn new(initial: String, instructions: Vec<char>) -> Path {
        Path {
            current: initial,
            instructions,
            index: 0,
            steps: 0,
        }
    }

    fn next(&mut self) -> char {
        self.index %= self.instructions.len();
        let next = self.instructions[self.index];
        self.index += 1;
        next
    }

    fn tick(&mut self, map: &HashMap<String, (String, String)>) {
        let (left, right) = map.get(&self.current).unwrap();
        self.current = if self.next() == 'L' {
            left.clone()
        } else {
            right.clone()
        };
        self.steps += 1;
    }

    fn done(&self) -> bool {
        self.current.ends_with('Z')
    }

    fn jump_to_next_z(&mut self, map: &HashMap<String, (String, String)>) {
        while !self.done() && self.steps < 100000 {
            self.tick(map);
        }
    }
}

pub fn main() {
    let mut lines = stdin_lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut map = HashMap::new();
    for line in lines {
        let label = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        map.insert(label.to_string(), (left.to_string(), right.to_string()));
    }

    let mut single_path = Path::new("AAA".to_string(), instructions.clone());
    single_path.jump_to_next_z(&map);
    println!("{}", single_path.steps);

    let nodes: Vec<_> = map.keys().cloned().collect();
    let mut paths: Vec<_> = map
        .keys()
        .filter(|&key| key.ends_with('A'))
        .map(|start| Path::new(start.clone(), instructions.clone()))
        .collect();

    let mut next_zs = HashMap::new();
    for node in nodes {
        let mut next_z = Vec::new();
        for (i, _) in instructions.iter().enumerate() {
            let mut path = Path::new(node.clone(), instructions.clone());
            path.index += i;
            path.tick(&map);
            path.jump_to_next_z(&map);
            next_z.push(path);
        }
        next_zs.insert(node, next_z);
    }

    paths.iter_mut().for_each(|path| path.jump_to_next_z(&map));
    loop {
        let path = paths.iter_mut().min_by_key(|path| path.steps).unwrap();
        let next_z = next_zs.get(&path.current).unwrap();
        let next = &next_z[path.index % next_z.len()];
        path.current = next.current.clone();
        path.index = next.index;
        path.steps += next.steps;
        let steps = path.steps;

        if paths.iter().all(|path| path.steps == steps) {
            println!("{}", steps);
            break;
        }
    }
}
