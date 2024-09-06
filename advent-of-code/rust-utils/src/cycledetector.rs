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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let v = vec![7];
        CycleDetector::new(v.into_iter(), 1);
    }

    #[test]
    fn detect_cycle() {
        let v = vec![7, 7, 7];

        let mut det = CycleDetector::new(v.into_iter(), 10);
        let cycle = det.run();
        assert!(cycle.is_some());
        let cycle = cycle.unwrap();
        println!("{cycle}");

        let v = vec![1, 2, 7, 2, 3, 7, 9, 8, 7];
        let mut det = CycleDetector::new(v.into_iter(), 10);
        let cycle = det.run();
        assert!(cycle.is_some());
        let cycle = cycle.unwrap();
        println!("{cycle}");

        let v = vec![7, 1, 2, 3, 7, 4, 5, 7, 6, 8, 7];
        let mut det = CycleDetector::new(v.into_iter(), 100);
        let cycle = det.run();
        assert!(cycle.is_some());
        let cycle = cycle.unwrap();
        println!("{cycle}");
    }
}
