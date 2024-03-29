use parse::*;
use std::collections::{HashMap, VecDeque};
use std::fmt;

mod parse;

#[derive(Debug)]
struct App {
    instructions: VecDeque<Instruction>,
    cycle: usize,
    regx: isize,
    signals: HashMap<usize, Option<isize>>,
    crt: Crt,
}

impl App {
    fn new(input: &str) -> Self {
        let instructions = input.lines().map(Instruction::parse).collect();

        Self {
            cycle: 0,
            regx: 1,
            instructions,
            signals: HashMap::from([
                (20, None),
                (60, None),
                (100, None),
                (140, None),
                (180, None),
                (220, None),
            ]),
            crt: Crt::new(40, 6),
        }
    }

    fn draw(&mut self) {
        let pos = self.cycle - 1;

        let y = pos / self.crt.width();
        let x = pos % self.crt.width();

        if self.regx.abs_diff(x as isize) < 2 {
            self.crt.draw_pixel_at(x, y);
        }
    }

    fn cycle(&mut self) {
        self.cycle += 1;

        if let Some(signal) = self.signals.get_mut(&self.cycle) {
            let strength = self.cycle as isize * self.regx;
            *signal = Some(strength);
        }

        self.draw();
    }

    fn execute_instructions(&mut self) {
        while let Some(instruction) = self.instructions.pop_front() {
            match instruction {
                Instruction::Noop => self.cycle(),
                Instruction::Add(val) => {
                    self.cycle();
                    self.cycle();
                    self.regx += val;
                }
            }
        }
    }
}

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: \n{}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let mut app = App::new(input);

    app.execute_instructions();

    let sum = app.signals.values().flatten().sum::<isize>();

    sum as _
}

fn part_2(input: &str) -> impl fmt::Display {
    let mut app = App::new(input);

    app.execute_instructions();

    let _sum = app.signals.values().flatten().sum::<isize>();

    app.crt
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 13140);
    }

    #[test]
    fn part_2_test() {
        let output = part_2(INPUT);

        assert_eq!(
            output.to_string(),
            "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
