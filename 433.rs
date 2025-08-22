//433. Minimum Genetic Mutation

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        fn is_in_bank(bank_set: &HashSet<String>, current: &str) -> Vec<String> {
            let mut result = vec![];
            for gene in bank_set.iter() {
                let diff = gene.chars()
                    .zip(current.chars())
                    .filter(|(a, b)| a != b)
                    .count();
                if diff == 1 {
                    result.push(gene.clone());
                }
            }
            result
        }

        let mut bank_set: HashSet<String> = bank.into_iter().collect();
        if !bank_set.contains(&end_gene) {
            return -1;
        }

        let mut queue = VecDeque::new();
        queue.push_back(start_gene.clone());
        let mut steps = 0;

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();
                if current == end_gene {
                    return steps;
                }

                let next_mutations = is_in_bank(&bank_set, &current);
                for next in next_mutations {
                    queue.push_back(next.clone());
                    bank_set.remove(&next); 
                }
            }
            steps += 1;
        }

        -1
    }
}
