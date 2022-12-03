use itertools::Itertools;

type Strats = Vec<(char, char)>;

fn main() {
    let strats: Strats = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (opp, _, me) = line.chars().next_tuple().unwrap();
            (opp, me)
        })
        .collect();
    println!("*  {}", part1(&strats));
    println!("** {}", part2(&strats));
}

fn part1(strats: &Strats) -> u32 {
    let mut score = 0;
    for strat in strats {
        score += match strat {
            ('A', 'X') => 4,
            ('A', 'Y') => 8,
            ('A', 'Z') => 3,
            ('B', 'X') => 1,
            ('B', 'Y') => 5,
            ('B', 'Z') => 9,
            ('C', 'X') => 7,
            ('C', 'Y') => 2,
            ('C', 'Z') => 6,
            _ => panic!(),
        };
    }
    score
}

fn part2(strats: &Strats) -> u32 {
    let mut score = 0;
    for strat in strats {
        score += match strat {
            ('A', 'X') => 3,
            ('A', 'Y') => 4,
            ('A', 'Z') => 8,
            ('B', 'X') => 1,
            ('B', 'Y') => 5,
            ('B', 'Z') => 9,
            ('C', 'X') => 2,
            ('C', 'Y') => 6,
            ('C', 'Z') => 7,
            _ => panic!(),
        };
    }
    score
}
