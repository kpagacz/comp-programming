// https://leetcode.com/problems/design-movie-rental-system/description/?envType=daily-question&envId=2025-09-21
use std::collections::{BTreeSet, HashMap};

struct MovieRentingSystem {
    to_rent: HashMap<i32, BTreeSet<(i32, i32)>>, // movie -> (price, shop id)
    rented: BTreeSet<(i32, i32, i32)>,           // price, shop id, movie id
    movies: HashMap<(i32, i32), i32>,            // (movie, shop) -> price
}

#[allow(dead_code)]
impl MovieRentingSystem {
    fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut to_rent: HashMap<i32, BTreeSet<(i32, i32)>> = HashMap::new();
        let mut movies = HashMap::new();
        for entry in entries {
            let (shop, movie, price) = (entry[0], entry[1], entry[2]);
            to_rent
                .entry(movie)
                .and_modify(|set| {
                    set.insert((price, shop));
                })
                .or_insert(BTreeSet::from([(price, shop)]));
            movies.insert((movie, shop), price);
        }
        Self {
            to_rent,
            rented: BTreeSet::new(),
            movies,
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        self.to_rent
            .get(&movie)
            .map(|set| set.iter().take(5).map(|(_, shop)| *shop).collect())
            .unwrap_or_default()
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let price = *self.movies.get(&(movie, shop)).unwrap();
        self.rented.insert((price, shop, movie));
        self.to_rent
            .get_mut(&movie)
            .map(|set| set.remove(&(price, shop)));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let price = *self.movies.get(&(movie, shop)).unwrap();
        self.rented.remove(&(price, shop, movie));
        self.to_rent
            .get_mut(&movie)
            .map(|set| set.insert((price, shop)));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.rented
            .iter()
            .take(5)
            .map(|(_, shop, movie)| vec![*shop, *movie])
            .collect()
    }
}

fn main() {}
