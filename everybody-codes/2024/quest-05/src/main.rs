// https://everybody.codes/event/2024/quests/5
use std::collections::VecDeque;

fn parse_input(input: &str) -> Vec<VecDeque<usize>> {
    let mut answer = vec![];
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    first_line
        .split_whitespace()
        .for_each(|num| answer.push(VecDeque::from(vec![num.parse::<usize>().unwrap()])));
    for line in lines {
        let split = line.split_whitespace().collect::<Vec<_>>();
        split.into_iter().enumerate().for_each(|(pos, num)| {
            answer[pos].push_back(num.parse::<usize>().unwrap());
        });
    }
    answer
}

// ========= Part 1
fn run_round(columns: &mut [VecDeque<usize>], to_run: usize) {
    let clapper = columns[to_run]
        .pop_front()
        .expect("There is always at least one clapper");
    let column_to_join = (to_run + 1) % columns.len();
    let column_length = columns[column_to_join].len();

    if ((clapper - 1) / column_length) % 2 == 0 {
        columns[column_to_join].insert((clapper - 1) % column_length, clapper);
    } else if clapper % column_length == 1 {
        columns[column_to_join].push_back(clapper);
    } else {
        columns[column_to_join].insert(column_length - ((clapper - 1) % column_length), clapper);
    }
}

fn shout(columns: &[VecDeque<usize>]) -> String {
    let mut answer = String::default();
    for column in columns {
        answer += &column.front().unwrap().to_string();
    }
    answer
}

fn part1(input: &str, rounds: usize) -> String {
    let mut columns = parse_input(input);
    let n = columns.len();
    let mut last_shout = String::default();
    for round in 0..rounds {
        run_round(&mut columns, round % n);
        last_shout = shout(&columns);
        // println!("Round: {}", round + 1);
        // println!("{last_shout}");
        // println!("{columns:?}");
    }
    last_shout
}

// ====== Part 2
fn hash_columns(columns: &[VecDeque<usize>]) -> u64 {
    use std::hash::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;
    let mut overall_hasher = DefaultHasher::new();

    for column in columns {
        let mut hasher = DefaultHasher::new();
        column.hash(&mut hasher);
        overall_hasher.write_u64(hasher.finish());
    }

    overall_hasher.finish()
}

struct Dance {
    columns: Vec<VecDeque<usize>>,
    current_round: usize,
}

impl Dance {
    fn new(columns: Vec<VecDeque<usize>>) -> Self {
        Self {
            columns,
            current_round: 0,
        }
    }
}

impl Iterator for Dance {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.columns.len();
        run_round(&mut self.columns, self.current_round % n);
        self.current_round += 1;
        Some(hash_columns(&self.columns))
    }
}

fn part2(input: &str) -> usize {
    use std::collections::HashMap;
    let mut shouts = HashMap::<String, usize>::default();
    let columns = parse_input(input);
    let mut dance = Dance::new(columns);
    let mut rounds = 0;
    let mut last_shout: String;

    loop {
        dance.next();
        last_shout = shout(&dance.columns);
        *shouts.entry(last_shout.clone()).or_insert(0usize) += 1;
        rounds += 1;
        if *shouts.get(&last_shout).unwrap() == 2024 {
            break;
        }
    }

    println!("Stopped at {rounds} with last shout {last_shout}");
    rounds * last_shout.parse::<usize>().unwrap()
}

fn part3(input: &str) -> usize {
    use std::collections::HashSet;
    let mut shouts = HashSet::<String>::default();
    let columns = parse_input(input);
    let mut dance = Dance::new(columns);
    let mut rounds = 0;

    loop {
        dance.next();
        shouts.insert(shout(&dance.columns));
        rounds += 1;
        if rounds == 100000 {
            break;
        }
    }

    shouts
        .into_iter()
        .map(|shout| shout.parse::<usize>().unwrap())
        .max()
        .unwrap()
}

