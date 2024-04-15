use day11::{Monkey, Result};
use std::result;
use std::str::FromStr;

fn main() -> Result<()> {
    const INPUT: &str = include_str!("../input.txt");

    println!("Part 1: {}", part_1(INPUT)?);
    // println!("Part 2: \n{}", part_2(INPUT));

    Ok(())
}

fn do_round(monkeys: &mut [Monkey]) {
    let num_monkeys = monkeys.len();

    for i in 0..num_monkeys {
        let mc;

        {
            let monkey = &mut monkeys[i];
            mc = monkey.clone();
            monkey.items_inspected += mc.items.len() as u64;
        }

        for mut item in mc.items.iter().copied() {
            item = mc.operation.eval(item);
            item /= 3;
            if item % mc.divisor == 0 {
                monkeys[mc.receiver_if_true].items.push(item);
            } else {
                monkeys[mc.receiver_if_false].items.push(item);
            }
        }

        monkeys[i].items.clear();
    }
}

fn part_1(input: &str) -> Result<u64> {
    let mut monkeys = input
        .split("\n\n")
        .map(FromStr::from_str)
        .collect::<result::Result<Vec<Monkey>, _>>()?;

    for _ in 0..20 {
        do_round(&mut monkeys);
    }

    let mut all_inspect_counts = monkeys
        .iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<_>>();

    all_inspect_counts.sort_by_key(|&c| std::cmp::Reverse(c));

    let monkey_business = all_inspect_counts.into_iter().take(2).product();

    Ok(monkey_business)
}

fn part_2(input: &str) -> Result<u64> {
    // let mut monkeys = input.split("\n\n").map(Monkey::parse).collect::<Vec<_>>();

    // let rounds = 10000;

    // for _ in 0..rounds {
    //     for m in 0..monkeys.len() {
    //         while let Some(i) = monkeys[m].items.pop_front() {
    //             monkeys[m].items_inspected += 1;
    //             let new_val = (monkeys[m].operation)(i);
    //             let throw_to = (monkeys[m].test)(new_val);
    //             monkeys[throw_to].items.push_back(new_val);
    //         }
    //     }
    // }

    // dbg!(monkeys);

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
        assert_eq!(part_1(INPUT).unwrap(), 10605);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT).unwrap(), 2713310158);
    }
}
