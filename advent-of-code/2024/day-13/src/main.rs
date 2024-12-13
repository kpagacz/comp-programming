#[derive(Debug)]
struct Game {
    abtn: (usize, usize),
    bbtn: (usize, usize),
    prize: (usize, usize),
}

impl Game {
    fn new(ax: usize, ay: usize, bx: usize, by: usize, prizex: usize, prizey: usize) -> Self {
        Self {
            abtn: (ax, ay),
            bbtn: (bx, by),
            prize: (prizex, prizey),
        }
    }
}

use regex::Regex;
use std::sync::LazyLock;
static GAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)\s*"#,
    )
    .expect("REGEX IS FUCKED")
});
impl std::str::FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Game, Self::Err> {
        if let Some(caps) = GAME_REGEX.captures(s) {
            let (_, [x1, y1, x2, y2, xp, yp]) = caps.extract();
            Ok(Game::new(
                x1.parse::<usize>().unwrap(),
                y1.parse::<usize>().unwrap(),
                x2.parse::<usize>().unwrap(),
                y2.parse::<usize>().unwrap(),
                xp.parse::<usize>().unwrap(),
                yp.parse::<usize>().unwrap(),
            ))
        } else {
            Err("WTF?!".to_string())
        }
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .split("\n\n")
        .map(|game| game.parse::<Game>().unwrap())
        .collect()
}

fn calculate_token_cost(game: Game, acost: usize, bcost: usize) -> Option<usize> {
    for a_presses in 0..=100 {
        let moved_x = game.abtn.0 * a_presses;
        let moved_y = game.abtn.1 * a_presses;

        if moved_x > game.prize.0 || moved_y > game.prize.1 {
            continue;
        }
        if (game.prize.0 - moved_x) % game.bbtn.0 == 0
            && (game.prize.1 - moved_y) % game.bbtn.1 == 0
            && (game.prize.0 - moved_x) / game.bbtn.0 == (game.prize.1 - moved_y) / game.bbtn.1
        {
            return Some(a_presses * acost + bcost * (game.prize.0 - moved_x) / game.bbtn.0);
        }
    }
    None
}

fn part1(input: &str) -> usize {
    let games = parse_input(input);

    games
        .into_iter()
        .filter_map(|game| calculate_token_cost(game, 3, 1))
        .sum::<usize>()
}

fn calculate_token_cost_with_matrices(
    game: Game,
    acost: usize,
    bcost: usize,
    offset: i64,
) -> Option<usize> {
    let (x1, x2, y1, y2, xp, yp) = (
        game.abtn.0 as i64,
        game.bbtn.0 as i64,
        game.abtn.1 as i64,
        game.bbtn.1 as i64,
        game.prize.0 as i64 + offset,
        game.prize.1 as i64 + offset,
    );

    // | x1 x2 | * | a | = | xp |
    // | y1 y2 |   | b |   | yp |
    let det = x1 * y2 - x2 * y1;
    if det == 0 {
        return None;
    }
    let det_a = xp * y2 - x2 * yp;
    if det_a % det != 0 {
        return None;
    }

    let a_presses = det_a / det;
    if a_presses < 0 {
        return None;
    }

    let det_b = x1 * yp - xp * y1;
    let b_presses = det_b / det;
    if det_b % det != 0 {
        return None;
    }
    if b_presses < 0 {
        return None;
    }

    Some(a_presses as usize * acost + b_presses as usize * bcost)
}

fn part2(input: &str, offset: i64) -> usize {
    let games = parse_input(input);

    games
        .into_iter()
        .filter_map(|game| calculate_token_cost_with_matrices(game, 3, 1, offset))
        .sum::<usize>()
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");

    println!("Part 1 (test): {} expected 480", part1(test));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (test): {} expected 480", part2(test, 0));
    println!(
        "Part 2 (test): {} expected {}",
        part2(input, 0),
        part1(input)
    );
    println!("Part 2: {}", part2(input, 10000000000000));
}
