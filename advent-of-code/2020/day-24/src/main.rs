use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::many0, IResult};

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    fn coordinates_parser(input: &str) -> IResult<&str, (i32, i32)> {
        map(
            many0(alt((
                map(tag("ne"), |_| (1, 0)),
                map(tag("sw"), |_| (-1, 0)),
                map(tag("e"), |_| (0, 1)),
                map(tag("w"), |_| (0, -1)),
                map(tag("se"), |_| (-1, 1)),
                map(tag("nw"), |_| (1, -1)),
            ))),
            |coords| {
                coords
                    .into_iter()
                    .fold((0, 0), |acc, dir| (acc.0 + dir.0, acc.1 + dir.1))
            },
        )(input)
    }
    input
        .lines()
        .map(|line| coordinates_parser(line).unwrap().1)
        .collect()
}

fn part1(coords: Vec<(i32, i32)>) {
    use std::collections::HashMap;
    let black_count = coords
        .into_iter()
        .fold(HashMap::new(), |mut map, coord| {
            *map.entry(coord).or_insert(0) += 1;
            map
        })
        .values()
        .filter(|&&count| count % 2 == 1)
        .count();
    println!("Part 1. Black tiles: {black_count}");
}

fn part2(coords: Vec<(i32, i32)>) {
    use std::collections::HashMap;
    use std::collections::HashSet;
    let mut black_tiles: HashSet<(i32, i32)> = coords
        .into_iter()
        .fold(HashMap::new(), |mut map, coord| {
            *map.entry(coord).or_insert(0) += 1;
            map
        })
        .into_iter()
        .filter(|(_, count)| count % 2 == 1)
        .map(|(key, _)| key)
        .collect();

    fn count_black_neighbours(tile: &(i32, i32), black_tiles: &HashSet<(i32, i32)>) -> i32 {
        let neighbour_delta = [(0, 1), (0, -1), (1, 0), (-1, 0), (-1, 1), (1, -1)];
        neighbour_delta
            .into_iter()
            .map(|delta| (tile.0 + delta.0, tile.1 + delta.1))
            .map(|neighbour| {
                if black_tiles.contains(&neighbour) {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
    fn remove_black_tiles(black_tiles: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
        black_tiles
            .iter()
            .filter_map(|black_tile| {
                if count_black_neighbours(black_tile, &black_tiles) == 0
                    || count_black_neighbours(black_tile, &black_tiles) > 2
                {
                    None
                } else {
                    Some(*black_tile)
                }
            })
            .collect()
    }

    fn to_add(black_tiles: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
        let neighbour_delta = [(0, 1), (0, -1), (1, 0), (-1, 0), (-1, 1), (1, -1)];
        black_tiles
            .iter()
            .fold(HashMap::new(), |mut map, &black_tile| {
                neighbour_delta
                    .iter()
                    .map(|&delta| (black_tile.0 + delta.0, black_tile.1 + delta.1))
                    .filter(|neighbour| !black_tiles.contains(neighbour))
                    .for_each(|neighbour| *map.entry(neighbour).or_insert(0) += 1);
                map
            })
            .into_iter()
            .filter(|(_, value)| *value == 2)
            .map(|(key, _)| key)
            .collect()
    }

    let days = 100;
    for i in 0..days {
        let to_add = to_add(&black_tiles);
        black_tiles = remove_black_tiles(black_tiles);
        black_tiles.extend(to_add);
        // println!("Day {}: {}", i + 1, black_tiles.len());
    }

    println!("Part 2. Black tiles: {}", black_tiles.len());
}

fn main() {
    println!("TEST");
    let test_input = include_str!("../test");
    let parsed_input = parse_input(test_input);
    part1(parsed_input.clone());
    part2(parsed_input);

    println!("INPUT");
    let input = include_str!("../input");
    let parsed_input = parse_input(input);
    part1(parsed_input.clone());
    part2(parsed_input);
}
