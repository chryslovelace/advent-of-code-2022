use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("*  {}", part1(input));
    println!("** {}", part2(input));
}

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        (27 + c as u8 - b'A') as u32
    } else {
        (1 + c as u8 - b'a') as u32
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            priority(left.chars().find(|&c| right.contains(c)).unwrap())
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .tuples()
        .map(|(x, y, z)| priority(x.chars().find(|&c| y.contains(c) && z.contains(c)).unwrap()))
        .sum()
}
