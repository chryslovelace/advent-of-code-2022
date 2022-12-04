use itertools::Itertools;

type Section = std::ops::RangeInclusive<u32>;

fn main() {
    let input: Vec<(Section, Section)> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut ids = line.split([',', '-']).map(|n| n.parse().unwrap());
            let elf1: (u32, u32) = ids.next_tuple().unwrap();
            let elf2: (u32, u32) = ids.next_tuple().unwrap();
            (elf1.0..=elf1.1, elf2.0..=elf2.1)
        })
        .collect();
    println!("*  {}", part1(input.iter().cloned()));
    println!("** {}", part2(input.into_iter()));
}

fn part1(sections: impl Iterator<Item = (Section, Section)>) -> usize {
    sections
        .filter(|(elf1, elf2)| {
            (elf1.contains(elf2.start()) && elf1.contains(elf2.end()))
                || (elf2.contains(elf1.start()) && elf2.contains(elf1.end()))
        })
        .count()
}

fn part2(sections: impl Iterator<Item = (Section, Section)>) -> usize {
    sections
        .filter(|(elf1, elf2)| elf1.clone().any(|x| elf2.contains(&x)))
        .count()
}
