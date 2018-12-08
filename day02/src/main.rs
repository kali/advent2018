extern crate itertools;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let a1 = s.split("\n").fold((0, 0), |acc, it| {
        let mut hash: HashMap<char, usize> = HashMap::new();
        for c in it.chars() {
            *hash.entry(c).or_default() += 1;
        }
        let has_double = hash.values().any(|&l| l == 2);
        let has_triple = hash.values().any(|&l| l == 3);
        (acc.0 + has_double as usize, acc.1 + has_triple as usize)
    });
    println!("1: {}", a1.0 * a1.1);
    let pair = s
        .split("\n")
        .tuple_combinations()
        .find(|&(a, b)| a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() == 1)
        .unwrap();
    let common: String = pair
        .0
        .chars()
        .zip(pair.1.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect();
    println!("2: {}", common);
}
