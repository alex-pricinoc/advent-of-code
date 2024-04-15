use day11::Monkey;

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    println!("Part 1: {}", part_1(INPUT));
    // println!("Part 2: \n{}", part_2(INPUT));
}

fn part_1(input: &'static str) -> usize {
    let mut monkeys = input.split("\n\n").map(Monkey::parse).collect::<Vec<_>>();

    let rounds = 20;

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some(i) = monkeys[m].items.pop_front() {
                monkeys[m].items_inspected += 1;
                let new_val = (monkeys[m].operation)(i) / 3;
                let throw_to = (monkeys[m].test)(new_val);
                monkeys[throw_to].items.push_back(new_val);
            }
        }
    }

    let mut items_inspected = monkeys
        .iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<_>>();

    items_inspected.sort_by(|a, b| a.cmp(b).reverse());

    items_inspected[0] * items_inspected[1]
}

fn part_2(input: &'static str) -> usize {
    let mut monkeys = input.split("\n\n").map(Monkey::parse).collect::<Vec<_>>();

    let rounds = 10000;

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some(i) = monkeys[m].items.pop_front() {
                monkeys[m].items_inspected += 1;
                let new_val = (monkeys[m].operation)(i);
                let throw_to = (monkeys[m].test)(new_val);
                monkeys[throw_to].items.push_back(new_val);
            }
        }
    }

    dbg!(monkeys);

    unimplemented!()

    // let mut items_inspected = monkeys
    //     .iter()
    //     .map(|m| m.items_inspected)
    //     .collect::<Vec<_>>();

    // items_inspected.sort_by(|a, b| a.cmp(b).reverse());

    // items_inspected[0] * items_inspected[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 10605);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 2713310158);
    }
}
