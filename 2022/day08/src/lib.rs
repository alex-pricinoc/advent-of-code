use std::convert::Infallible;
use std::str::FromStr;

const LEFT: (i32, i32) = (-1, 0);
const UP: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (1, 0);
const DOWN: (i32, i32) = (0, 1);

pub struct Forest {
    width: usize,

    trees: Vec<char>,
}

impl Forest {
    pub fn width(&self) -> i32 {
        self.width as _
    }

    pub fn height(&self) -> i32 {
        (self.trees.len() / self.width) as _
    }

    pub fn tree_at(&self, x: i32, y: i32) -> char {
        self.trees[(x + y * self.width()) as usize]
    }

    fn tree_is_at_edge(&self, x: i32, y: i32) -> bool {
        x == 0 || y == 0 || x == self.width() - 1 || y == self.height() - 1
    }

    pub fn tree_visible(&self, x: i32, y: i32) -> (bool, i32) {
        let tree = self.tree_at(x, y);

        let mut res = (false, 1);

        for dir in [LEFT, UP, DOWN, RIGHT] {
            let (mut x, mut y) = (x, y);
            let (mut visible, mut score) = (true, 0);

            loop {
                if self.tree_is_at_edge(x, y) {
                    break;
                }

                x += dir.0;
                y += dir.1;
                score += 1;

                if tree <= self.tree_at(x, y) {
                    visible = false;
                    break;
                }
            }

            res = (res.0 || visible, res.1 * score);
        }

        res
    }
}

impl FromStr for Forest {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().chars().count();

        let trees = s.lines().flat_map(|l| l.chars()).collect();

        Ok(Self { width, trees })
    }
}
