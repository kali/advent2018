use itertools::Itertools;

use std::ops::Range;

struct Piece {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

fn range_inter(a: Range<usize>, b: Range<usize>) -> bool {
    !(a.end < b.start || a.start > b.end)
}

impl Piece {
    pub fn inter(&self, other: &Piece) -> bool {
        range_inter(
            self.left..self.left + self.width,
            other.left..other.left + other.width,
        ) && range_inter(
            self.top..self.top + self.height,
            other.top..other.top + other.width,
        )
    }
}

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let mut pieces = vec![];
    for s in s.split("\n") {
        if let [id, _arobase, corner, dims] = &*s.split(" ").collect::<Vec<_>>() {
            let id = id[1..].parse().unwrap();
            let corner = &corner[..corner.len() - 1];
            if let ([left, top], [width, height]) = (
                &*corner
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
                &*dims
                    .split("x")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            ) {
                pieces.push(Piece {
                    id,
                    left: *left,
                    top: *top,
                    width: *width,
                    height: *height,
                })
            }
        }
    }
    println!("pieces: {:?}", pieces.len());
    let width = pieces.iter().map(|p| p.left + p.width).max().unwrap();
    let height = pieces.iter().map(|p| p.top + p.height).max().unwrap();
    let mut claims = vec![vec!(0usize; width); height];
    for p in &pieces {
        for x in p.left..p.left + p.width {
            for y in p.top..p.top + p.height {
                claims[y][x] += 1;
            }
        }
    }
    println!(
        "conflicts: {}",
        claims.iter().map(|v| v.iter().filter(|&c| *c > 1).count()).sum::<usize>()
    );

    for p in &pieces {
        let mut ok = true;
        for x in p.left..p.left + p.width {
            for y in p.top..p.top + p.height {
                if claims[y][x] > 1 {
                    ok = false;
                }
            }
        }
        if ok {
            println!("p: {}", p.id);
        }
    }
}
