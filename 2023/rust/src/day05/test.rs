#![cfg(test)]

use super::main::split;

#[test]
fn cutter_start() {
    let splitted = split((1, 3), (1, 2));
    assert_eq!(splitted.0 .0, (0, 0));
    assert_eq!(splitted.0 .1, (3, 3));
    assert_eq!(splitted.1, (1, 2));
}

#[test]
fn cutter_end() {
    let splitted = split((4, 6), (1, 5));
    assert_eq!(splitted.0 .0, (0, 0));
    assert_eq!(splitted.0 .1, (6, 6));
    assert_eq!(splitted.1, (4, 5));
}

#[test]
fn cutter_middle() {
    let splitted = split((1, 10), (3, 5));
    assert_eq!(splitted.0 .0, (1, 2));
    assert_eq!(splitted.0 .1, (6, 10));
    assert_eq!(splitted.1, (3, 5));
}

#[test]
fn cutter_misses() {
    let splitted = split((4, 6), (1, 2));
    assert_eq!(splitted.0 .0, (4, 6));
    assert_eq!(splitted.0 .1, (0, 0));
    assert_eq!(splitted.1, (0, 0));
}

#[test]
fn cutter_swallows() {
    let splitted = split((4, 5), (1, 10));
    assert_eq!(splitted.0 .0, (0, 0));
    assert_eq!(splitted.0 .1, (0, 0));
    assert_eq!(splitted.1, (4, 5));
}
