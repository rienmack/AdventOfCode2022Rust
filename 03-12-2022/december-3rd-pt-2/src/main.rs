fn main() {
    println!("{}", sum());
}

fn sum() -> u32
{
    include_str!("../data.txt")
    .lines()
    .collect::<Vec<&str>>()
    .chunks(3)
    .map(|line|{
        line[0]
            .chars()
            .find(|c| line[1].contains(*c) && line[2].contains(*c))
            .map(get_value)
            .unwrap()
    })
    .sum()
}

fn get_value(item: char) -> u32 {
    let item = item as u32;
    item - match item {
        41..=90 => 38,
        _ => 96,
    }
}
