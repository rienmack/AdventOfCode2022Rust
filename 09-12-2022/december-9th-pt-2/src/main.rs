use std::collections::HashSet;

fn main(){
    println!("{:?}", part_two(include_str!("../data.txt")))
}


pub fn part_two(input: &str) -> usize {
    let moves = parse_moves(input);

    let visited = do_moves(moves, 10);

    visited.len()
}

fn do_moves(moves: Vec<(i32, i32)>, len: usize) -> HashSet<(i32, i32)> {
    let mut snake = vec![(0,0); len];

    let mut visited = HashSet::from([(0,0)]); 

    for (mx, my) in moves {
        snake[0].0 += mx;
        snake[0].1 += my;

        for i in 1..len {
            let dx = snake[i-1].0 - snake[i].0;
            let dy = snake[i-1].1 - snake[i].1;
            if dx.abs() < 2 && dy.abs() < 2 { continue;}

            if dx != 0 { snake[i].0 += dx/dx.abs(); }
            if dy != 0 { snake[i].1 += dy/dy.abs(); }
        }

        visited.insert(snake[len-1]);
    }

    visited
}

fn parse_moves(input: &str) -> Vec<(i32, i32)>{
    let mut moves = Vec::new();

    for line in input.trim().lines() {
        for (dir, count) in line.trim().split_once(" "){
            for _ in 0..count.parse().expect("invalid count"){
                match dir {
                    "R" => moves.push((1, 0)),
                    "L" => moves.push((-1, 0)),
                    "U" => moves.push((0, 1)),
                    "D" => moves.push((0, -1)),
                    _ => ()
                }
            }
        }
    }
    moves
}
