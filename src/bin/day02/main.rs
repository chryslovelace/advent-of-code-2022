fn main() {
    let input = include_str!("input.txt");
    println!("*  {}", part1(input));
    println!("** {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|strat| match strat {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => panic!(),
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|strat| match strat {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => panic!(),
        })
        .sum()
}
