use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("{:?}", run(&s, 5, 60));
}

fn run(s: &str, workers: usize, period: usize) -> (String, usize) {
    let mut steps = HashSet::new();
    let mut depends = HashMap::<char, Vec<char>>::new();
    s.trim().split("\n").filter(|l| l.len() > 0).for_each(|l| {
        let mut s = l.split(" ");
        let before = s.nth(1).unwrap().chars().next().unwrap();
        let after = s.nth(5).unwrap().chars().next().unwrap();
        steps.insert(before);
        steps.insert(after);
        depends.entry(before).or_default();
        depends.entry(after).or_default().push(before)
    });
    let mut steps = steps.into_iter().collect::<Vec<char>>();
    steps.sort();
    let mut sorted = String::new();
    while sorted.len() < steps.len() {
        for &s in &steps {
            if sorted.chars().any(|c| c == s) {
                continue;
            }
            if depends[&s].iter().all(|d| sorted.chars().any(|c| c == *d)) {
                sorted.push(s);
                break;
            }
        }
    }
    let mut started:Vec<(usize, char)> = Vec::new();
    let mut done = HashSet::new();
    let mut time = 0;
    loop {
        for s in &started {
            if s.0 == time {
                done.insert(s.1);
            }
        }
        if done.len() == steps.len() {
            break;
        }
        started.retain(|job| job.0 > time);
        while started.len() < workers {
            if let Some(start) = steps.iter().find(|&t| {
                !started.iter().any(|s| s.1 == *t) && !done.contains(t) && depends[t].iter().all(|d| done.contains(d))
            }) {
                started.push((time + period + (*start as u8 - b'A' + 1) as usize, *start));
            } else {
                break;
            }
        }
        println!("time: {} started: {:?} done: {:?}", time, started, done);
        time += 1;
    }
    (sorted, time)
}

#[test]
fn test() {
    assert_eq!(
        run(
            r#"
    Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#,
            2,
            0
        ),
        ("CABDFE".to_string(), 15)
    )
}
