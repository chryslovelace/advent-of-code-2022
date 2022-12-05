use itertools::Itertools;

type Stacks = Vec<Vec<char>>;
struct Inst {
    n: usize,
    from: usize,
    to: usize,
}

fn main() {
    let (stack_drawing, instructions) = include_str!("input.txt")
        .split("\n\n")
        .collect_tuple()
        .unwrap();
    let mut stacks = vec![vec![]; 9];
    for line in stack_drawing.lines() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_alphabetic() {
                stacks[i].push(c);
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    let instructions = instructions
        .lines()
        .map(|instruction| {
            let (n, from, to) = instruction
                .chars()
                .group_by(|c| c.is_numeric())
                .into_iter()
                .filter_map(|(is_numeric, cs)| {
                    if is_numeric {
                        Some(cs.collect::<String>().parse::<usize>().unwrap())
                    } else {
                        None
                    }
                })
                .collect_tuple()
                .unwrap();
            Inst { n, from, to }
        })
        .collect_vec();

    part1(stacks.clone(), &instructions);
    part2(stacks, &instructions);
}

fn part1(mut stacks: Stacks, instructions: &[Inst]) {
    for Inst { n, from, to } in instructions {
        for _ in 0..*n {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    }
    print!("*  ");
    for stack in &stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}

fn part2(mut stacks: Stacks, instructions: &[Inst]) {
    for Inst { n, from, to } in instructions {
        let moving = {
            let from = &mut stacks[from - 1];
            from.drain(from.len() - n..).collect_vec()
        };
        stacks[to - 1].extend(moving.into_iter())
    }
    print!("** ");
    for stack in &stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
