use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Clone, Debug, Ord, Eq, PartialOrd, PartialEq)]
enum BallColor {
    Green,
    Red,
    Blue,
}

fn parse_one_ball(input: &str) -> IResult<&str, (usize, BallColor)> {
    tuple((
        map(tuple((u32, multispace0)), |(n, _)| n as usize),
        alt((
            map(tag("green"), |_| BallColor::Green),
            map(tag("blue"), |_| BallColor::Blue),
            map(tag("red"), |_| BallColor::Red),
        )),
    ))(input)
}

use std::collections::BTreeMap;
type BallMap = BTreeMap<BallColor, usize>;
fn parse_one_ball_draw(input: &str) -> IResult<&str, BallMap> {
    map(
        separated_list1(tuple((multispace0, tag(","), multispace0)), parse_one_ball),
        |balls| {
            balls
                .into_iter()
                .fold(BallMap::new(), |mut map, (count, color)| {
                    *map.entry(color).or_insert(0) += count;
                    map
                })
        },
    )(input)
}

fn parse_game(input: &str) -> IResult<&str, (usize, Vec<BallMap>)> {
    tuple((
        delimited(tag("Game "), map(u32, |n| n as usize), tag(": ")),
        separated_list1(tag("; "), parse_one_ball_draw),
    ))(input)
}

fn part1(input: &str) -> usize {
    let limits = BallMap::from([
        (BallColor::Red, 12),
        (BallColor::Green, 13),
        (BallColor::Blue, 14),
    ]);
    input
        .lines()
        .filter_map(|line| {
            let (_, (game_id, draws)) = parse_game(line).expect("Game is always parsable");
            for draw in draws {
                for (color, count) in draw {
                    if **limits.get(&color).get_or_insert(&0) < count {
                        return None;
                    }
                }
            }
            Some(game_id)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, (_, draws)) = parse_game(line).expect("Game is always parsable");
            let mut maxes = BallMap::new();
            for draw in draws {
                for (colour, count) in draw {
                    let current = maxes.get(&colour).unwrap_or_else(|| &0);
                    *maxes.entry(colour).or_insert(0) = *current.max(&count);
                }
            }

            maxes
                .into_values()
                .reduce(|acc, count| acc * count)
                .expect("There is at least one draw in a game")
        })
        .sum()
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
