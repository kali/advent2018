fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("{:?}", run(&s.trim()));
}

fn run(s: &str) -> (usize, usize) {
    (
        reduce(s),
        (b'a'..=b'z')
            .map(|unit| {
                let cleaned:String = s
                    .chars()
                    .filter(|c| c.to_lowercase().next().unwrap() != unit as _)
                    .collect();
                reduce(&cleaned)
            })
            .min()
            .unwrap(),
    )
}

fn reduce(s: &str) -> usize {
    let mut s = s.to_string();
    let mut skip_one_more = false;
    loop {
        let mut next = String::new();
        for (a, b) in s.chars().zip(s.chars().skip(1)) {
            if skip_one_more {
                skip_one_more = false;
                continue;
            }
            if !(a != b && a.to_lowercase().next().unwrap() == b.to_lowercase().next().unwrap()) {
                next.push(a);
            } else {
                skip_one_more = true;
            }
        }
        if !skip_one_more {
            next.push(s.chars().last().unwrap());
        }
        if s == next {
            break;
        }
        s = next;
    }
    s.len()
}

#[test]
fn test() {
    assert_eq!(run("dabAcCaCBAcCcaDA"), (10, 4));
}
