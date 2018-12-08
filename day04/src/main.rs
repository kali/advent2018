use std::collections::HashMap;
use std::ops::Range;

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let answer = run(&s);
    println!("{:?}", answer);
}

fn run(s: &str) -> (usize, usize) {
    let mut lines = s.split("\n").filter(|l| l.len() > 0).collect::<Vec<_>>();
    lines.sort();
    let mut sleep_periods = HashMap::<usize, Vec<Range<usize>>>::new();
    let mut current = None;
    let mut asleep_since = 0;
    for l in &lines {
        if l.ends_with("shift") {
            let id = l.split(" ").nth(3).unwrap();
            current = Some(id[1..].parse::<usize>().unwrap());
        } else {
            let min = l.split(" ").nth(1).unwrap()[3..=4]
                .parse::<usize>()
                .unwrap();
            if l.ends_with("up") {
                sleep_periods
                    .entry(current.unwrap())
                    .or_default()
                    .push(asleep_since..min)
            } else {
                asleep_since = min;
            }
        }
    }
    let most_asleep:usize = *sleep_periods
        .iter()
        .max_by_key(|&(k, v)| v.iter().map(|r| r.end - r.start).sum::<usize>())
        .unwrap()
        .0;
    println!("most asleep: #{}", most_asleep);
    let mut minutes = vec![0; 60];
    for period in &sleep_periods[&most_asleep] {
        for i in period.start..period.end {
            minutes[i] += 1;
        }
    }
    println!("minutes: {:?}", minutes);
    let best_minute = minutes
        .iter()
        .enumerate()
        .max_by_key(|&(ix, m)| m)
        .unwrap()
        .0;
    println!("best_minute: {}", best_minute);
    let answer1 = best_minute * most_asleep;
    let best_opportunity = sleep_periods.iter().flat_map(|(&gard, periods)| {
        (0..60).map(move |min| (gard, min, periods.iter().filter(|r| r.start <= min && min < r.end ).count()))
    }).max_by_key(|p| p.2).unwrap();
    (answer1, best_opportunity.0*best_opportunity.1)
}

#[test]
fn test() {
    let r = run(r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"#);
    assert_eq!(r, (240, 4455));
}
