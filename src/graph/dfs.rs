use std::{collections::HashSet, hash::Hash};

use super::Graph;

/// Performs a depth first search
/// https://en.wikipedia.org/wiki/Depth-first_search
fn depth_first_search<'a, T>(graph: &'a Graph<T>, start: &'a T) -> Vec<&'a T>
where
    T: Eq + Hash,
{
    let mut path = Vec::with_capacity(graph.len());
    let mut seen = HashSet::<&T>::new();
    let mut queue = vec![start];
    while let Some(node) = queue.pop() {
        if !seen.contains(node) {
            path.push(node);
            seen.insert(node);
            if let Some(neighbors) = graph.get(&node) {
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
    use std::collections::HashMap;

    use super::depth_first_search;

    #[test]
    fn test_kahn() {
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
