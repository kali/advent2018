fn main() {
    println!("{}", run(478, 71240));
    println!("{}", run(478, 7124000));
}

fn run(players: usize, points: usize) -> usize {
    let mut marbles = vec!((0, 0)); // cw, ccw
    let mut current = 0;
    let mut scores = vec!(0; players);
    for i in 1..=points {
        if i%23 != 0 {
            current = marbles[current].0;
            let next = marbles[current].0;
            marbles.push((next, current));
            marbles[next].1 = i;
            marbles[current].0 = i;
            current = i;
        } else {
            for _ in 0..7 {
                current = marbles[current].1;
            }
            let cw = marbles[current].0;
            let ccw = marbles[current].1;
            marbles[cw].1 = ccw;
            marbles[ccw].0 = cw;
            scores[(i%players)] += i + current;
            current = cw;
            marbles.push((0,0));
        }
        /*
        let mut j = 0;
        print!("{:2} ", i);
        loop {
            if j == current {
                print!("({:2}) ", j);
            } else {
                print!(" {:2}  ", j);
            }
            j = marbles[j].0;
            if j == 0 {
                break
            }
        }
        println!("");
        */
    }
    scores.into_iter().max().unwrap()
}

#[test]
fn test() {
    assert_eq!(run(9, 25), 32);
}
