fn main() {
    part_2();
}
fn part_2() {
    let mut buf: Vec<Vec<u32>> = Vec::new();
    for l in include_str!("../data.txt").split("\n") {
        buf.push(
                l.chars()
                .map(|x| x.to_digit(10).unwrap() as u32)
                .collect::<Vec<_>>(),
        );
    }

    let grid = buf;
    let mut score: Vec<Vec<u32>> = Vec::new();

    for y in 0..grid.len() {
        let mut tmp: Vec<u32> = Vec::new();
        for x in 0..grid[0].len() {
            let left = grid[y][0..x].iter().copied().rev().collect::<Vec<_>>();
            let right = grid[y][x + 1..grid[0].len()].to_vec();

            let top = grid[0..y].iter().map(|r| r[x]).rev().collect::<Vec<_>>();
            let bottom = grid[y + 1..grid.len()]
            .iter()
            .map(|r| r[x])
            .collect::<Vec<_>>();

            tmp.push(
                    get_score(grid[y][x], &right)
                    * get_score(grid[y][x], &left)
                    * get_score(grid[y][x], &top)
                    * get_score(grid[y][x], &bottom),
            );
        }

        score.push(tmp);
    }

    println!(
            "{}",
    score.iter().map(|r| r.iter().max().unwrap()).max().unwrap()
    );
}

fn get_score(val: u32, direction: &Vec<u32>) -> u32 {
    let mut score = 0;
    for tree in direction {
        if val > *tree {
            score += 1;
        } else {
            return score + 1;
        }
    }
    return score;
}