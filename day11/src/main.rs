fn main() {
    println!("{:?}", run(3214));
}

fn level(x: usize, y: usize, serial: isize) -> isize {
    if x == 0 || y == 0 {
        return 0;
    }
    let x = x as isize;
    let y = y as isize;
    (((10 + x) * y) + serial) * (10 + x) / 100 % 10 - 5
}

fn run(serial: isize) -> ((usize, usize), (usize, usize, usize)) {
    let ref integral = {
        let mut integral = 
            vec![vec![0isize; 301]; 301];
        for y in 1..=300 {
            let mut s = 0;
            for x in 1..=300 {
                s += level(x, y, serial);
                integral[y][x] = integral[y - 1][x] + s;
            }
        }
        integral
    };
    let (_l, x, y) = (1..=298)
        .flat_map(|x| {
            (1..=298).map(move |y| {
                let s = integral[y + 2][x + 2] + integral[y - 1][x - 1]
                    - integral[y + 2][x - 1]
                    - integral[y - 1][x + 2];
                (s, x, y)
            })
        })
        .max()
        .unwrap();
    let a1 = (x, y);
    let (_l, x, y, d) = (1..=300)
        .flat_map(move |d| {
            (1..=(300 - d)).flat_map(move |x| {
                (1..=(300 - d)).map(move |y| {
                    (
                        integral[y + d - 1][x + d - 1] + integral[y - 1][x - 1]
                            - integral[y - 1][x + d - 1]
                            - integral[y + d - 1][x - 1],
                        x,
                        y,
                        d,
                    )
                })
            })
        })
        .max()
        .unwrap();
    let a2 = (x, y, d);
    (a1, a2)
}

#[test]
fn test() {
    assert_eq!(level(3, 5, 8), 4);
    assert_eq!(run(18), ((33, 45), (90, 269, 16)));
}
