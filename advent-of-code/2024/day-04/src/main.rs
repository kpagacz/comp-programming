fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let mut answer = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            for row_d in [usize::MAX, 0, 1] {
                for col_d in [usize::MAX, 0, 1] {
                    answer += small_flood_directed(&grid, 'X', 'S', i, j, (row_d, col_d));
                }
            }
        }
    }
    answer
}

fn small_flood_directed(
    grid: &[Vec<char>],
    target: char,
    last_char: char,
    row: usize,
    col: usize,
    direction: (usize, usize),
) -> usize {
    if grid[row][col] != target {
        return 0;
    }
    if target == last_char {
        return 1;
    }
    let next_target = match target {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => unreachable!(),
    };

    let new_row = row.wrapping_add(direction.0);
    let new_col = col.wrapping_add(direction.1);
    if new_row < grid.len() && new_col < grid[0].len() {
        small_flood_directed(grid, next_target, last_char, new_row, new_col, direction)
    } else {
        0
    }
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let mut answer = 0;

    fn check_x_mas(grid: &[Vec<char>], row: usize, col: usize) -> Option<bool> {
        let first_beam = [
            *grid
                .get(row.wrapping_add(usize::MAX))?
                .get(col.wrapping_add(usize::MAX))?,
            grid[row][col],
            *grid.get(row + 1)?.get(col + 1)?,
        ];
        let second_beam = [
            *grid.get(row.wrapping_add(usize::MAX))?.get(col + 1)?,
            grid[row][col],
            *grid.get(row + 1)?.get(col.wrapping_add(usize::MAX))?,
        ];

        let target = ['M', 'A', 'S'];
        if (first_beam == target || first_beam.iter().rev().eq(target.iter()))
            && (second_beam == target || second_beam.iter().rev().eq(target.iter()))
        {
            Some(true)
        } else {
            Some(false)
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if check_x_mas(&grid, i, j).unwrap_or(false) {
                answer += 1;
            }
        }
    }

    answer
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");

    println!("Part 1: {}", part1(input));
    println!("Part 1 (test): {}", part1(test));

    println!("Part 2: {}", part2(input));
    println!("Part 2 (test): {}", part2(test));
}
