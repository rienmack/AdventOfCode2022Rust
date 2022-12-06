use std::collections::HashSet;

fn main() {
    let input = include_str!("../data.txt");

    println!("{:?}", part_one(input))
}

fn part_one(input: &str) -> usize {

    let list = input.chars();

    list
        .collect::<Vec<char>>()
        .windows(4)
        .map(|ch| -> HashSet<char> { HashSet::from_iter(ch.to_vec())})
        .position(|ch| ch.len() == 4)
        .unwrap()
        +4


}