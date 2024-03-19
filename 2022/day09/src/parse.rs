use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl fmt::Debug for GridPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for GridPos {
    type Output = GridPos;

    fn add(self, other: GridPos) -> GridPos {
        GridPos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for GridPos {
    fn add_assign(&mut self, other: GridPos) {
        *self = GridPos {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::Sub for GridPos {
    type Output = GridPos;

    fn sub(self, other: GridPos) -> GridPos {
        GridPos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn parse(i: &str) -> Self {
        use Direction::*;

        match i {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => unreachable!(),
        }
    }

    pub fn delta(self) -> GridPos {
        use Direction::*;
        match self {
            Up => GridPos { x: 0, y: -1 },
            Down => GridPos { x: 0, y: 1 },
            Left => GridPos { x: -1, y: 0 },
            Right => GridPos { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub dir: Direction,
    pub dist: u32,
}

impl Instruction {
    pub fn parse(i: &str) -> Self {
        let (dir, dist) = i.split_once(' ').unwrap();

        let dir = Direction::parse(dir);
        let dist = dist.parse().unwrap();

        Self { dir, dist }
    }
}
