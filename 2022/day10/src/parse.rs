use std::fmt;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Add(isize),
}

impl Instruction {
    pub fn parse(i: &str) -> Self {
        if i.starts_with("noop") {
            Self::Noop
        } else {
            let val = i.split_once(' ').unwrap().1.parse().unwrap();

            Self::Add(val)
        }
    }
}

#[derive(Debug)]
pub struct Crt {
    width: usize,
    data: Vec<char>,
}

impl Crt {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            data: vec!['.'; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    fn rows(&self) -> impl Iterator<Item = &[char]> {
        self.data.chunks_exact(self.width)
    }

    pub fn draw_pixel_at(&mut self, x: usize, y: usize) {
        self.data[y * self.width + x] = '#';
    }
}

impl fmt::Display for Crt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows() {
            for pixel in row {
                write!(f, "{pixel}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
