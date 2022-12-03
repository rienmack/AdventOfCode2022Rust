

fn main() {
    println!("{}", sum());
}

fn sum() -> u32
{
    include_str!("../data.txt")
    .lines()
    .map(|line| {
        let (x , y) = line.split_at(line.len() / 2);
        x.chars()
        .find(|item| y.contains(*item))
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
