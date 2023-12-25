use smallvec::SmallVec;
use std::{convert::Infallible, fmt, str::FromStr};

#[derive(Clone, Copy)]
pub struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct Piles(Vec<Vec<Crate>>);

impl Piles {
    pub fn apply(&mut self, ins: Instruction, moving: bool) {
        if moving {
            let amount = (self.0[ins.from].len() - ins.amount)..;
            let mut values = self.0[ins.from].splice(amount, []).collect();
            self.0[ins.to].append(&mut values);
        } else {
            for _ in 0..ins.amount {
                let el = self.0[ins.from].pop().unwrap();
                self.0[ins.to].push(el);
            }
        }
    }

    pub fn word(&self) -> String {
        self.0.iter().map(|pile| pile.last().unwrap().0).collect()
    }
}

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Piles {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut piles = Vec::new();

        for line in s.lines().rev().skip(1) {
            for (i, c) in line
                .chars()
                .enumerate()
                .filter(|(i, c)| *c != ' ' && (i + 3) % 4 == 0)
            {
                let stack = (i - 1) / 4;

                while piles.len() < stack + 1 {
                    piles.push(Vec::new());
                }

                piles[stack].push(Crate(c));
            }
        }

        Ok(Piles(piles))
    }
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split_whitespace()
            .flat_map(|i| i.parse())
            .collect::<SmallVec<[usize; 3]>>();

        Ok(Self {
            amount: v[0],
            from: v[1] - 1,
            to: v[2] - 1,
        })
    }
}
