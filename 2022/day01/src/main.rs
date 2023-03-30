fn main() {
    let input = include_str!("../input.txt");

    let mut cals = input
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();

    println!("part 1: {}", cals.iter().max().unwrap());

    cals.sort_unstable();

    println!("part 2: {}", cals.iter().rev().take(3).sum::<u32>());
}
