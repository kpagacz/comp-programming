fn parse_input(input: &str) -> Vec<(&str, Vec<&str>)> {
    input
        .lines()
        .map(|line| {
            let (track_name, actions) = line.split_once(":").unwrap();
            (track_name, actions.split(",").collect())
        })
        .collect()
}

fn parse_racetrack(input: &str) -> Vec<char> {
    let mut racetrack = vec![];
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (mut x, mut y) = (0usize, 1usize);
    let (mut prev_x, mut prev_y) = (0usize, 0usize);

    'outer: while x != 0 || y != 0 {
        racetrack.push(map[x][y]);
        for new_x in [x.saturating_sub(1), x + 1] {
            if new_x != x
                && (new_x != prev_x || y != prev_y)
                && new_x < map.len()
                && map[new_x][y] != ' '
            {
                prev_x = x;
                prev_y = y;
                x = new_x;
                continue 'outer;
            }
        }
        for new_y in [y.saturating_sub(1), y + 1] {
            if new_y != y
                && (x != prev_x || new_y != prev_y)
                && new_y < map[0].len()
                && map[x][new_y] != ' '
            {
                prev_x = x;
                prev_y = y;
                y = new_y;
                continue 'outer;
            }
        }
    }
    racetrack.push('=');
    racetrack
}

fn rate_track(track: &[&str], start: usize, race_duration: usize) -> usize {
    track
        .iter()
        .cycle()
        .take(race_duration)
        .fold((0usize, start), |acc, &action| {
            let (acc, mut current_power) = acc;
            match action {
                "+" => current_power += 1,
                "-" => current_power = current_power.saturating_sub(1),
                _ => {}
            }

            (acc + current_power, current_power)
        })
        .0
}

fn part1(input: &str) -> String {
    use std::cmp::Reverse;
    let mut tracks = parse_input(input);
    tracks.sort_unstable_by_key(|(_, track)| Reverse(rate_track(track, 10, 10)));
    tracks.into_iter().map(|(name, _)| name).collect::<String>()
}

fn rate_racetrack_with_device(racetrack: &[char], device: &[&str], loops: usize) -> usize {
    let track_length = racetrack.len();

    racetrack
        .iter()
        .cycle()
        .zip(
            device
                .iter()
                .map(|&action| action.chars().next().unwrap())
                .cycle(),
        )
        .take(loops * track_length)
        .fold((0usize, 10usize), |acc, (&track, device)| {
            let (acc, mut current_power) = acc;
            match track {
                '=' => match device {
                    '-' => current_power = current_power.saturating_sub(1),
                    '+' => current_power += 1,
                    _ => {}
                },
                '-' => current_power = current_power.saturating_sub(1),
                '+' => current_power += 1,
                _ => unreachable!(),
            }
            (acc + current_power, current_power)
        })
        .0
}

fn part2(input: &str, racetrack: &str) -> String {
    let racetrack = parse_racetrack(racetrack);
    let mut device_behaviour = parse_input(input);

    use std::cmp::Reverse;
    device_behaviour.sort_unstable_by_key(|(_, device)| {
        Reverse(rate_racetrack_with_device(&racetrack, device, 10))
    });
    device_behaviour
        .into_iter()
        .map(|(name, _)| name)
        .collect::<String>()
}

fn rate_racetrack_with_device_for_2024_loops(action_plan: &[&str], racetrack: &[char]) -> usize {
    let eleven_loops = rate_racetrack_with_device(racetrack, action_plan, 11);
    // 184 * 11 = 2024
    eleven_loops * 184
}

fn part3(opponent: &str, racetrack: &str) -> usize {
    let racetrack = parse_racetrack(racetrack);
    println!("Length of racetrack {}", racetrack.len());

    const PLUSES: usize = 5;
    const MINUSES: usize = 3;
    const EQUALS: usize = 3;
    let mut candidate_action_plan: Vec<_> = std::iter::repeat("+")
        .take(PLUSES)
        .chain(std::iter::repeat("-").take(MINUSES))
        .chain(std::iter::repeat("=").take(EQUALS))
        .collect();
    // Heap's permutation generation
    fn heaps(
        k: usize,
        original_action_plan: &mut [&str],
        racetrack: &[char],
        count: &mut usize,
        opponents_result: usize,
    ) {
        if k == 1 {
            let score = rate_racetrack_with_device_for_2024_loops(original_action_plan, racetrack);
            if score > opponents_result {
                *count += 1;
            }
            if *count > 1 && *count % 1_000_000 == 0 {
                println!("Got {count} solutions.");
                println!("plan is: {original_action_plan:?}");
            }
        } else {
            for i in 0..k {
                heaps(
                    k - 1,
                    original_action_plan,
                    racetrack,
                    count,
                    opponents_result,
                );
                if k % 2 == 0 {
                    original_action_plan.swap(i, k - 1);
                } else {
                    original_action_plan.swap(0, k - 1);
                }
            }
        }
    }

    let mut count = 0usize;
    let parsed_opp = &parse_input(opponent)[0].1;
    let opponents_score = rate_racetrack_with_device_for_2024_loops(parsed_opp, &racetrack);
    heaps(
        candidate_action_plan.len(),
        &mut candidate_action_plan,
        &racetrack,
        &mut count,
        opponents_score,
    );
    count / 5 / 4 / 3 / 2 / 3 / 2 / 3 / 2
}

fn main() {
    let input1 = include_str!("../1-input");
    let test1 = include_str!("../1-test");
    println!("Part 1 (test): {}", part1(test1));
    println!("Part 1: {}", part1(input1));

    let input2 = include_str!("../2-input");
    let racetrack2 = include_str!("../2-racetrack");
    let test2 = include_str!("../2-test");
    let testracetrack2 = include_str!("../2-test-racetrack");
    println!("Part 2 (test): {}", part2(test2, testracetrack2));
    println!("Part 2: {}", part2(input2, racetrack2));

    let input3 = include_str!("../3-opponent");
    let racetrack3 = include_str!("../3-racetrack");
    println!("Part 3: {}", part3(input3, racetrack3));
}
