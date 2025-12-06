fn parse_input(input: &str) -> Vec<(Vec<usize>, u8)> {
    let mut lines_it = input.lines().rev();
    let ops: Vec<_> = lines_it
        .next()
        .unwrap()
        .split_whitespace()
        .map(|op| op.as_bytes()[0])
        .collect();
    let nums: Vec<Vec<_>> = lines_it
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (0..ops.len())
        .map(|problem_id| {
            let mut numbers = vec![];
            for nums in &nums {
                numbers.push(nums[problem_id]);
            }
            (numbers, ops[problem_id])
        })
        .collect()
}

fn pad_lines_to_max_length(lines: Vec<&str>) -> Vec<String> {
    let max_length = lines.iter().map(|line| line.len()).max().unwrap();
    lines
        .iter()
        .map(|line| {
            line.chars()
                .chain(std::iter::repeat(' '))
                .take(max_length)
                .collect::<String>()
        })
        .collect()
}

fn parse_columnar_math(input: &str) -> Vec<(Vec<usize>, u8)> {
    let lines: Vec<_> = input.lines().collect();
    let padded_lines = pad_lines_to_max_length(lines);
    let mut padded_lines = padded_lines.iter().rev();
    let ops_line = padded_lines.next().unwrap().as_bytes();
    let other_lines: Vec<_> = padded_lines.rev().collect();
    let mut sign_pos = 0usize;
    let mut problems = vec![];
    while sign_pos < ops_line.len() {
        // Find next sign
        let next_sign = sign_pos
            + 1
            + ops_line[sign_pos + 1..]
                .iter()
                .position(|c| !c.is_ascii_whitespace())
                .unwrap_or(ops_line.len() - sign_pos);
        let after_digit = next_sign - 1; // because there's one column of whitespace

        let mut numbers = vec![];

        // Cut out numbers
        let rows: Vec<_> = other_lines
            .iter()
            .map(|&line| line.chars().collect::<Vec<_>>()[sign_pos..after_digit].to_owned())
            .collect();

        // Transpose
        let mut columns = vec![vec![' '; rows.len()]; rows[0].len()];
        for i in 0..rows.len() {
            for j in 0..rows[0].len() {
                columns[j][i] = rows[i][j];
            }
        }

        // Parse lines as numbers and add
        columns
            .into_iter()
            .map(|num| {
                num.into_iter()
                    .collect::<String>()
                    .trim()
                    .parse::<usize>()
                    .unwrap()
            })
            .for_each(|num| numbers.push(num));

        problems.push((numbers, ops_line[sign_pos]));
        sign_pos = next_sign;
    }
    problems
}

fn solve_problem(nums: &[usize], op: u8) -> usize {
    nums.iter()
        .copied()
        .reduce(|acc, el| match op {
            b'+' => acc + el,
            b'*' => acc * el,
            _ => unreachable!("No other operators are defined for cephalod math"),
        })
        .expect("Problem has at least one num")
}

fn part1(input: &str) -> usize {
    let problems = parse_input(input);
    problems
        .into_iter()
        .map(|problem| solve_problem(&problem.0, problem.1))
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let problems = parse_columnar_math(input);
    problems
        .into_iter()
        .map(|problem| solve_problem(&problem.0, problem.1))
        .sum::<usize>()
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");
    println!("Part 1 (test): {}  exp: 4277556", part1(test));
    println!("Part 1: {}", part1(input));
    println!("Part 2 (test): {}  exp: 3263827", part2(test));
    println!("Part 2: {}", part2(input));
}
