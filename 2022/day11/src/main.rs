use day11::{Monkey, Result};
use std::result;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    let mut monkeys = INPUT
        .split("\n\n")
        .map(FromStr::from_str)
        .collect::<result::Result<Vec<Monkey>, _>>()?;

    let divisor_product = monkeys.iter().map(|m| m.divisor).product::<u64>();
    dbg!(divisor_product);

    for _ in 0..10_000 {
        do_round(&mut monkeys, divisor_product);
    }

    let mut all_inspect_counts = monkeys
        .iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<_>>();

    all_inspect_counts.sort_by_key(|&c| std::cmp::Reverse(c));

    let monkey_business = all_inspect_counts.into_iter().take(2).product::<u64>();

    dbg!(monkey_business);

    Ok(())
}

fn do_round(monkeys: &mut [Monkey], divisor_product: u64) {
    let num_monkeys = monkeys.len();

    for i in 0..num_monkeys {
        let mc;

        {
            let monkey = &mut monkeys[i];
            mc = monkey.clone();
            monkey.items_inspected += mc.items.len() as u64;
        }

        for mut item in mc.items.iter().copied() {
            item %= divisor_product;
            item = mc.operation.eval(item);
            if item % mc.divisor == 0 {
                monkeys[mc.receiver_if_true].items.push(item);
            } else {
                monkeys[mc.receiver_if_false].items.push(item);
            }
        }

        monkeys[i].items.clear();
    }
}
