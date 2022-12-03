fn main() {
    let mut elves = Vec::new();
    let mut elf = 0;
    for line in include_str!("input.txt").lines() {
        if line.is_empty() {
            elves.push(std::mem::replace(&mut elf, 0));
        } else {
            elf += line.parse::<u32>().unwrap();
        }
    }
    elves.sort_by(|a, b| b.cmp(a));
    println!("*  {}", elves[0]);
    println!("** {}", elves[0] + elves[1] + elves[2]);
}
