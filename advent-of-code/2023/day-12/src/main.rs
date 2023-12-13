fn parse_input(input: &str) -> Vec<(Vec<u8>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (chars, nums) = line.split_once(' ').unwrap();
            (
                chars.as_bytes().to_owned(),
                nums.split(',')
                    .map(str::parse::<usize>)
                    .map(Result::unwrap)
                    .collect(),
            )
        })
        .collect()
}

fn verify_line(line: &[u8], groupings: &[usize]) -> bool {
    line.split(|c| c == &b'.')
        .filter_map(|grouping| {
            if !grouping.is_empty() {
                Some(grouping.len())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        == groupings
}

fn solve(
    line: &mut [u8],
    current_position: usize,
    springs_left: usize,
    groupings: &[usize],
) -> usize {
    if current_position >= line.len() {
        if verify_line(line, groupings) {
            return 1;
        } else {
            return 0;
        }
    }

    if line[current_position] != b'?' {
        let ans = solve(line, current_position + 1, springs_left, groupings);
        return ans;
    }

    line[current_position] = b'.';
    let mut answer = solve(line, current_position + 1, springs_left, groupings);

    if springs_left > 0 {
        line[current_position] = b'#';
        answer += solve(line, current_position + 1, springs_left - 1, groupings);
    }
    line[current_position] = b'?';
    answer
}

fn custom_print(text: String, depth: i32) {
    for _ in 0..depth {
        print!(" ");
    }
    println!("{text}");
}

fn solve2(
    line: &mut [u8],
    mut current_position: usize,
    mut current_grouping: usize,
    mut springs_so_far: usize,
    groupings: &[usize],
    depth: i32,
) -> usize {
    if current_grouping > groupings.len() {
        // println!("current grouping too large");
        return 0;
    }
    while current_position < line.len() && line[current_position] != b'?' {
        let current_char = line[current_position];
        match current_char {
            b'.' => {
                println!("Found dot at: {current_position}");
                if current_grouping != groupings.len() {
                    if springs_so_far > 0 && springs_so_far < groupings[current_grouping] {
                        println!("current grouping != groupings.len && springs_so_far > 0 && springs_so_far < groupiongs[current_grouping]");
                        return 0;
                    }
                    // The three lines below make test3 fail
                    if springs_so_far == groupings[current_grouping] {
                        current_grouping += 1;
                    }
                }
                springs_so_far = 0;

                // The below works for test3 but fails test2
                // println!("Found dot at: {current_position}");
                // if current_grouping != groupings.len() {
                //     if springs_so_far > 0 && springs_so_far < groupings[current_grouping] {
                //         println!("current grouping != groupings.len && springs_so_far > 0 && springs_so_far < groupiongs[current_grouping]");
                //         return 0;
                //     }
                // }
                // springs_so_far = 0;

                // if current_grouping < groupings.len()
                //     && springs_so_far < groupings[current_grouping]
                // {
                //     return 0;
                // } else if current_grouping == groupings.len() && springs_so_far > 0 {
                //     return 0;
                // } else if current_grouping < groupings.len()
                //     && springs_so_far == groupings[current_grouping]
                // {
                //     current_grouping += 1;
                // }
                //
                // springs_so_far = 0;
            }
            b'#' => {
                println!("Got static # at: {current_position}");
                springs_so_far += 1;
                println!("Spring so far (including this one) {springs_so_far}");
                if current_grouping >= groupings.len()
                    || springs_so_far > groupings[current_grouping]
                {
                    println!("current_groupoing >= groupings.len() or springs_so_far > groupings[current_grouping]");
                    return 0;
                }
            }
            _ => {
                unreachable!()
            }
        }
        current_position += 1;
    }

    if current_position == line.len() {
        if current_grouping == groupings.len() {
            println!("Returning 1");
            return 1;
        } else {
            println!("Returning 0");
            return 0;
        }
    }

    if current_grouping == groupings.len() {
        println!("No groupings left, just add dot");
        return solve2(line, current_position + 1, current_grouping, 0, groupings);
    }

    if springs_so_far == 0 {
        println!("no previous springs, so add dot or # at: {current_position}");
        return solve2(line, current_position + 1, current_grouping, 0, groupings)
            + solve2(line, current_position + 1, current_grouping, 1, groupings);
    }

    if springs_so_far < groupings[current_grouping] {
        println!("have to continue # at: {current_position}");
        return solve2(
            line,
            current_position + 1,
            current_grouping,
            springs_so_far + 1,
            groupings,
        );
    }

    if springs_so_far == groupings[current_grouping] {
        println!("Add dot because the limit was achieved at {current_position}");
        return solve2(
            line,
            current_position + 1,
            current_grouping + 1,
            0,
            groupings,
        );
    }

    if springs_so_far > groupings[current_grouping] {
        println!("too many springs for the grouping at {current_position}. returning 0");
        return 0;
    }

    unreachable!()
}

fn count_springs_left(line: &[u8], groupings: &[usize]) -> usize {
    let total_springs: usize = groupings.iter().sum();
    let already_inserted = line.iter().filter(|&&c| c == b'#').count();
    total_springs - already_inserted
}

fn part1(input: &str) -> usize {
    let springs_and_groupings = parse_input(input);

    springs_and_groupings
        .into_iter()
        .map(|(mut line, groupings)| {
            let springs_left = count_springs_left(&line, &groupings);
            solve(&mut line, 0, springs_left, &groupings)
        })
        .sum()
}

fn multiply_by_five(line_and_grouping: (Vec<u8>, Vec<usize>)) -> (Vec<u8>, Vec<usize>) {
    let multiplier = 2;
    let (line, groupings) = (line_and_grouping.0, line_and_grouping.1);
    let line_n = line.len();
    let groupings_n = groupings.len();
    (
        (line.into_iter().chain(std::iter::once(b'?')))
            .cycle()
            .take((line_n + 1) * multiplier)
            .collect(),
        groupings
            .into_iter()
            .cycle()
            .take(groupings_n * multiplier)
            .collect(),
    )
}

fn part2(input: &str) -> usize {
    let springs_and_groupings = parse_input(input);

    springs_and_groupings
        .into_iter()
        .map(multiply_by_five)
        .map(|(mut line, groupings)| {
            line.push(b'.');
            println!("{} {groupings:?}", String::from_utf8(line.clone()).unwrap());
            let answer = solve2(&mut line, 0, 0, 0, &groupings, 0);
            println!("{answer}");
            answer
        })
        .sum()
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));
    // println!("Part 2: {}", part2(test));

    println!("Another part 2 test: should return 1");
    let test = include_str!("../test2");
    println!("Part 2: {}", part2(test));

    println!("Another part 2 test: should return 506250");
    let test = include_str!("../test3");
    println!("Part 2: {}", part2(test));

    println!("INPUT");
    let input = include_str!("../input");
    // println!("Part 1: {}", part1(input));
    // println!("Part 2: {}", part2(input));
}
