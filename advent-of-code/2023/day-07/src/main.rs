#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<u8>,
    bet: usize,
}

impl Hand {
    fn new(hand: Vec<u8>, bet: usize) -> Self {
        Self { cards: hand, bet }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::collections::BTreeMap;
        let ordering: BTreeMap<u8, i32> = BTreeMap::from([
            (b'2', 1),
            (b'3', 2),
            (b'4', 3),
            (b'5', 4),
            (b'6', 5),
            (b'7', 6),
            (b'8', 7),
            (b'9', 8),
            (b'T', 9),
            (b'J', 10),
            (b'Q', 11),
            (b'K', 12),
            (b'A', 13),
        ]);

        fn count_cards(cards: &[u8]) -> BTreeMap<&u8, usize> {
            cards.iter().fold(BTreeMap::new(), |mut map, card| {
                *map.entry(card).or_insert(0) += 1;
                map
            })
        }

        let my_hand = count_cards(&self.cards);
        let other_hand = count_cards(&other.cards);

        other_hand.len().cmp(&my_hand.len()).then_with(|| {
            let has_four_of_a_kind = my_hand.values().any(|count| count == &4);
            let other_has_four_of_a_kind = other_hand.values().any(|count| count == &4);
            match (has_four_of_a_kind, other_has_four_of_a_kind) {
                (true, false) => return std::cmp::Ordering::Greater,
                (false, true) => return std::cmp::Ordering::Less,
                (_, _) => {}
            }

            let has_three_of_a_kind = my_hand.values().any(|count| count == &3);
            let other_has_three_of_a_kind = other_hand.values().any(|count| count == &3);
            match (has_three_of_a_kind, other_has_three_of_a_kind) {
                (true, false) => return std::cmp::Ordering::Greater,
                (false, true) => return std::cmp::Ordering::Less,
                (_, _) => {}
            }

            self.cards
                .iter()
                .zip(&other.cards)
                .find(|(my_card, other_card)| my_card != other_card)
                .and_then(|(my_card, other_card)| {
                    Some(
                        ordering
                            .get(my_card)
                            .unwrap()
                            .cmp(ordering.get(other_card).unwrap()),
                    )
                })
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

fn part1(input: &str) -> usize {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .and_then(|(hand, bet)| {
                    Some(Hand::new(
                        hand.as_bytes().to_owned(),
                        bet.parse::<usize>().unwrap(),
                    ))
                })
                .unwrap()
        })
        .collect();
    hands.sort_unstable();
    hands
        .into_iter()
        .enumerate()
        .fold(0, |sum, (pos, hand)| sum + hand.bet * (pos + 1))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand2 {
    cards: Vec<u8>,
    bet: usize,
}

impl Hand2 {
    fn new(hand: Vec<u8>, bet: usize) -> Self {
        Self { cards: hand, bet }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::collections::BTreeMap;
        let ordering: BTreeMap<u8, i32> = BTreeMap::from([
            (b'J', 0),
            (b'2', 1),
            (b'3', 2),
            (b'4', 3),
            (b'5', 4),
            (b'6', 5),
            (b'7', 6),
            (b'8', 7),
            (b'9', 8),
            (b'T', 9),
            (b'Q', 11),
            (b'K', 12),
            (b'A', 13),
        ]);

        fn count_cards(cards: &[u8]) -> BTreeMap<&u8, usize> {
            cards.iter().fold(BTreeMap::new(), |mut map, card| {
                *map.entry(card).or_insert(0) += 1;
                map
            })
        }

        let mut my_hand = count_cards(&self.cards);
        let mut other_hand = count_cards(&other.cards);

        fn replace_jokers_with_other_cards(mut hand: BTreeMap<&u8, usize>) -> BTreeMap<&u8, usize> {
            let jokers_count = *hand.get(&b'J').unwrap_or(&0);
            hand.remove(&b'J');
            if let Some(max_entry) = hand.iter_mut().max_by_key(|entry| *entry.1) {
                *max_entry.1 += jokers_count;
            } else {
                hand.insert(&b'A', jokers_count);
            }
            hand
        }

        my_hand = replace_jokers_with_other_cards(my_hand);
        other_hand = replace_jokers_with_other_cards(other_hand);

        other_hand.len().cmp(&my_hand.len()).then_with(|| {
            let has_four_of_a_kind = my_hand.values().any(|count| count == &4);
            let other_has_four_of_a_kind = other_hand.values().any(|count| count == &4);
            match (has_four_of_a_kind, other_has_four_of_a_kind) {
                (true, false) => return std::cmp::Ordering::Greater,
                (false, true) => return std::cmp::Ordering::Less,
                (_, _) => {}
            }

            let has_three_of_a_kind = my_hand.values().any(|count| count == &3);
            let other_has_three_of_a_kind = other_hand.values().any(|count| count == &3);
            match (has_three_of_a_kind, other_has_three_of_a_kind) {
                (true, false) => return std::cmp::Ordering::Greater,
                (false, true) => return std::cmp::Ordering::Less,
                (_, _) => {}
            }

            self.cards
                .iter()
                .zip(&other.cards)
                .find(|(my_card, other_card)| my_card != other_card)
                .and_then(|(my_card, other_card)| {
                    Some(
                        ordering
                            .get(my_card)
                            .unwrap()
                            .cmp(ordering.get(other_card).unwrap()),
                    )
                })
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

fn part2(input: &str) -> usize {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .and_then(|(hand, bet)| {
                    Some(Hand2::new(
                        hand.as_bytes().to_owned(),
                        bet.parse::<usize>().unwrap(),
                    ))
                })
                .unwrap()
        })
        .collect();
    hands.sort_unstable();
    hands
        .into_iter()
        .enumerate()
        .fold(0, |sum, (pos, hand)| sum + hand.bet * (pos + 1))
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));
    println!("Part 2: {}", part2(test));

    println!("TEST2");
    let test = include_str!("../test2");
    println!("Part 1: {}", part1(test));
    println!("Part 2: {}", part2(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
