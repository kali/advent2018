use std::ops::Range;

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("{:?}", run(&s));
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Wall,
    Dry,
    Set,
    Flow,
    Off,
}

fn run(s: &str) -> (usize, usize) {
    let walls: Vec<(Range<usize>, Range<usize>)> = s
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| {
            let mut x = None;
            let mut y = None;
            for token in l.split(", ") {
                let mut values = (&token[2..])
                    .split("..")
                    .map(|s| s.parse::<usize>().unwrap());
                let start = values.next().unwrap();
                let end = values.next().unwrap_or(start);
                if token.starts_with("x=") {
                    x = Some(start..end);
                } else {
                    y = Some(start..end);
                }
            }
            (x.unwrap(), y.unwrap())
        })
        .collect();
    let x_min = walls.iter().map(|r| r.0.start).min().unwrap() - 1;
    let x_max = walls.iter().map(|r| r.0.end).max().unwrap() + 1;
    let y_min = walls.iter().map(|r| r.1.start).min().unwrap();
    let y_max = walls.iter().map(|r| r.1.end).max().unwrap();
    let mut map = vec![vec![Cell::Dry; x_max - x_min + 1]; y_max + 1];
    for w in walls {
        for y in w.1.start..=w.1.end {
            for x in w.0.start..=w.0.end {
                map[y][x - x_min] = Cell::Wall;
            }
        }
    }
    let mut tovisit = vec![];
    map[0][500 - x_min] = Cell::Flow;
    tovisit.push((500, 0));
    while let Some((x, y)) = tovisit.pop() {
        let c = map[y][x - x_min];
        if c == Cell::Flow && y < y_max {
            let below = map[y + 1][x - x_min];
            if below == Cell::Dry {
                map[y + 1][x - x_min] = Cell::Flow;
                tovisit.push((x, y + 1));
            }
            if below == Cell::Wall || below == Cell::Set {
                let left = if x == x_min {
                    Cell::Off
                } else {
                    map[y][x - x_min - 1]
                };
                if left == Cell::Dry {
                    map[y][x - x_min - 1] = Cell::Flow;
                    tovisit.push((x - 1, y));
                }
                let right = if x == x_max {
                    Cell::Off
                } else {
                    map[y][x - x_min + 1]
                };
                if right == Cell::Dry {
                    map[y][x - x_min + 1] = Cell::Flow;
                    tovisit.push((x + 1, y));
                }
                if left == Cell::Wall || right == Cell::Wall {
                    let mut f_min = x;
                    while map[y][f_min - x_min] == Cell::Flow {
                        f_min -= 1;
                    }
                    let mut f_max = x;
                    while f_max < x_max && map[y][f_max - x_min] == Cell::Flow {
                        f_max += 1;
                    }
                    if map[y][f_min - x_min] == Cell::Wall && map[y][f_max - x_min] == Cell::Wall {
                        for x in (f_min + 1)..f_max {
                            map[y][x - x_min] = Cell::Set;
                            tovisit.push((x, y - 1));
                        }
                    }
                }
            }
        }
    }
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let c = match map[y][x - x_min] {
                Cell::Wall => '\u{2588}',
                Cell::Dry => '.',
                Cell::Set => '#',
                Cell::Flow => '*',
                Cell::Off => 'X',
            };
            print!("{}", c);
        }
        println!("");
    }
    let wet = map
        .iter()
        .skip(y_min)
        .map(|l| {
            l.iter()
                .filter(|&c| *c == Cell::Set || *c == Cell::Flow)
                .count()
        })
        .sum::<usize>();
    let set = map
        .iter()
        .skip(y_min)
        .map(|l| l.iter().filter(|&c| *c == Cell::Set).count())
        .sum::<usize>();
    (wet, set)
}

#[test]
fn t0() {
    let s = r#"
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"#;
    assert_eq!(run(s), (57, 29));
}

#[test]
fn t1() {
    let s = r#"
x=498, y=3..5
y=5, x=498..502
x=502, y=3..5"#;
    assert_eq!(run(s), (3 * 2 + 6, 3*2));
}

#[test]
fn t2() {
    let s = r#"
x=498, y=2..5
y=5, x=498..502
x=502, y=3..5"#;
    assert_eq!(run(s), (3 * 2 + 4 + 4, 3*2));
}

#[test]
fn t3() {
    let s = r#"
x=498, y=3..5
y=5, x=498..502
x=502, y=2..5"#;
    assert_eq!(run(s), (3 * 2 + 4 + 4, 3*2));
}

#[test]
fn t4() {
    let s = r#"
x=495, y=1
x=505, y=7
x=498, y=3..5
y=5, x=498..502
x=502, y=3..5"#;
    assert_eq!(run(s), (3 * 2 + 5 + 1 + 12, 3*2));
}

#[test]
fn t5() {
    let s = r#"
x=495, y=1
x=505, y=7
x=498, y=2..5
y=5, x=498..502
x=502, y=3..5"#;
    assert_eq!(run(s), (3 * 2 + 4 + 1 + 6, 3*2));
}

#[test]
fn t6() {
    let s = r#"
x=495, y=1
x=505, y=7
x=498, y=3..5
y=5, x=498..502
x=502, y=2..5"#;
    assert_eq!(run(s), (3 * 2 + 4 + 1 + 6, 3*2));
}
