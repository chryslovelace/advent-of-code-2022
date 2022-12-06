use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("*  {}", find_marker(input, 4));
    println!("** {}", find_marker(input, 14));
}

fn find_marker(input: &str, len: usize) -> usize {
    input
        .as_bytes()
        .windows(len)
        .find_position(|chunk| chunk.iter().all_unique())
        .unwrap()
        .0
        + len
}
