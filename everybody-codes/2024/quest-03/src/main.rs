fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => -1,
                    '#' => 0,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let mut old_ponds = parse_input(input);
    let mut new_ponds = old_ponds.clone();

    let limit = old_ponds.len().max(old_ponds[0].len());
    let rows = old_ponds.len();
    let cols = old_ponds[0].len();
    let mut dug = 0;

    for i in 0..=limit as i32 {
        for row in 0..rows {
            for col in 0..cols {
                if old_ponds[row][col] == 0 {
                    new_ponds[row][col] = 1;
                    dug += 1;
                } else if old_ponds[row][col] == i
                    && old_ponds[row - 1][col] == i
                    && old_ponds[row + 1][col] == i
                    && old_ponds[row][col - 1] == i
                    && old_ponds[row][col + 1] == i
                {
                    new_ponds[row][col] = i + 1;
                    dug += 1;
                }
            }
        }

        old_ponds = new_ponds.clone();
    }

    dug
}

fn part3(input: &str) -> usize {
    let old_ponds = parse_input(input);
    fn add_guards(mut ponds: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        ponds.iter_mut().for_each(|row| {
            row.insert(0, -1);
            row.push(-1)
        });
        let one_row = std::iter::repeat(-1)
            .take(ponds[0].len())
            .collect::<Vec<_>>();
        std::iter::once(one_row.clone())
            .chain(ponds.into_iter())
            .chain(std::iter::once(one_row.clone()))
            .collect()
    }
    let mut old_ponds = add_guards(old_ponds);
    let mut new_ponds = old_ponds.clone();

    let limit = old_ponds.len().max(old_ponds[0].len());
    let rows = old_ponds.len();
    let cols = old_ponds[0].len();
    let mut dug = 0;

    for i in 0..=limit as i32 {
        for row in 0..rows {
            for col in 0..cols {
                if old_ponds[row][col] == 0 {
                    new_ponds[row][col] = 1;
                    dug += 1;
                } else if old_ponds[row][col] == i
                    && old_ponds[row - 1][col] == i
                    && old_ponds[row + 1][col] == i
                    && old_ponds[row][col - 1] == i
                    && old_ponds[row][col + 1] == i
                    && old_ponds[row + 1][col + 1] == i
                    && old_ponds[row + 1][col - 1] == i
                    && old_ponds[row - 1][col - 1] == i
                    && old_ponds[row - 1][col + 1] == i
                {
                    new_ponds[row][col] = i + 1;
                    dug += 1;
                }
            }
        }

        old_ponds = new_ponds.clone();
    }

    dug
}

fn main() {
    let input = include_str!("../input1");
    println!("Part 1: {}", part1(input));
    let input2 = include_str!("../input2");
    println!("Part 2: {}", part1(input2));
    let input3 = include_str!("../input3");
    println!("Part 3: {}", part3(input3));
}
