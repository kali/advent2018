#[derive(Debug, PartialEq)]
struct Dot {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

impl Dot {
    pub fn parse(line: &str) -> Dot {
        let x = line[10..16].trim().parse().unwrap();
        let y = line[18..24].trim().parse().unwrap();
        let dx = line[36..38].trim().parse().unwrap();
        let dy = line[40..42].trim().parse().unwrap();
        Dot { x, y, dx, dy }
    }
}

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let dots: Vec<Dot> = s.trim().split("\n").map(Dot::parse).collect();
    let guess = (dots
        .iter()
        .map(|d| (d.y as f32 / d.dy as f32).abs())
        .sum::<f32>()
        / dots.len() as f32)
        .round() as isize;
    for guess in (guess-2..guess+2) {
        let pts: Vec<(isize, isize)> = dots
            .iter()
            .map(|d| (d.x + d.dx * guess, d.y + d.dy * guess))
            .collect();
        let min_x = pts.iter().map(|pair| pair.0).min().unwrap();
        let max_x = pts.iter().map(|pair| pair.0).max().unwrap();
        let min_y = pts.iter().map(|pair| pair.1).min().unwrap();
        let max_y = pts.iter().map(|pair| pair.1).max().unwrap();
        println!("x: {} - {} y: {} - {} guess: {}", min_x, max_x, min_y, max_y, guess);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!("{}", if pts.contains(&(x, y)) { "#" } else { " " });
            }
            println!("");
        }
    }
}

#[test]
fn test_parse() {
    let s = "position=<-50948,  20587> velocity=< 5, -2>";
    assert_eq!(
        Dot::parse(s),
        Dot {
            x: -50948,
            y: 20587,
            dx: 5,
            dy: -2
        }
    );
}
