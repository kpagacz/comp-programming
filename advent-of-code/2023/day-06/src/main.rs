use std::cmp::Ordering;

fn parse_race_records(input: &str) -> (Vec<usize>, Vec<usize>) {
    let lines: Vec<_> = input.lines().collect();
    let times: Vec<_> = lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();
    let distances: Vec<_> = lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();
    (times, distances)
}

fn part1(input: &str) -> usize {
    let (times, records) = parse_race_records(input);

    times
        .into_iter()
        .zip(records)
        .map(|(time_limit, record_distance)| {
            (0..time_limit)
                .filter(|time_spent_pressing_the_button| {
                    time_spent_pressing_the_button * (time_limit - time_spent_pressing_the_button)
                        > record_distance
                })
                .count()
        })
        .reduce(|acc, ways_to_beat_a_record| acc * ways_to_beat_a_record)
        .unwrap()
}

fn part2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let time: Vec<_> = lines[0].split_ascii_whitespace().skip(1).collect();
    let time = time.join("").parse::<usize>().unwrap();

    let record: Vec<_> = lines[1].split_ascii_whitespace().skip(1).collect();
    let record = record.join("").parse::<usize>().unwrap();

    let middle_time = time / 2;

    fn does_beat_record(
        time_spent_pressing_the_button: usize,
        record: usize,
        time_limit: usize,
    ) -> bool {
        time_spent_pressing_the_button * (time_limit - time_spent_pressing_the_button) > record
    }

    let left_half: Vec<_> = (0..middle_time).collect();
    let first_time_that_beats_record = left_half
        .binary_search_by(|&probe| {
            if does_beat_record(probe, record, time) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .unwrap_err();

    let right_half: Vec<_> = (middle_time..time).collect();
    let first_time_that_does_not_beat_the_record = right_half
        .binary_search_by(|&probe| {
            if does_beat_record(probe, record, time) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .unwrap_err();

    right_half[first_time_that_does_not_beat_the_record] - left_half[first_time_that_beats_record]
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
