// https://leetcode.com/problems/reconstruct-itinerary/description
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_itinerary(mut tickets: Vec<Vec<String>>) -> Vec<String> {
        tickets.sort_unstable_by(|ticket1, ticket2| ticket1[1].cmp(&ticket2[1]));

        let mut ticket_dict = HashMap::new();
        for i in 0..tickets.len() {
            ticket_dict
                .entry(tickets[i][0].clone())
                .and_modify(|tickets: &mut Vec<usize>| tickets.push(i))
                .or_insert(vec![i]);
        }
        tickets.push(vec!["".to_owned(), "JFK".to_owned()]);
        let mut answer = vec![tickets.len() - 1];

        let mut used_tickets = vec![false; tickets.len()];
        Self::find_itinerary_rec(&mut answer, &ticket_dict, &tickets, &mut used_tickets);
        answer.iter().map(|id| tickets[*id][1].clone()).collect()
    }

    fn find_itinerary_rec(
        answer: &mut Vec<usize>,
        ticket_dict: &HashMap<String, Vec<usize>>,
        tickets: &Vec<Vec<String>>,
        used_tickets: &mut Vec<bool>,
    ) {
        if answer.len() == tickets.len() {
            return;
        }

        for &destination in ticket_dict
            .get(&tickets[*answer.last().unwrap()][1])
            .or(Some(&Vec::new()))
            .unwrap()
        {
            if used_tickets[destination] {
                continue;
            }
            answer.push(destination);
            used_tickets[destination] = true;
            Self::find_itinerary_rec(answer, ticket_dict, tickets, used_tickets);
            if answer.len() == tickets.len() {
                return;
            }
            used_tickets[*answer.last().unwrap()] = false;
            answer.pop();
        }
    }
}

fn main() {
    let data = vec![["JFK", "KUL"], ["JFK", "NRT"], ["NRT", "JFK"]];
    let case: Vec<_> = data
        .iter()
        .map(|ticket| vec![ticket[0].to_owned(), ticket[1].to_owned()])
        .collect();
    println!("{:?}", Solution::find_itinerary(case));
}
