struct State {
    list: Vec<u8>,
    e1: usize,
    e2: usize,
}

impl State {
    pub fn new() -> State {
        State {
            list: vec![3, 7],
            e1: 0,
            e2: 1,
        }
    }
    pub fn step(&mut self) {
        let r1 = self.list[self.e1];
        let r2 = self.list[self.e2];
        if (r1 + r2) / 10 != 0 {
            self.list.push((r1 + r2) / 10);
        }
        self.list.push((r1 + r2) % 10);
        self.e1 = (self.e1 + 1 + r1 as usize) % self.list.len();
        self.e2 = (self.e2 + 1 + r2 as usize) % self.list.len();
    }
}

fn run_1(step: usize) -> String {
    let mut s = State::new();
    (0..(step + 10)).for_each(|_| {
        s.step();
    });
    s.list[step..][..10]
        .iter()
        .map(|b| (b'0' + b) as char)
        .collect()
}

fn run_2(pat: &[u8]) -> usize {
    let mut s = State::new();
    let mut i = 0;
    loop {
        s.step();
        let check = if s.list.len() > 10 {
            &s.list[(s.list.len() - 10)..]
        } else {
            &*s.list
        };
        if check.windows(pat.len()).any(|s| s == pat) {
            break;
        }
        i += 1;
    }
    s.list.windows(pat.len()).position(|s| s == pat).unwrap()
}

fn main() {
    let input = 330121;
    println!("{}", run_1(input));
    println!("{}", run_2(&vec!(3, 3, 0, 1, 2, 1)));
}

#[test]
fn t_1() {
    assert_eq!(run_1(5), "0124515891");
}
#[test]
fn t_2() {
    assert_eq!(run_2(&vec!(0, 1, 2, 4, 5)), 5);
    assert_eq!(run_2(&vec!(5, 1, 5, 8, 9)), 9);
    assert_eq!(run_2(&vec!(9, 2, 5, 1, 0)), 18);
    assert_eq!(run_2(&vec!(5, 9, 4, 1, 4)), 2018);
}
