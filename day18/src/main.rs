use std::collections::HashMap;

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let mut f = parse(&s);
    println!("{:?}", run(f.clone(), 10));
    let mut map = HashMap::new();
    let target = 1000000000;
    for i in 0.. {
        if let Some(prev) = map.get(&f) {
            let period = i - prev;
            for _ in 0..((target - i) % period) {
                f = step(f);
            }
            println!("{}", score(&f));
            break;
        } else {
            map.insert(f.clone(), i);
        }
        f = step(f);
    }
}

fn parse(s: &str) -> Vec<Vec<u8>> {
    s.split("\n")
        .filter(|l| l.len() > 0)
        .map(|s| s.as_bytes().to_vec())
        .collect()
}

fn step(old: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new = vec![vec!(b'.'; old[0].len()); old.len()];
    for x in 0..old[0].len() {
        for y in 0..old.len() {
            let mut yard = 0;
            let mut trees = 0;
            for (dx, dy) in &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let (x1, y1) = (x as isize + dx, y as isize + dy);
                if x1 >= 0 && x1 < old[0].len() as isize && y1 >= 0 && y1 < old.len() as isize {
                    let c = old[y1 as usize][x1 as usize];
                    trees += (c == b'|') as usize;
                    yard += (c == b'#') as usize;
                }
            }
            new[y][x] = match old[y][x] {
                b'.' => {
                    if trees >= 3 {
                        b'|'
                    } else {
                        b'.'
                    }
                }
                b'|' => {
                    if yard >= 3 {
                        b'#'
                    } else {
                        b'|'
                    }
                }
                b'#' => {
                    if yard >= 1 && trees >= 1 {
                        b'#'
                    } else {
                        b'.'
                    }
                }
                _ => panic!(),
            }
        }
    }
    new
}
fn score(f: &Vec<Vec<u8>>) -> usize {
    let yards = f
        .iter()
        .map(|l| l.iter().filter(|c| **c == b'#').count())
        .sum::<usize>();
    let trees = f
        .iter()
        .map(|l| l.iter().filter(|c| **c == b'|').count())
        .sum::<usize>();
    yards * trees
}

fn run(mut f: Vec<Vec<u8>>, steps: usize) -> usize {
    for _i in 0..steps {
        f = step(f);
    }
    score(&f)
}

#[test]
fn test() {
    let input = r#"
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."#;
    let f = parse(input);
    let next = step(f);
    let expected = parse(
        r#"
.......##.
......|###
.|..|...#.
..|#||...#
..##||.|#|
...#||||..
||...|||..
|||||.||.|
||||||||||
....||..|.
"#,
    );
    assert_eq!(next, expected);
    assert_eq!(run(parse(input), 10), 1147);
}

