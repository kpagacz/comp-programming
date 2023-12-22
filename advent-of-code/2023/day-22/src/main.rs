type Brick = ((usize, usize, usize), (usize, usize, usize));
fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once('~').unwrap();
            let first = first
                .split(',')
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            let second = second
                .split(',')
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            (
                (first[0], first[1], first[2]),
                (second[0], second[1], second[2]),
            )
        })
        .collect()
}

// pov[x][y] contains the vector of z coords of the fallen bricks
// bricks is sorted by z so they are in order of falling (from the first to the last)
fn fall_bricks(bricks: &[Brick]) -> Vec<Vec<Vec<(usize, usize, usize)>>> {
    let mut answer = vec![vec![vec![(0, 0, usize::MAX)]; 10]; 10]; // max x,y coord in the input is 9
    for (brick_no, brick) in bricks.iter().enumerate() {
        let mut highest_z = usize::MIN;
        for x in brick.0 .0..=brick.1 .0 {
            for y in brick.0 .1..=brick.1 .1 {
                highest_z = highest_z.max(answer[x][y].last().unwrap().1);
            }
        }

        for x in brick.0 .0..=brick.1 .0 {
            for y in brick.0 .1..=brick.1 .1 {
                answer[x][y].push((
                    highest_z + 1,
                    highest_z + 1 + brick.1 .2 - brick.0 .2,
                    brick_no,
                ));
            }
        }
    }
    answer
}

type LieVec = Vec<Vec<usize>>;
type SupportsVec = Vec<Vec<usize>>;
fn find_dependencies(
    z_pov: &[Vec<Vec<(usize, usize, usize)>>],
    bricks_count: usize,
) -> (LieVec, SupportsVec) {
    let mut lies_on = vec![Vec::<usize>::new(); bricks_count];
    let mut supports = vec![Vec::<usize>::new(); bricks_count];

    for x in 0..z_pov.len() {
        for y in 0..z_pov[0].len() {
            z_pov[x][y].windows(2).for_each(|window| {
                let ((_, first_y2, first), (second_y1, _, second)) = (window[0], window[1]);
                if second_y1 - first_y2 == 1 && first != usize::MAX {
                    lies_on[second].push(first);
                    supports[first].push(second);
                }
            });
        }
    }
    lies_on.iter_mut().for_each(|below| {
        below.sort_unstable();
        below.dedup()
    });
    supports.iter_mut().for_each(|below| {
        below.sort_unstable();
        below.dedup()
    });

    (lies_on, supports)
}

fn can_be_disintegrated(brick_no: usize, lies_on: &LieVec, supports: &SupportsVec) -> bool {
    supports[brick_no]
        .iter()
        .all(|&supported_brick| lies_on[supported_brick].len() > 1)
}

fn part1(input: &str) -> usize {
    let mut bricks = parse_input(input);
    bricks.sort_unstable_by_key(|brick| brick.0 .2);
    let settled_z_pov = fall_bricks(&bricks);

    let (lies_on, supports) = find_dependencies(&settled_z_pov, bricks.len());
    (0..bricks.len())
        .filter(|&brick_no| can_be_disintegrated(brick_no, &lies_on, &supports))
        .count()
}

fn count_other_fallen(brick_no: usize, below: &LieVec, supports: &SupportsVec) -> usize {
    // println!("Started from {brick_no}");
    use std::collections::BTreeSet;
    let mut already_fallen = BTreeSet::from([brick_no]);
    let mut not_yet_fallen = supports[brick_no].iter().cloned().collect::<BTreeSet<_>>();
    let mut previous_not_yet_fallen = BTreeSet::new();

    while not_yet_fallen.difference(&previous_not_yet_fallen).count() != 0 {
        previous_not_yet_fallen = not_yet_fallen.clone();
        let mut new_not_yet_fallen = vec![];
        let mut to_remove_from_not_yet_fallen = vec![];
        for &brick in &not_yet_fallen {
            // println!("Considering brick: {brick}");
            if below[brick]
                .iter()
                .all(|brick_below| already_fallen.contains(brick_below))
            {
                // println!("Will fall");
                already_fallen.insert(brick);
                to_remove_from_not_yet_fallen.push(brick);
                for supported_brick in &supports[brick] {
                    new_not_yet_fallen.push(*supported_brick);
                }
            }
        }
        for brick in new_not_yet_fallen {
            not_yet_fallen.insert(brick);
        }

        for brick in to_remove_from_not_yet_fallen {
            not_yet_fallen.remove(&brick);
        }
    }

    already_fallen.len() - 1
}

fn part2(input: &str) -> usize {
    let mut bricks = parse_input(input);
    bricks.sort_unstable_by_key(|brick| brick.0 .2);
    let settled_z_pov = fall_bricks(&bricks);
    let (below, supports) = find_dependencies(&settled_z_pov, bricks.len());
    // println!("{below:?}");
    // println!("{supports:?}");

    (0..bricks.len())
        .map(|brick_no| count_other_fallen(brick_no, &below, &supports))
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
