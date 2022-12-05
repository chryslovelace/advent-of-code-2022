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

fn borrow_two<T>(slice: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    assert!(i != j);
    if i < j {
        let (left, right) = slice.split_at_mut(i + 1);
        (&mut left[i], &mut right[j - i - 1])
    } else {
        let (x, y) = borrow_two(slice, j, i);
        (y, x)
    }
}

fn part1(mut stacks: Stacks, instructions: &[Inst]) {
    for Inst { n, from, to } in instructions {
        let (from, to) = borrow_two(&mut stacks, from - 1, to - 1);
        for _ in 0..*n {
            to.push(from.pop().unwrap());
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
        let (from, to) = borrow_two(&mut stacks, from - 1, to - 1);
        to.extend(from.drain(from.len() - n..));
    }
    print!("** ");
    for stack in &stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
