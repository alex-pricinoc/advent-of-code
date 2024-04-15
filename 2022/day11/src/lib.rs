use std::error::Error;
use std::result;
use std::str::FromStr;

pub type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: Vec<u64>,
    pub items_inspected: u64,
    pub operation: Operation,
    pub divisor: u64,
    pub receiver_if_true: usize,
    pub receiver_if_false: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Add(Term, Term),
    Mul(Term, Term),
}

impl Operation {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Add(l, r) => l.eval(old) + r.eval(old),
            Operation::Mul(l, r) => l.eval(old) * r.eval(old),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Term {
    Old,
    Constant(u64),
}

impl Term {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Term::Old => old,
            Term::Constant(c) => c,
        }
    }
}

impl FromStr for Term {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let term = match s {
            "old" => Term::Old,
            n => Term::Constant(
                n.parse()
                    .map_err(|err| format!("failed to parse '{:?}': {}", n, err))?,
            ),
        };

        Ok(term)
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        lines.next().unwrap();

        let items = lines
            .next()
            .unwrap()
            .split_once("  Starting items: ")
            .unwrap()
            .1
            .split(", ")
            .flat_map(|s| s.parse())
            .collect();

        let operation = lines.next().unwrap();

        let operation = operation.split_once("  Operation: new = ").unwrap().1;

        let (l, op, r) = (|| {
            let mut s = operation.splitn(3, ' ');

            Some((s.next()?, s.next()?, s.next()?))
        })()
        .unwrap();

        let l = l.parse()?;
        let r = r.parse()?;

        let op = match op {
            "*" => Operation::Mul(l, r),
            "+" => Operation::Add(l, r),
            _ => unreachable!(),
        };

        let divisor = lines
            .next()
            .unwrap()
            .split_once(" Test: divisible by ")
            .unwrap()
            .1
            .parse()?;

        let receiver_if_true = lines
            .next()
            .unwrap()
            .chars()
            .next_back()
            .unwrap()
            .to_digit(10)
            .unwrap()
            .try_into()?;

        let receiver_if_false = lines
            .next()
            .unwrap()
            .chars()
            .next_back()
            .unwrap()
            .to_digit(10)
            .unwrap()
            .try_into()?;

        Ok(Self {
            items,
            items_inspected: 0,
            operation: op,
            divisor,
            receiver_if_true,
            receiver_if_false,
        })
    }
}
