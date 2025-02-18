//332. Reconstruct Itinerary

//attempt one: wrong answer 
/*use std::collections::{BTreeMap, BTreeSet, VecDeque};

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut flight_map: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        
        // Build adjacency list with sorted destinations
        for ticket in tickets {
            flight_map.entry(ticket[0].clone())
                .or_insert_with(BTreeSet::new)
                .insert(ticket[1].clone());
        }

        let mut stack = VecDeque::new();
        let mut itinerary = Vec::new();
        stack.push_back("JFK".to_string());

        while let Some(airport) = stack.back() {
            if let Some(destinations) = flight_map.get_mut(airport) {
                if let Some(next_dest) = destinations.iter().next().cloned() {
                    destinations.remove(&next_dest);
                    stack.push_back(next_dest);
                    continue;
                }
            }
            itinerary.insert(0, stack.pop_back().unwrap());
        }

        itinerary
    }
}*/

//Attempt two: Successful 
use std::collections::{BTreeMap, BinaryHeap, VecDeque};

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut flight_map: BTreeMap<String, BinaryHeap<std::cmp::Reverse<String>>> = BTreeMap::new();

        // Build adjacency list with a min-heap to maintain lexical order
        for ticket in tickets {
            flight_map
                .entry(ticket[0].clone())
                .or_insert_with(BinaryHeap::new)
                .push(std::cmp::Reverse(ticket[1].clone()));
        }

        let mut stack = VecDeque::new();
        let mut itinerary = Vec::new();
        stack.push_back("JFK".to_string());

        while let Some(airport) = stack.back() {
            if let Some(destinations) = flight_map.get_mut(airport) {
                if let Some(std::cmp::Reverse(next_dest)) = destinations.pop() {
                    stack.push_back(next_dest);
                    continue;
                }
            }
            itinerary.insert(0, stack.pop_back().unwrap());
        }

        itinerary
    }
}

