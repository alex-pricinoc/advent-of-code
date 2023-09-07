use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

fn part_1(data: &str) -> usize {
    let filesystem = FileSystem::new().parse(data);

    let mut total = 0;

    let mut to_visit = vec![filesystem.root];

    while let Some(dir) = to_visit.pop() {
        for d in dir.subdir.borrow().values() {
            to_visit.push(d.clone());
        }

        let size = dir.get_size();
        if size <= 100_000 {
            total += size;
        }
    }

    total
}

fn part_2(data: &str) -> usize {
    let total_disk_space = 70_000_000;
    let space_needed = 30_000_000;

    let filesystem = FileSystem::new().parse(data);
    let mut to_visit = vec![filesystem.root];

    let mut dir_sizes = vec![];

    while let Some(dir) = to_visit.pop() {
        for d in dir.subdir.borrow().values() {
            to_visit.push(d.clone());
        }

        let size = dir.get_size();

        dir_sizes.push(size);
    }

    dir_sizes.sort();

    let used_space = total_disk_space - dir_sizes.last().unwrap();

    let space_to_delete = space_needed - used_space;

    let found = dir_sizes.iter().find(|&x| x >= &space_to_delete).unwrap();

    *found
}

#[derive(Default)]
struct Dir {
    name: String,
    parent: Option<Rc<Dir>>,
    size: RefCell<usize>,
    subdir: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .subdir
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}

#[derive(Default)]
struct FileSystem {
    root: Rc<Dir>,
}

impl FileSystem {
    fn new() -> Self {
        Self::default()
    }

    fn parse(self, data: &str) -> Self {
        let mut cwd: Rc<Dir> = self.root.clone();

        for line in data.trim().lines() {
            let words = line.split_whitespace().collect::<Vec<&str>>();

            match (words[0], words[1]) {
                ("$", "ls") => {}
                ("$", "cd") => match words[2] {
                    "/" => cwd = self.root.clone(),
                    ".." => cwd = cwd.parent.clone().unwrap(),
                    dirname => {
                        let newdir = cwd.subdir.borrow().get(dirname).unwrap().clone();
                        cwd = newdir;
                    }
                },
                ("dir", name) => {
                    let dir = Dir {
                        name: name.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&cwd)),
                        subdir: RefCell::new(HashMap::new()),
                    };

                    cwd.subdir
                        .borrow_mut()
                        .insert(name.to_string(), Rc::new(dir));
                }
                (size, _name) => {
                    let size = size.parse::<usize>().unwrap();

                    *cwd.size.borrow_mut() += size;
                }
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lstm
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 95437);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(INPUT), 24933642);
    }
}
