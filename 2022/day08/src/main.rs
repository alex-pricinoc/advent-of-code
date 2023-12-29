use day08::{Grid, GridCoord};

const INPUT: &str = include_str!("../input.txt");
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let grid = parse_grid(INPUT);

    println!("Part 1: {}", part_1(&grid));
    println!("Part 2: {}", part_2(&grid));
}

fn part_1(grid: &Grid<usize>) -> usize {
    let all_coords =
        (0..grid.height()).flat_map(|y| (0..grid.width()).map(move |x| GridCoord::from((x, y))));

    all_coords
        .filter(|&coord| visible_trees(grid, coord))
        .count()
}

fn part_2(grid: &Grid<usize>) -> usize {
    let all_coords =
        (0..grid.height()).flat_map(|y| (0..grid.width()).map(move |x| GridCoord::from((x, y))));

    all_coords
        .map(|coord| (coord, scenic_score(grid, coord)))
        .max_by_key(|(_, score)| *score)
        .unwrap()
        .1
}

fn visible_trees(grid: &Grid<usize>, coord: GridCoord) -> bool {
    DIRECTIONS
        .iter()
        .any(|&(dx, dy)| visible_trees_in_dir(grid, coord, (dx, dy)).0)
}

fn scenic_score(grid: &Grid<usize>, coord: GridCoord) -> usize {
    DIRECTIONS
        .into_iter()
        .map(|(dx, dy)| visible_trees_in_dir(grid, coord, (dx, dy)).1)
        .product()
}

fn visible_trees_in_dir(
    grid: &Grid<usize>,
    coord: GridCoord,
    (dx, dy): (isize, isize),
) -> (bool, usize) {
    let line = (1..).map_while(|i| {
        let coord = GridCoord {
            x: coord.x.checked_add_signed(dx * i)?,
            y: coord.y.checked_add_signed(dy * i)?,
        };
        Some(*grid.cell(coord)?)
    });

    let (mut visible, mut total) = (true, 0);
    let our_height = *grid.cell(coord).unwrap();
    for height in line {
        total += 1;
        if height >= our_height {
            visible = false;
            break;
        }
    }

    (visible, total)
}

fn parse_grid(input: &str) -> Grid<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let data = input
        .lines()
        .flat_map(|l| l.chars().map(|c| c as usize - '0' as usize))
        .collect();

    Grid::new(width, height, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
30373
25512
65332
33549
35390
";

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(&parse_grid(INPUT)), 21);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(&parse_grid(INPUT)), 8);
    }
}
