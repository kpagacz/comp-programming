type Map = Vec<Vec<char>>;
fn parse_input(input: &str) -> (Map, Vec<(usize, usize)>) {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let map = map.lines().map(|line| line.chars().collect()).collect();
    let moves = moves.lines().fold(Vec::new(), |mut moves, line| {
        moves.extend(line.chars().map(|c| match c {
            '^' => (usize::MAX, 0),
            '>' => (0, 1),
            '<' => (0, usize::MAX),
            'v' => (1, 0),
            _ => unreachable!(),
        }));
        moves
    });

    (map, moves)
}

const ROBOT: char = '@';
const WALL: char = '#';
const BOX: char = 'O';
const EMPTY: char = '.';
fn is_free(map: &Map, x: usize, y: usize) -> bool {
    map[x][y] == EMPTY
}

fn is_box(map: &Map, x: usize, y: usize) -> bool {
    map[x][y] == BOX || map[x][y] == LEFT_BOX || map[x][y] == RIGHT_BOX
}

fn new_coords(x: usize, y: usize, step: (usize, usize)) -> (usize, usize) {
    (x.wrapping_add(step.0), y.wrapping_add(step.1))
}

fn first_empty(
    mut x: usize,
    mut y: usize,
    step: (usize, usize),
    map: &Map,
) -> Option<(usize, usize)> {
    while is_box(map, x, y) {
        (x, y) = new_coords(x, y, step);
    }

    if is_free(map, x, y) {
        Some((x, y))
    } else {
        None
    }
}

fn move_line(
    mut first_x: usize,
    mut first_y: usize,
    empty_x: usize,
    empty_y: usize,
    step: (usize, usize),
    map: &mut Map,
) {
    let mut last_c = map[first_x][first_y];
    map[first_x][first_y] = EMPTY;
    while first_x != empty_x || first_y != empty_y {
        (first_x, first_y) = new_coords(first_x, first_y, step);
        std::mem::swap(&mut last_c, &mut map[first_x][first_y]);
    }
}

fn move_robot(map: &mut Map, step: (usize, usize), x: usize, y: usize) -> (usize, usize) {
    let (nx, ny) = new_coords(x, y, step);

    if is_free(map, nx, ny) {
        map[x][y] = EMPTY;
        map[nx][ny] = ROBOT;
        return (nx, ny);
    } else if is_box(map, nx, ny) {
        if let Some((empty_x, empty_y)) = first_empty(nx, ny, step, map) {
            move_line(x, y, empty_x, empty_y, step, map);
            return (nx, ny);
        }
    }

    (x, y)
}

fn score_boxes(map: &Map) -> usize {
    let mut score = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == BOX || map[x][y] == LEFT_BOX {
                score += 100 * x + y;
            }
        }
    }
    score
}

fn find_robot(map: &Map) -> (usize, usize) {
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == ROBOT {
                return (x, y);
            }
        }
    }
    unreachable!()
}

fn print_map(map: &Map) {
    for row in map {
        for c in row {
            print!("{c}");
        }
        println!()
    }
    println!()
}

fn part1(input: &str) -> usize {
    let (mut map, moves) = parse_input(input);

    let (mut robot_x, mut robot_y) = find_robot(&map);
    for step in moves {
        (robot_x, robot_y) = move_robot(&mut map, step, robot_x, robot_y);
    }
    score_boxes(&map)
}

const LEFT_BOX: char = '[';
const RIGHT_BOX: char = ']';
fn double_map(map: Map) -> Map {
    let mut doubled = vec![vec!['.'; map[0].len() * 2]; map.len()];

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            match map[x][y] {
                ROBOT => {
                    doubled[x][2 * y] = ROBOT;
                    doubled[x][2 * y + 1] = EMPTY;
                }
                WALL => {
                    doubled[x][2 * y] = WALL;
                    doubled[x][2 * y + 1] = WALL;
                }
                BOX => {
                    doubled[x][2 * y] = LEFT_BOX;
                    doubled[x][2 * y + 1] = RIGHT_BOX;
                }
                EMPTY => {}
                _ => unreachable!(),
            }
        }
    }
    doubled
}

