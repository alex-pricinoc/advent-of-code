use std::collections::VecDeque;
use std::fmt;

pub struct Monkey {
    pub items_inspected: usize,
    pub items: VecDeque<u128>,
    pub operation: Box<dyn Fn(u128) -> u128>,
    pub test: Box<dyn Fn(u128) -> usize>,
}

impl Monkey {
    pub fn parse(i: &'static str) -> Self {
        let mut lines = i.lines();

        lines.next().unwrap();

        let items = lines.next().unwrap();
        let items = items
            .split_once("  Starting items: ")
            .unwrap()
            .1
            .split(", ")
            .flat_map(|s| s.parse())
            .collect();

        let operation = lines.next().unwrap();

        let operation = operation.split_once("  Operation: new = ").unwrap().1;

        let (_, op, c) = (|| {
            let mut s = operation.splitn(3, ' ');

            Some((s.next()?, s.next()?, s.next()?))
        })()
        .unwrap();

        let operation = move |old: u128| {
            let c = match c {
                "old" => old,
                n => n.parse().unwrap(),
            };

            match op {
                "+" => old + c,
                "*" => {
                    dbg!(old, c);

                    old * c
                }
                _ => unreachable!(),
            }
        };

        let test = lines.next().unwrap();

        let divisor: usize = test
            .split_once(" Test: divisible by ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let if_true = lines
            .next()
            .unwrap()
            .chars()
            .next_back()
            .unwrap()
            .to_digit(10)
            .unwrap();

        let if_false = lines
            .next()
            .unwrap()
            .chars()
            .next_back()
            .unwrap()
            .to_digit(10)
            .unwrap();

        let test = move |val: u128| -> usize {
            if val % divisor as u128 == 0 {
                if_true as _
            } else {
                if_false as _
            }
        };

        Self {
            items,
            items_inspected: 0,
            operation: Box::new(operation),
            test: Box::new(test),
        }
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("items_inspected", &self.items_inspected)
            .finish()
    }
}
