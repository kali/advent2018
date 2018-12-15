use std::collections::HashSet;

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("{:?}", run(&s));
}

struct Plantation {
    state: String,
    offset: isize
}

impl Plantation {
    fn step(&self, rules: &HashSet<&str>) -> Plantation {
        let mut offset = self.offset;
        let mut next_state = "".to_string();
        for i in 0..(self.state.len() - 4) {
            let site = &self.state[i..][..5];
            next_state.push(if rules.contains(site) {
                '#'
            } else {
                '.'
            });
        }
        let first = next_state.bytes().position(|c| c == b'#').unwrap();
        let last = next_state.bytes().rposition(|c| c == b'#').unwrap();
        let mut state:String = "....".into();
        offset += 2isize - first as isize;
        next_state.chars().skip(first).take(last-first+1).for_each(|c| state.push(c));
        state.push_str("....");
        Plantation { state, offset }
    }

    fn sum(&self) -> isize {
        self.state
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|&(ix, c)| *c == b'#')
            .map(|(ix, _)| ix as isize - self.offset)
            .sum::<isize>()
    }
}

fn run(s: &str) -> (isize, isize) {
    let mut lines = s.split("\n");
    let mut state: String = lines
        .next()
        .unwrap()
        .split(" ")
        .nth(2)
        .unwrap()
        .to_string();
    state.insert(0, '.');
    state.insert(0, '.');
    state.insert(0, '.');
    state.insert(0, '.');
    state.push('.');
    state.push('.');
    state.push('.');
    state.push('.');
    let rules: HashSet<&str> = lines
        .filter(|l| l.ends_with(" => #"))
        .map(|s| s.split(" ").next().unwrap())
        .collect();
    let offset = 4;
    let mut p = Plantation { state: state.clone(), offset };
    (0..20).for_each(|_| p = p.step(&rules));
    let p1 = p.sum();

    let mut current = Plantation { state, offset };
    let mut i = 0;
    let p2 = loop {
        println!("{:3} ({:3}) {} -> {}", i, current.offset, current.state, current.sum());
        let next = current.step(&rules);
        if next.state == current.state {
            let diff = next.sum() - current.sum();
            println!("i: {} diff: {}", i, diff);
            break (50_000_000_000 - i) * diff + current.sum();
        }
        i += 1;
        current = next;
    };
    (p1, p2)
}

#[test]
fn test() {
    assert_eq!(
        run(r#"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
"#).0,
        325
    );
}
