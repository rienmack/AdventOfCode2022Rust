fn main() {
    let mut sum = 0;
    include_str!("../data.txt")
    .lines()
    .map(|assignment| {
        let split = assignment.split(',').collect::<Vec<&str>>();
        let a: Vec<i32> = split[0]
        .split('-')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
        let b: Vec<i32> = split[1]
        .split('-')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
        if a[0] <= b[1] && a[1] >= b[0] {
            sum += 1;
        }
        else if b[0] <= a[1] && b[1] >= a[0] {
            sum += 1;
        }
    }).count();
    println!("{}", sum);
}