const LEFT_STEP: (usize, usize) = (0, usize::MAX);
const RIGHT_STEP: (usize, usize) = (0, 1);
fn doubled_can_move(map: &Map, x: usize, y: usize, step: (usize, usize)) -> bool {
    match map[x][y] {
        ROBOT => {
            let (nx, ny) = new_coords(x, y, step);
            doubled_can_move(map, nx, ny, step)
        }
        WALL => false,
        EMPTY => true,
        LEFT_BOX => {
            if step == LEFT_STEP {
                let (nx, ny) = new_coords(x, y, step);
                doubled_can_move(map, nx, ny, step)
            } else if step == RIGHT_STEP {
                let (nx2, ny2) = new_coords(x, y + 1, step);
                doubled_can_move(map, nx2, ny2, step)
            } else {
                let (nx, ny) = new_coords(x, y, step);
                let (nx2, ny2) = new_coords(x, y + 1, step);
                doubled_can_move(map, nx, ny, step) && doubled_can_move(map, nx2, ny2, step)
            }
        }
        RIGHT_BOX => {
            if step == LEFT_STEP {
                let (nx, ny) = new_coords(x, y - 1, step);
                doubled_can_move(map, nx, ny, step)
            } else if step == RIGHT_STEP {
                let (nx2, ny2) = new_coords(x, y, step);
                doubled_can_move(map, nx2, ny2, step)
            } else {
                let (nx, ny) = new_coords(x, y, step);
                let (nx2, ny2) = new_coords(x, y - 1, step);
                doubled_can_move(map, nx, ny, step) && doubled_can_move(map, nx2, ny2, step)
            }
        }
        _ => unreachable!(),
    }
}

fn doubled_move_stuff(map: &mut Map, x: usize, y: usize, step: (usize, usize)) {
    let mut queue: Vec<(usize, usize, char, i32)> = Vec::new();
    const VERTICAL: i32 = 1;
    const HORIZONTAL: i32 = 0;
    queue.push((x, y, EMPTY, VERTICAL));

    fn type_from_step(step: (usize, usize)) -> i32 {
        if step == LEFT_STEP || step == RIGHT_STEP {
            HORIZONTAL
        } else {
            VERTICAL
        }
    }

    while !queue.is_empty() {
        let mut new_queue = Vec::new();
        for (x, y, c, move_type) in queue.into_iter() {
            match map[x][y] {
                ROBOT => {
                    let (nx, ny) = new_coords(x, y, step);
                    map[x][y] = EMPTY;
                    new_queue.push((nx, ny, ROBOT, type_from_step(step)));
                }
                EMPTY => {
                    map[x][y] = c;
                }
                LEFT_BOX => {
                    if move_type == VERTICAL {
                        let (nx, ny) = new_coords(x, y, step);
                        let (nx2, ny2) = new_coords(x, y + 1, step);
                        map[x][y] = c;
                        new_queue.push((nx, ny, LEFT_BOX, type_from_step(step)));
                        let second = (nx2, ny2, RIGHT_BOX, type_from_step(step));
                        if !new_queue.contains(&second) {
                            new_queue.push(second);
                            map[x][y + 1] = EMPTY;
                        }
                    } else {
                        let (nx, ny) = new_coords(x, y, step);
                        map[x][y] = c;
                        new_queue.push((nx, ny, LEFT_BOX, type_from_step(step)));
                    }
                }
                RIGHT_BOX => {
                    if move_type == VERTICAL {
                        let (nx, ny) = new_coords(x, y, step);
                        let (nx2, ny2) = new_coords(x, y - 1, step);
                        map[x][y] = c;
                        new_queue.push((nx, ny, RIGHT_BOX, type_from_step(step)));
                        let second = (nx2, ny2, LEFT_BOX, type_from_step(step));
                        if !new_queue.contains(&second) {
                            new_queue.push(second);
                            map[x][y - 1] = EMPTY;
                        }
                    } else {
                        let (nx, ny) = new_coords(x, y, step);
                        map[x][y] = c;
                        new_queue.push((nx, ny, RIGHT_BOX, type_from_step(step)));
                    }
                }
                _ => unreachable!(
                    "The move was checked to be possible on {x} {y} {c} {}",
                    map[x][y]
                ),
            }
        }

        queue = new_queue;
    }
}

fn doubled_move_robot(map: &mut Map, step: (usize, usize), x: usize, y: usize) -> (usize, usize) {
    let (nx, ny) = new_coords(x, y, step);
    if is_free(map, nx, ny) {
        map[nx][ny] = ROBOT;
        map[x][y] = EMPTY;
        return (nx, ny);
    } else if is_box(map, nx, ny) && doubled_can_move(map, x, y, step) {
        doubled_move_stuff(map, x, y, step);
        return (nx, ny);
    }

    (x, y)
}

fn part2(input: &str) -> usize {
    let (map, moves) = parse_input(input);
    let mut map = double_map(map);

    let (mut robot_x, mut robot_y) = find_robot(&map);
    print_map(&map);
    for step in moves {
        (robot_x, robot_y) = doubled_move_robot(&mut map, step, robot_x, robot_y);
    }
    print_map(&map);
    score_boxes(&map)
}

fn main() {
    let test = include_str!("../test");
    let test2 = include_str!("../test2");
    let input = include_str!("../input");

    println!("Part 1 (test): {}, expected 2028", part1(test));
    println!("Part 1 (test): {}, expected 10092", part1(test2));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (test): {}, expected 9021", part2(test2));
    println!("Part 2: {}", part2(input));
}