fn main() {
    let test1 = include_str!("../test1");
    println!("Part 1 (test): {}", part1(test1, 10));
    let test2 = include_str!("../test2");
    println!("Part 2 (test): {}", part2(test2));
    let input1 = include_str!("../input1");
    println!("Part 1: {}", part1(input1, 10));
    let input2 = include_str!("../input2");
    println!("Part 2: {}", part2(input2));
    let input3 = include_str!("../input3");
    println!("Part 3: {}", part3(input3));
}

mod cycledetection {
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::hash::Hash;

    #[derive(Debug, Clone)]
    pub struct CycleDefinition {
        pub first_occurence: usize,
        pub cycle_length: usize,
        pub occurences: Vec<usize>,
    }

    impl CycleDefinition {
        pub fn new(first_occurence: usize, cycle_length: usize, occurences: Vec<usize>) -> Self {
            Self {
                first_occurence,
                cycle_length,
                occurences,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Cycle<T> {
        pub item: T,
        pub definition: CycleDefinition,
    }

    impl<T> Cycle<T> {
        fn new(item: T, definition: CycleDefinition) -> Self {
            Self { item, definition }
        }
    }

    impl<T> std::fmt::Display for Cycle<T>
    where
        T: Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            writeln!(f, "Cycle detected")?;
            writeln!(f, "Item: {:?}", self.item)?;
            writeln!(f, "First occurence: {}", self.definition.first_occurence)?;
            writeln!(f, "Cycle length: {}", self.definition.cycle_length)?;
            writeln!(f, "Occurences: {:?}", self.definition.occurences)
        }
    }

    pub struct CycleDetector<Item, Iter> {
        memory: HashMap<Item, Vec<usize>>,
        iterator: Iter,
        cycle_limit: usize,
        counter: usize,
        min_occurences_to_detect_cycle: usize,
        min_same_diffs_to_confirm_cycle: usize,
        already_run: bool,
    }

    impl<Item, Iter> CycleDetector<Item, Iter>
    where
        Iter: Iterator<Item = Item>,
        Item: Eq + Hash + Debug + Clone,
    {
        pub fn new(iterator: Iter, cycle_limit: usize) -> Self {
            Self {
                memory: HashMap::new(),
                iterator,
                cycle_limit,
                counter: 0,
                min_occurences_to_detect_cycle: 3,
                min_same_diffs_to_confirm_cycle: 2,
                already_run: false,
            }
        }

        pub fn run(&mut self) -> Option<Cycle<Item>> {
            if self.already_run {
                println!("Already run");
                return None;
            } else {
                self.already_run = true;
            }
            println!("Running the detector");
            while self.counter < self.cycle_limit {
                if let Some(item) = self.iterator.next() {
                    self.memory
                        .entry(item.clone())
                        .or_default()
                        .push(self.counter);
                    if let Some(definition) = self.detect_cycle(self.memory.get(&item).unwrap()) {
                        return Some(Cycle::new(item, definition));
                    }
                    self.counter += 1;
                }
            }

            println!("No cycle detected");
            None
        }

        fn detect_cycle(&self, occurences: &[usize]) -> Option<CycleDefinition> {
            if occurences.len() < self.min_occurences_to_detect_cycle {
                return None;
            }

            let occurence_diffs: Vec<usize> = occurences
                .windows(2)
                .rev()
                .map(|window| window[1] - window[0])
                .collect();
            if occurence_diffs.len() < self.min_same_diffs_to_confirm_cycle {
                return None;
            }

            if occurence_diffs
                .iter()
                .take(self.min_same_diffs_to_confirm_cycle)
                .all(|&diff| diff == occurence_diffs[0])
            {
                // Cycle detected
                let first_occurence = occurences[0];
                let cycle_length = occurence_diffs[0];
                Some(CycleDefinition::new(
                    first_occurence,
                    cycle_length,
                    occurences.to_vec(),
                ))
            } else {
                None
            }
        }
    }
}
