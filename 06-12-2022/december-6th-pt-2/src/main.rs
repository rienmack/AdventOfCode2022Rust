use std::collections::HashSet;

fn main() {
    let input = include_str!("../data.txt");

    println!("{:?}", part_two(input))
}

fn part_two(input: &str) -> usize {

    let list = input.chars();

    list
    .collect::<Vec<char>>()
    .windows(14)
    .map(|ch| -> HashSet<char> { HashSet::from_iter(ch.to_vec())})
    .position(|ch| ch.len() == 14)
    .unwrap()
    +14
}