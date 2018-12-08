fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("{:?}", run(s.trim(), 10000));
}

pub fn run(s: &str, limit: usize) -> (usize, usize) {
    let coords: Vec<(usize, usize)> = s
        .split("\n")
        .map(|l| {
            let mut s = l.split(",").map(|n| n.trim().parse::<usize>().unwrap());
            (s.next().unwrap(), s.next().unwrap())
        })
        .collect();
    println!("coords: {:?}", coords);
    let max_x = coords.iter().map(|p| p.0).max().unwrap();
    let max_y = coords.iter().map(|p| p.1).max().unwrap();
    let mut map = vec![vec!(None; max_x + 1); max_y + 1];
    for x in 0..=max_x {
        for y in 0..=max_y {
            let dists: Vec<usize> = coords
                .iter()
                .map(|(tx, ty)| {
                    (*tx as isize - x as isize).abs() as usize
                        + (*ty as isize - y as isize).abs() as usize
                })
                .collect();
            let min = dists.iter().min().unwrap();
            if dists.iter().filter(|&d| d == min).count() == 1 {
                let node = dists
                    .iter()
                    .enumerate()
                    .find(|&(_ix, d)| d == min)
                    .unwrap()
                    .0;
                map[y][x] = Some(node)
            }
        }
    }
    let finite: Vec<usize> = (0..coords.len())
        .filter(|&pt| {
            !(0..=max_x).any(|x| map[0][x] == Some(pt) || map[max_y][x] == Some(pt))
                && !(0..=max_y).any(|y| map[y][0] == Some(pt) || map[y][max_x] == Some(pt))
        })
        .collect();
    let best = finite
        .iter()
        .map(|i| {
            map.iter()
                .map(|l| l.iter().filter(|&x| *x == Some(*i)).count())
                .sum::<usize>()
        })
        .max().unwrap();
    let mut safe = 0;
    for x in 0..=max_x {
        for y in 0..=max_y {
            let dist = coords
                .iter()
                .map(|(tx, ty)| {
                    (*tx as isize - x as isize).abs() as usize
                        + (*ty as isize - y as isize).abs() as usize
                })
                .sum::<usize>();
            if dist < limit {
                safe += 1;
            }
        }
    }
    (best, safe)
}

#[test]
fn test() {
    assert_eq!(
        run(r#"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"#, 32),
        (17, 16)
    )
}
