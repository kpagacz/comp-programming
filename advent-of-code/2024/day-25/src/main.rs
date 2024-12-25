fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let (mut locks, mut keys) = (vec![], vec![]);

    input.split("\n\n").for_each(|schematic| {
        let mut heights = [0, 0, 0, 0, 0];

        let mut lines = schematic.lines();
        let first_line = lines.next().unwrap();
        let is_key = &first_line[..1] != "#";

        schematic.lines().for_each(|line| {
            line.chars().enumerate().for_each(|(pos, c)| {
                if c == '#' {
                    heights[pos] += 1;
                }
            });
        });

        heights.iter_mut().for_each(|height| *height -= 1);
        if is_key {
            keys.push(Key(
                heights[0], heights[1], heights[2], heights[3], heights[4],
            ));
        } else {
            locks.push(Lock(
                heights[0], heights[1], heights[2], heights[3], heights[4],
            ));
        }
    });

    (locks, keys)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Lock(usize, usize, usize, usize, usize);
#[derive(Debug, Clone, Copy)]
struct Key(usize, usize, usize, usize, usize);

fn do_fit(lock: &Lock, key: &Key) -> bool {
    lock.0 + key.0 <= 5
        && lock.1 + key.1 <= 5
        && lock.2 + key.2 <= 5
        && lock.3 + key.3 <= 5
        && lock.4 + key.4 <= 5
}

fn part1(input: &str) -> usize {
    let (locks, keys) = parse_input(input);
    println!("{locks:?}");
    println!("Keys: {keys:?}");

    let mut answer = 0;
    for key in keys {
        for lock in &locks {
            if do_fit(lock, &key) {
                answer += 1;
            }
        }
    }
    answer
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");

    println!("Part 1 (test): {}", part1(test));
    println!("Part 1: {}", part1(input));
}
