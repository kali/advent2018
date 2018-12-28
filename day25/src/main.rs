fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("{:?}", run(&s));
}

type Pt = (isize, isize, isize, isize);

fn d(a: &Pt, b: &Pt) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()) as usize
}

fn run(s: &str) -> usize {
    let points: Vec<_> = s
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|line| {
            let mut tokens = line
                .trim()
                .split(",")
                .map(|s| s.parse::<isize>().unwrap());
            (
                tokens.next().unwrap(),
                tokens.next().unwrap(),
                tokens.next().unwrap(),
                tokens.next().unwrap(),
            )
        })
        .collect();
    let mut neighbours = vec![vec![]];
    for p1 in &points {
        let ns = points
            .iter()
            .enumerate()
            .filter(|(_, p2)| d(p1, p2) <= 3)
            .map(|(ix, _)| ix)
            .collect();
        neighbours.push(ns);
    }
    pathfinding::undirected::connected_components::components(&neighbours).len()
}

#[test]
fn t1() {
    let input = r#"
 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0"#;
    assert_eq!(run(input), 2);
}
