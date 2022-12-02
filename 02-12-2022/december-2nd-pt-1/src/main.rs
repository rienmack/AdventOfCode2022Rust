enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}


fn main() {
    let mut score = 0;
   include_str!("../data.txt")
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
       .for_each(|pick| {
            let opp_pick = get_choice(&pick[0]);
            score += game(&opp_pick, &get_choice(&pick[1]));
       });
    println!("score: {}", score);
}

fn game(opponent: &Choice, me: &Choice) -> i32 {
    let item_score = *me as i32;
    let result = match (opponent, me) {
        (Choice::Rock, Choice::Paper) => 6,
        (Choice::Rock, Choice::Scissors) => 0,
        (Choice::Paper, Choice::Rock) => 0,
        (Choice::Paper, Choice::Scissors) => 6,
        (Choice::Scissors, Choice::Rock) => 6,
        (Choice::Scissors, Choice::Paper) => 0,
        _ => 3,
    };
    result + item_score
}

fn get_choice(strategy: &str) -> Choice {
    match strategy {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        _ => panic!("bad pick"),
    }
}