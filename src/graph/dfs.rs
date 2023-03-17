//! Depth-first search
use std::{collections::HashSet, hash::Hash};

use super::Graph;

/// Performs a depth first search
/// <https://en.wikipedia.org/wiki/Depth-first_search>
pub fn depth_first_search<'src, T>(graph: &'src Graph<T>, start: &'src T) -> Vec<&'src T>
where
    T: Eq + Hash,
{
    // potentially over-allocates if the graph is disconnected
    let mut path = Vec::with_capacity(graph.len());
    let mut seen = HashSet::<&T>::new();
    let mut queue = vec![start];
    while let Some(node) = queue.pop() {
        if !seen.contains(node) {
            path.push(node);
            seen.insert(node);
            if let Some(neighbors) = graph.get(node) {
                for neighbor in neighbors {
                    queue.push(neighbor);
                }
            }
        }
    }
    path
}

#[cfg(test)]
mod tests {
    use super::depth_first_search;
    use std::collections::HashMap;

    #[test]
    fn test_dfs() {
        let graph = HashMap::from([
            (4, vec![1, 5]),
            (1, vec![5, 2]),
            (2, vec![3]),
            (5, vec![2, 3]),
            (3, vec![]),
        ]);

        assert_eq!(depth_first_search(&graph, &4), vec![&4, &5, &3, &2, &1])
    }
}
