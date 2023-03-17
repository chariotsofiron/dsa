//! Topological sort
use std::{collections::HashMap, hash::Hash};

use super::Graph;

/// Kahn's topological sort algorithm.
/// O(|V| + |E|)
#[must_use]
pub fn kahns_algorithm<T>(graph: &Graph<T>) -> Vec<&T>
where
    T: Eq + Hash,
{
    // compute the "in-degree" for each node
    let mut in_degree = HashMap::<&T, u32>::with_capacity(graph.len());
    for neighbors in graph.values() {
        for node in neighbors {
            *in_degree.entry(node).or_insert(0) += 1;
        }
    }

    // get the nodes with no parents
    let mut queue: Vec<&T> = graph
        .keys()
        .filter(|x| !in_degree.contains_key(x))
        .collect();

    let mut ordering = Vec::with_capacity(graph.len());
    while let Some(parent) = queue.pop() {
        ordering.push(parent);
        if let Some(neighbors) = graph.get(parent) {
            for neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(neighbor);
                    }
                }
            }
        }
    }
    ordering
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::kahns_algorithm;

    #[test]
    fn test_kahn() {
        let graph = HashMap::from([
            (4, vec![1, 5]),
            (1, vec![5, 2]),
            (2, vec![3]),
            (5, vec![2, 3]),
            (3, vec![]),
        ]);

        assert_eq!(kahns_algorithm(&graph), vec![&4, &1, &5, &2, &3])
    }
}
