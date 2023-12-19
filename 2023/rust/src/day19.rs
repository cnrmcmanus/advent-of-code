use crate::util::*;
use std::collections::HashMap;

#[derive(Clone)]
enum Exit {
    Result(bool),
    Goto(String),
}

#[derive(Clone)]
enum Rule {
    End(Exit),
    Comparison {
        category: char,
        op: char,
        number: u64,
        exit: Exit,
    },
}

type Rating = (u64, u64);

fn parse_workflow(line: &str) -> (String, Vec<Rule>) {
    let opening = line.find('{').unwrap();
    let label = line[..opening].to_string();

    let rules: Vec<_> = line[opening + 1..line.len() - 1]
        .split(',')
        .map(|rule| match rule.find(':') {
            None => {
                let exit = if rule == "A" || rule == "R" {
                    Exit::Result(rule == "A")
                } else {
                    Exit::Goto(rule.to_string())
                };

                Rule::End(exit)
            }
            Some(index) => {
                let chars = rule.chars().collect::<Vec<_>>();
                let category = chars[0];
                let op = chars[1];
                let number = rule[2..index].parse().unwrap();
                let exit = if &rule[index + 1..] == "A" || &rule[index + 1..] == "R" {
                    Exit::Result(&rule[index + 1..] == "A")
                } else {
                    Exit::Goto(rule[index + 1..].to_string())
                };

                Rule::Comparison {
                    category,
                    op,
                    number,
                    exit,
                }
            }
        })
        .collect();

    (label, rules)
}

fn parse_ratings(line: &str) -> Vec<u64> {
    line[1..line.len() - 1]
        .split(',')
        .map(|part| part[2..].parse().unwrap())
        .collect()
}

fn combinations(ratings: &[Rating]) -> u64 {
    ratings
        .iter()
        .map(|rating| rating.1 + 1 - rating.0)
        .product()
}

fn valid(ratings: &[Rating]) -> bool {
    ratings.iter().all(|rating| rating.0 <= rating.1)
}

fn accepted(ratings: &[u64], label: String, workflows: &HashMap<String, Vec<Rule>>) -> bool {
    let mut workflow = workflows.get(&label).unwrap();
    loop {
        for rule in workflow {
            match rule.clone() {
                Rule::End(Exit::Result(result)) => {
                    return result;
                }
                Rule::End(Exit::Goto(label)) => {
                    workflow = workflows.get(&label).unwrap();
                    break;
                }
                Rule::Comparison {
                    category,
                    op,
                    number,
                    exit,
                } => {
                    let i = ['x', 'm', 'a', 's']
                        .into_iter()
                        .position(|c| c == category)
                        .unwrap();
                    let rating = ratings[i];
                    if op == '<' && rating < number || op == '>' && rating > number {
                        match exit {
                            Exit::Result(result) => {
                                return result;
                            }
                            Exit::Goto(label) => {
                                workflow = workflows.get(&label).unwrap();
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn tally(label: String, mut ratings: Vec<Rating>, workflows: &HashMap<String, Vec<Rule>>) -> u64 {
    let workflow = workflows.get(&label).unwrap();
    let mut total: u64 = 0;

    for rule in workflow {
        match rule.clone() {
            Rule::End(Exit::Result(true)) => return total + combinations(&ratings),
            Rule::End(Exit::Result(false)) => return total,
            Rule::End(Exit::Goto(label)) => {
                return total + tally(label.clone(), ratings, workflows)
            }
            Rule::Comparison {
                category,
                op,
                number,
                exit,
            } => {
                let i = ['x', 'm', 'a', 's']
                    .into_iter()
                    .position(|c| c == category)
                    .unwrap();
                let mut split_ratings = ratings.clone();
                if number >= ratings[i].0 && number <= ratings[i].1 {
                    if op == '<' {
                        ratings[i].0 = number;
                        split_ratings[i].1 = number - 1;
                    } else {
                        ratings[i].1 = number;
                        split_ratings[i].0 = number + 1;
                    }

                    if valid(&split_ratings) {
                        match exit {
                            Exit::Result(true) => {
                                total += combinations(&split_ratings);
                            }
                            Exit::Result(false) => {}
                            Exit::Goto(label) => {
                                total += tally(label.clone(), split_ratings, workflows)
                            }
                        }
                    }

                    if !valid(&ratings) {
                        return total;
                    }
                }
            }
        }
    }

    0
}

pub fn main() {
    let chunks = line_chunks(stdin_lines());
    let [workflow_lines, ratings_lines] = chunks.as_slice() else {
        return;
    };

    let mut workflows = HashMap::new();
    for line in workflow_lines {
        let (label, rules) = parse_workflow(line);
        workflows.insert(label, rules);
    }

    let total_1 = ratings_lines.iter().fold(0, |total, line| {
        let ratings = parse_ratings(line);
        if accepted(&ratings, "in".to_string(), &workflows) {
            total + ratings.into_iter().sum::<u64>()
        } else {
            total
        }
    });
    println!("{}", total_1);

    let ratings = vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)];
    let total_2 = tally("in".to_string(), ratings, &workflows);
    println!("{}", total_2);
}
