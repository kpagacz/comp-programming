fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let lines: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut answer = vec![vec![b'*'; lines[0].len() + 2]; lines.len() + 2];

    for i in 1..answer.len() - 1 {
        for j in 1..answer[0].len() - 1 {
            answer[i][j] = lines[i - 1][j - 1];
        }
    }

    answer
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Point = (usize, usize);

#[derive(Debug, Clone)]
struct Cell {
    energized: bool,
    directions: Vec<Direction>,
}

impl Cell {
    fn new() -> Self {
        Self {
            energized: false,
            directions: vec![],
        }
    }
}

const BORDER: u8 = b'*';
const VSPLIT: u8 = b'|';
const HSPLIT: u8 = b'-';
const FMIRROR: u8 = b'/';
const BMIRROR: u8 = b'\\';
const DOT: u8 = b'.';

fn beam(
    mut start: Point,
    direction: Direction,
    grid: &[Vec<u8>],
    grid_with_energy: &mut [Vec<Cell>],
) {
    // End conditions
    if grid[start.0][start.1] == b'*' {
        return;
    }
    if grid_with_energy[start.0][start.1]
        .directions
        .contains(&direction)
    {
        return;
    }

    // Mark energy
    grid_with_energy[start.0][start.1].energized = true;
    grid_with_energy[start.0][start.1]
        .directions
        .push(direction);

    let pass_through_types = match direction {
        Direction::Up | Direction::Down => [DOT, VSPLIT],
        Direction::Left | Direction::Right => [DOT, HSPLIT],
    };

    while pass_through_types.contains(&grid[start.0][start.1]) {
        grid_with_energy[start.0][start.1].energized = true;
        grid_with_energy[start.0][start.1]
            .directions
            .push(direction);

        match direction {
            Direction::Up => start.0 -= 1,
            Direction::Down => start.0 += 1,
            Direction::Left => start.1 -= 1,
            Direction::Right => start.1 += 1,
        }
    }

    match grid[start.0][start.1] {
        HSPLIT | VSPLIT | FMIRROR | BMIRROR => {
            grid_with_energy[start.0][start.1].energized = true;
            grid_with_energy[start.0][start.1]
                .directions
                .push(direction);
        }
        _ => {}
    }

    match grid[start.0][start.1] {
        HSPLIT => {
            beam(
                (start.0, start.1 - 1),
                Direction::Left,
                grid,
                grid_with_energy,
            );
            beam(
                (start.0, start.1 + 1),
                Direction::Right,
                grid,
                grid_with_energy,
            );
        }
        VSPLIT => {
            beam(
                (start.0 - 1, start.1),
                Direction::Up,
                grid,
                grid_with_energy,
            );
            beam(
                (start.0 + 1, start.1),
                Direction::Down,
                grid,
                grid_with_energy,
            );
        }
        // /
        FMIRROR => match direction {
            Direction::Up => beam(
                (start.0, start.1 + 1),
                Direction::Right,
                grid,
                grid_with_energy,
            ),
            Direction::Down => beam(
                (start.0, start.1 - 1),
                Direction::Left,
                grid,
                grid_with_energy,
            ),
            Direction::Left => beam(
                (start.0 + 1, start.1),
                Direction::Down,
                grid,
                grid_with_energy,
            ),
            Direction::Right => beam(
                (start.0 - 1, start.1),
                Direction::Up,
                grid,
                grid_with_energy,
            ),
        },
        // \
        BMIRROR => match direction {
            Direction::Up => beam(
                (start.0, start.1 - 1),
                Direction::Left,
                grid,
                grid_with_energy,
            ),
            Direction::Down => beam(
                (start.0, start.1 + 1),
                Direction::Right,
                grid,
                grid_with_energy,
            ),
            Direction::Left => beam(
                (start.0 - 1, start.1),
                Direction::Up,
                grid,
                grid_with_energy,
            ),
            Direction::Right => beam(
                (start.0 + 1, start.1),
                Direction::Down,
                grid,
                grid_with_energy,
            ),
        },
        BORDER => {}
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);

    let mut grid_with_energy = vec![vec![Cell::new(); grid[0].len()]; grid.len()];
    beam((1, 1), Direction::Right, &grid, &mut grid_with_energy);

    grid_with_energy
        .into_iter()
        .map(|row| row.into_iter().filter(|c| c.energized).count())
        .sum()
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let mut answer = 0;
    (1..grid[0].len() - 1).for_each(|col| {
        let mut initial_point = (1, col);
        let mut grid_with_energy = vec![vec![Cell::new(); grid[0].len()]; grid.len()];
        beam(initial_point, Direction::Down, &grid, &mut grid_with_energy);
        let energized = grid_with_energy
            .into_iter()
            .map(|row| row.into_iter().filter(|c| c.energized).count())
            .sum();
        answer = answer.max(energized);

        initial_point = (grid.len() - 2, col);
        let mut grid_with_energy = vec![vec![Cell::new(); grid[0].len()]; grid.len()];
        beam(initial_point, Direction::Up, &grid, &mut grid_with_energy);
        let energized = grid_with_energy
            .into_iter()
            .map(|row| row.into_iter().filter(|c| c.energized).count())
            .sum();
        answer = answer.max(energized);
    });
    (1..grid.len() - 1).for_each(|row| {
        let mut initial_point = (row, 1);
        let mut grid_with_energy = vec![vec![Cell::new(); grid[0].len()]; grid.len()];
        beam(
            initial_point,
            Direction::Right,
            &grid,
            &mut grid_with_energy,
        );
        let energized = grid_with_energy
            .into_iter()
            .map(|row| row.into_iter().filter(|c| c.energized).count())
            .sum();
        answer = answer.max(energized);

        initial_point = (row, grid[0].len() - 2);
        let mut grid_with_energy = vec![vec![Cell::new(); grid[0].len()]; grid.len()];
        beam(initial_point, Direction::Left, &grid, &mut grid_with_energy);
        let energized = grid_with_energy
            .into_iter()
            .map(|row| row.into_iter().filter(|c| c.energized).count())
            .sum();
        answer = answer.max(energized);
    });
    answer
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));
    println!("Part 2: {}", part2(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
