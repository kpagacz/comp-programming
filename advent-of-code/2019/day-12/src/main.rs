// https://adventofcode.com/2019/day/12

use std::ops::Add;

use aoc_utils::cycledetector::CycleDetector;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Triple(i32, i32, i32);
impl Triple {
    fn energy(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}
impl Add for Triple {
    type Output = Triple;

    fn add(self, rhs: Self) -> Self::Output {
        Triple(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Moon(Triple, Triple);
impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self(Triple(x, y, z), Triple(0, 0, 0))
    }

    fn energy(&self) -> i32 {
        self.0.energy() * self.1.energy()
    }

    fn x(&self) -> &i32 {
        &self.0 .0
    }
    fn y(&self) -> &i32 {
        &self.0 .1
    }
    fn z(&self) -> &i32 {
        &self.0 .2
    }
    fn vx(&self) -> i32 {
        self.1 .0
    }
    fn vy(&self) -> i32 {
        self.1 .1
    }
    fn vz(&self) -> i32 {
        self.1 .2
    }

    fn measure_gravity(&self, other: &Self) -> Triple {
        Triple(
            match self.x().cmp(other.x()) {
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => -1,
            },
            match self.y().cmp(other.y()) {
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => -1,
            },
            match self.z().cmp(other.z()) {
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => -1,
            },
        )
    }

    fn apply_velocity(&mut self) {
        self.0 = self.0 + self.1;
    }

    fn apply_gravity(&mut self, velocity_changes: Triple) {
        self.1 = self.1 + velocity_changes;
    }
}

fn apply_gravity(moons: &mut [Moon]) {
    let mut overall_gravity = vec![Triple::default(); moons.len()];
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            overall_gravity[i] = overall_gravity[i] + moons[i].measure_gravity(&moons[j]);
        }
    }

    for i in 0..moons.len() {
        moons[i].apply_gravity(overall_gravity[i]);
    }
}

fn do_loop(moons: &mut [Moon]) {
    apply_gravity(moons);
    moons.iter_mut().for_each(|moon| moon.apply_velocity());
}

fn part1(mut moons: Vec<Moon>, steps: i32) -> i32 {
    for _ in 0..steps {
        do_loop(&mut moons);
    }

    moons.iter().map(|moon| moon.energy()).sum::<i32>()
}

trait Dimension {
    fn get_pos(moon: &Moon) -> i32;
    fn get_v(moon: &Moon) -> i32;
}
struct X;
struct Y;
struct Z;
impl Dimension for X {
    fn get_pos(moon: &Moon) -> i32 {
        *moon.x()
    }
    fn get_v(moon: &Moon) -> i32 {
        moon.vx()
    }
}
impl Dimension for Y {
    fn get_pos(moon: &Moon) -> i32 {
        *moon.y()
    }
    fn get_v(moon: &Moon) -> i32 {
        moon.vy()
    }
}
impl Dimension for Z {
    fn get_pos(moon: &Moon) -> i32 {
        *moon.z()
    }
    fn get_v(moon: &Moon) -> i32 {
        moon.vz()
    }
}

struct OneDimensionalNBodyProblem<D: Dimension> {
    moons: Vec<Moon>,
    phantom: std::marker::PhantomData<D>,
}

impl<D: Dimension> OneDimensionalNBodyProblem<D> {
    fn new(moons: Vec<Moon>) -> Self {
        Self {
            moons,
            phantom: std::marker::PhantomData::<D> {},
        }
    }
}

impl<D: Dimension> Iterator for OneDimensionalNBodyProblem<D> {
    type Item = (i32, i32, i32, i32, i32, i32, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let mut it = self
            .moons
            .iter()
            .map(|moon| (D::get_pos(moon), D::get_v(moon)));
        let mut res = (0, 0, 0, 0, 0, 0, 0, 0);
        (res.0, res.1) = it.next().unwrap();
        (res.2, res.3) = it.next().unwrap();
        (res.4, res.5) = it.next().unwrap();
        (res.6, res.7) = it.next().unwrap();

        do_loop(&mut self.moons);
        Some(res)
    }
}

fn part2(moons: Vec<Moon>, cycle_limit: usize) -> usize {
    let mut cycles = vec![];

    let x_it = OneDimensionalNBodyProblem::<X>::new(moons.clone());
    let mut det = CycleDetector::new(x_it, cycle_limit);
    if let Some(cycle) = det.run() {
        println!("{cycle}");
        cycles.push(cycle.definition.cycle_length);
    }

    let y_it = OneDimensionalNBodyProblem::<Y>::new(moons.clone());
    let mut det = CycleDetector::new(y_it, cycle_limit);
    if let Some(cycle) = det.run() {
        println!("{cycle}");
        cycles.push(cycle.definition.cycle_length);
    }

    let z_it = OneDimensionalNBodyProblem::<Z>::new(moons.clone());
    let mut det = CycleDetector::new(z_it, cycle_limit);
    if let Some(cycle) = det.run() {
        println!("{cycle}");
        cycles.push(cycle.definition.cycle_length);
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    let common_gcd = cycles.iter().copied().reduce(gcd).unwrap();
    cycles
        .into_iter()
        .reduce(|acc, num| (acc * num) / common_gcd)
        .unwrap()
        / common_gcd
}

fn main() {
    let input = vec![
        Moon::new(-19, -4, 2),
        Moon::new(-9, 8, -16),
        Moon::new(-4, 5, -11),
        Moon::new(1, 9, -13),
    ];
    let test = vec![
        Moon::new(-1, 0, 2),
        Moon::new(2, -10, -7),
        Moon::new(4, -8, 8),
        Moon::new(3, 5, -1),
    ];
    let test2 = vec![
        Moon::new(-8, -10, 0),
        Moon::new(5, 5, 10),
        Moon::new(2, -7, 3),
        Moon::new(9, -8, -3),
    ];

    println!("Part 1 (test with 10 steps): {}", part1(test.clone(), 10));
    println!(
        "Part 1 (test2 with 100 steps): {}",
        part1(test2.clone(), 100)
    );
    println!(
        "Part 1 (input with 1000 steps): {}",
        part1(input.clone(), 1000)
    );

    println!("Part 2 (test):");
    let a = part2(test.clone(), 10000);
    println!("Answer: {a}");

    println!("Part 2 (input):");
    let a = part2(input.clone(), 1000000);
    println!("Answer: {a}");
}
