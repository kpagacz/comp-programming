fn part1(input: &str) -> i32 {
    input.chars().map(monster_to_potion).sum()
}

fn monster_to_potion(c: char) -> i32 {
    match c {
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => 0,
    }
}
fn part2(input: &str) -> i32 {
    let bytes = input.as_bytes();

    bytes
        .chunks(2)
        .map(|pairs| {
            if pairs.len() == 2 {
                match (pairs[0], pairs[1]) {
                    (b'x', monster) | (monster, b'x') => monster_to_potion(monster as char),
                    (first, second) => {
                        monster_to_potion(first as char) + monster_to_potion(second as char) + 2
                    }
                }
            } else {
                monster_to_potion(pairs[0] as char)
            }
        })
        .sum()
}

fn part3(input: &str) -> i32 {
    let bytes = input.as_bytes();

    bytes
        .chunks(3)
        .map(|threes| {
            let xs = threes.iter().filter(|&&c| c == b'x').count();
            let monsters = threes.len() - xs;

            let potions = threes
                .iter()
                .map(|&monster| monster_to_potion(monster as char))
                .sum::<i32>();

            potions + ((monsters.saturating_sub(1)) * monsters) as i32
        })
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
