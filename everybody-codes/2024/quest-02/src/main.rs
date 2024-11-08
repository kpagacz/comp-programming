fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    lines.next();

    (
        first_line
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(str::to_string)
            .collect::<Vec<_>>(),
        lines.map(str::to_string).collect(),
    )
}

fn count_ocurrances(s: &str, pattern: &str) -> usize {
    let s = format!("{pattern}${s}");
    let s = s.as_bytes();
    let mut dp = vec![0usize; s.len()];

    for i in 1..s.len() {
        let mut previous = dp[i - 1];

        while s[i] != s[previous] && previous != 0 {
            previous = dp[previous - 1];
        }

        if s[i] == s[previous] {
            dp[i] = previous + 1;
        }
    }

    dp.iter()
        .filter(|&&matching| matching == pattern.len())
        .count()
}

fn part1(input: &str) -> usize {
    let (runic_words, inscriptions) = parse_input(input);

    runic_words
        .iter()
        .map(|word| count_ocurrances(&inscriptions[0], word))
        .sum()
}

fn mark_runic_symbols(inscription: &str, runic_words: &[String]) -> Vec<bool> {
    let mut answer = vec![false; inscription.len()];

    for i in 0..inscription.len() {
        for word in runic_words {
            if inscription[i..].starts_with(word) {
                answer[i..i + word.len()].copy_from_slice(&vec![true; word.len()]);
            }
        }
    }

    answer
}

fn part2(input: &str) -> usize {
    let (runic_words, inscriptions) = parse_input(input);

    inscriptions
        .iter()
        .map(|inscription| {
            let inscription_backwards = inscription.chars().rev().collect::<String>();
            let mut runic_symbols_used = vec![false; inscription.len()];
            let forward = mark_runic_symbols(inscription, &runic_words);
            for i in 0..runic_symbols_used.len() {
                runic_symbols_used[i] = forward[i] || runic_symbols_used[i];
            }
            let backward = mark_runic_symbols(&inscription_backwards, &runic_words)
                .into_iter()
                .rev()
                .collect::<Vec<_>>();
            for i in 0..runic_symbols_used.len() {
                runic_symbols_used[i] = backward[i] || runic_symbols_used[i];
            }

            runic_symbols_used.into_iter().filter(|used| *used).count()
        })
        .sum()
}

fn mark_runic_symbols_wrapping(inscription: &str, runic_words: &[String]) -> Vec<bool> {
    let inscription = format!("{inscription}{inscription}");
    let inscription = inscription.as_str();
    let mut answer = vec![false; inscription.len()];

    for i in 0..inscription.len() / 2 {
        for word in runic_words {
            if inscription[i..].starts_with(word) {
                for j in i..i + word.len() {
                    let m = answer.len() / 2;
                    // force the wrapping behaviour
                    answer[j % m] = true;
                }
            }
        }
    }
    let to_take = answer.len() / 2;
    answer.into_iter().take(to_take).collect()
}

fn part3(input: &str) -> usize {
    let (runic_words, inscriptions) = parse_input(input);

    let mut answer = vec![vec![false; inscriptions[0].len()]; inscriptions.len()];

    // Do rows
    for i in 0..inscriptions.len() {
        let inscription = &inscriptions[i];
        let inscription_backwards = inscription.chars().rev().collect::<String>();
        let mut runic_symbols_used = vec![false; inscription.len()];
        let forward = mark_runic_symbols_wrapping(inscription, &runic_words);
        for i in 0..runic_symbols_used.len() {
            runic_symbols_used[i] = forward[i] || runic_symbols_used[i];
        }
        let backward = mark_runic_symbols_wrapping(&inscription_backwards, &runic_words)
            .into_iter()
            .rev()
            .collect::<Vec<_>>();
        for i in 0..runic_symbols_used.len() {
            runic_symbols_used[i] = backward[i] || runic_symbols_used[i];
        }
        answer[i][..].copy_from_slice(&runic_symbols_used);
    }

    // Do columns
    for col in 0..inscriptions[0].len() {
        let inscription = (0..inscriptions.len())
            .map(|row| inscriptions[row].chars().nth(col).unwrap())
            .collect::<String>();
        let inscription = &inscription;
        let inscription_backwards = inscription.chars().rev().collect::<String>();
        let mut runic_symbols_used = vec![false; inscription.len()];
        let forward = mark_runic_symbols(inscription, &runic_words);
        for i in 0..runic_symbols_used.len() {
            runic_symbols_used[i] = forward[i] || runic_symbols_used[i];
        }
        let backward = mark_runic_symbols(&inscription_backwards, &runic_words)
            .into_iter()
            .rev()
            .collect::<Vec<_>>();
        for i in 0..runic_symbols_used.len() {
            runic_symbols_used[i] = backward[i] || runic_symbols_used[i];
        }

        for row in 0..answer.len() {
            answer[row][col] = answer[row][col] || runic_symbols_used[row];
        }
    }

    answer
        .into_iter()
        .map(|row| row.into_iter().filter(|used| *used).count())
        .sum()
}

fn main() {
    let input = include_str!("../input1");
    println!("Part 1: {}", part1(input));
    let input2 = include_str!("../input2");
    println!("Part 2: {}", part2(input2));
    let input3 = include_str!("../input3");
    println!("Part 3: {}", part3(input3));
}
