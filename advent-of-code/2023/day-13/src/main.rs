type Pattern = Vec<Vec<u8>>;
fn parse_input(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.as_bytes().to_vec())
                .collect()
        })
        .collect()
}

fn transpose(pattern: Pattern) -> Pattern {
    let mut answer = vec![vec![b' '; pattern.len()]; pattern[0].len()];

    for i in 0..answer.len() {
        for j in 0..answer[0].len() {
            answer[i][j] = pattern[j][i];
        }
    }

    answer
}

fn calculate_reflection_column(pattern: &Pattern) -> usize {
    for col in 1..pattern[0].len() {
        let left_size = col;
        let right_size = pattern[0].len() - left_size;
        let reflected_size = left_size.min(right_size);

        if pattern.iter().all(|line| {
            let left = line[..left_size].iter().rev().take(reflected_size);
            let right = &line[line.len() - right_size..][..reflected_size];
            left.zip(right.iter()).all(|(left, right)| left == right)
        }) {
            return col;
        }
    }
    0
}

fn calculate_reflection_column2(pattern: &Pattern, original: usize) -> usize {
    for col in 1..pattern[0].len() {
        let left_size = col;
        let right_size = pattern[0].len() - left_size;
        let reflected_size = left_size.min(right_size);

        if pattern.iter().all(|line| {
            let left = line[..left_size].iter().rev().take(reflected_size);
            let right = &line[line.len() - right_size..][..reflected_size];
            left.zip(right.iter()).all(|(left, right)| left == right)
        }) {
            if col == original {
                continue;
            } else {
                return col;
            }
        }
    }
    0
}

fn part1(input: &str) -> usize {
    let patterns = parse_input(input);
    let mut answer = 0;
    for pattern in patterns {
        let reflection_col = calculate_reflection_column(&pattern);
        answer += reflection_col;

        let transposed = transpose(pattern);
        let reflection_row = calculate_reflection_column(&transposed);
        answer += 100 * reflection_row;
    }

    answer
}

fn part2(input: &str) -> usize {
    let mut patterns = parse_input(input);
    let mut answer = 0;

    for mut pattern in patterns {
        let original_reflection_col = calculate_reflection_column(&pattern);
        let transposed = transpose(pattern.clone());
        let original_reflection_row = calculate_reflection_column(&transposed);

        'outer: for i in 0..pattern.len() {
            for j in 0..pattern[0].len() {
                // Swap the char
                if pattern[i][j] == b'.' {
                    pattern[i][j] = b'#';
                } else {
                    pattern[i][j] = b'.';
                }

                // Try to find the reflection
                let reflection_col =
                    calculate_reflection_column2(&pattern, original_reflection_col);
                if reflection_col != 0 {
                    answer += reflection_col;
                    break 'outer;
                }

                pattern = transpose(pattern);
                let reflection_row =
                    calculate_reflection_column2(&pattern, original_reflection_row);
                if reflection_row != 0 {
                    answer += 100 * reflection_row;
                    break 'outer;
                }

                // Cleanup
                pattern = transpose(pattern);
                if pattern[i][j] == b'.' {
                    pattern[i][j] = b'#';
                } else {
                    pattern[i][j] = b'.';
                }
            }
        }
    }

    answer
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
