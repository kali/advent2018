#[macro_use]
extern crate nom;

use std::fs;
use std::str::FromStr;
use std::collections::HashSet;

named!(nums <&str, Vec<isize>>, separated_list_complete!(tag!("\n"), alt!(pos|neg)));

named!(pos <&str, isize>,
    preceded!(tag!("+"), map_res!(nom::digit, std::str::FromStr::from_str))
);

named!(neg <&str, isize>,
    preceded!(tag!("-"), map_res!(nom::digit,
                                  |x| isize::from_str(x).map(|x| -x)))
);

fn main() {
    let f = fs::read_to_string("input").unwrap();
    let nums = nums(&f).unwrap().1;
    println!("1: {}", nums.iter().sum::<isize>());
    let mut seen = HashSet::new();
    let mut current = 0;
    loop {
        for x in &nums {
            current += x;
            if seen.contains(&current) {
                println!("2: {}", current);
                return;
            } else {
                seen.insert(current);
            }
        }
    }
}
