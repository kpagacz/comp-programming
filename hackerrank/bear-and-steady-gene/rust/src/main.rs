// https://www.hackerrank.com/challenges/bear-and-steady-gene/problem

use std::collections::btree_map::IntoValues;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'steadyGene' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING gene as parameter.
 */

fn steadyGene(gene: &str) -> i32 {
    let mut frequencies = HashMap::new();

    for c in gene.chars() {
        frequencies
            .entry(c)
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
    }

    if canBeSteadied(0, &frequencies) {
        return 0;
    }
    let gene: Vec<char> = gene.chars().collect();

    let mut minimum_length = i32::MAX;

    let mut left = 0_usize;
    let mut right = 0_usize;

    // println!("{gene:?}");
    // println!("{frequencies:?}");
    while right < gene.len() {
        let char = gene[right];
        *frequencies.get_mut(&char).unwrap() -= 1;
        right += 1;

        // println!("{frequencies:?}");
        while canBeSteadied((right - left) as i32, &frequencies) && left < right {
            minimum_length = minimum_length.min((right - left) as i32);

            *frequencies.get_mut(&gene[left]).unwrap() += 1;
            left += 1;
        }
    }

    if canBeSteadied((right - left) as i32, &frequencies) {
        minimum_length = minimum_length.min((right - left + 1) as i32);
    }

    minimum_length
}

fn canBeSteadied(availableChars: i32, frequencies: &HashMap<char, i32>) -> bool {
    let sum: i32 = frequencies.values().sum::<i32>() + availableChars;
    if sum % 4 != 0 {
        return false;
    }
    let max_allowed_freq = sum / 4;
    if frequencies.values().any(|freq| *freq > max_allowed_freq) {
        return false;
    }

    let differences: i32 = frequencies
        .values()
        .map(|freq| max_allowed_freq - freq)
        .sum();

    differences <= availableChars
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let gene = stdin_iterator.next().unwrap().unwrap();
    let result = steadyGene(&gene);

    writeln!(&mut fptr, "{}", result).ok();
}
