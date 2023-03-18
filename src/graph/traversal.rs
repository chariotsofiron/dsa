//! Directed graph traversal algorithms
use std::collections::{HashSet, VecDeque};

use super::Graph;

/// Returns a vector of nodes in preorder traversal order.
/// This is the same as DFS <https://en.wikipedia.org/wiki/Depth-first_search>
/// Visit current node before children.
#[must_use]
pub fn preorder<'a, T>(graph: &'a Graph<T>, start: &'a T) -> Vec<&'a T>
where
    T: Eq + std::hash::Hash,
{
    let mut seen = HashSet::<&T>::new();
    let mut path = Vec::new();
    let mut stack = vec![start];
    while let Some(node) = stack.pop() {
        if !seen.contains(node) {
            path.push(node);
            seen.insert(node);
            if let Some(neighbors) = graph.get(node) {
                for neighbor in neighbors.iter().rev() {
                    stack.push(neighbor);
                }
            }
        }
    }
    path
}

///
/// aka breadth first search (BFS) <https://en.wikipedia.org/wiki/Breadth-first_search>
#[must_use]
pub fn levelorder<'a, T>(graph: &'a Graph<T>, start: &'a T) -> Vec<&'a T>
where
    T: Eq + std::hash::Hash,
{
    let mut seen = HashSet::<&T>::new();
    let mut path: Vec<&T> = Vec::new();
    let mut queue = VecDeque::from([start]);

    while let Some(node) = queue.pop_front() {
        if !seen.contains(node) {
            path.push(node);
            seen.insert(node);
            if let Some(neighbors) = graph.get(node) {
                for neighbor in neighbors {
                    queue.push_back(neighbor);
                }
            }
        }
    }
    path
}

/// Returns a vector of nodes in postorder traversal order.
/// The vertices are listed in the order in which they are last visited
/// by a DFS traversal.
#[must_use]
pub fn postorder<'a, T>(graph: &'a Graph<T>, start: &'a T) -> Vec<&'a T>
where
    T: Eq + std::hash::Hash,
{
    // visit the children, then visit myself
    // <https://stackoverflow.com/questions/36488968/post-order-graph-traversal>
    let mut visited = HashSet::with_capacity(graph.len());
    let mut stack = vec![start];
    let mut path = Vec::new();
    visited.insert(start);
    while let Some(&node) = stack.last() {
        let mut tail = true;
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor);
                    stack.push(neighbor);
                    tail = false;
                    break;
                }
            }
        }
        if tail {
            if let Some(item) = stack.pop() {
                path.push(item);
            }
        }
    }

    path
}

#[cfg(test)]
mod tests {
    use crate::graph::traversal::{levelorder, preorder};

    use super::postorder;
    use std::collections::HashMap;

    #[test]
    fn test() {
        let graph = HashMap::from([
            (1, vec![2, 4]),
            (2, vec![5]),
            (3, vec![5, 6]),
            (4, vec![2]),
            (5, vec![4, 6]),
        ]);
        assert_eq!(postorder(&graph, &1), vec![&4, &6, &5, &2, &1]);

        let graph = HashMap::from([
            (1, vec![2]),
            (2, vec![1, 3]),
            (4, vec![2, 3]),
            (5, vec![1]),
            (6, vec![5, 4]),
        ]);
        assert_eq!(postorder(&graph, &6), vec![&3, &2, &1, &5, &4, &6]);

        /* tree structure
            1
           / \
          2   3
         /   / \
        4   5   6
           / \
          7   8
         */
        let graph = HashMap::from([
            (1, vec![2, 3]),
            (2, vec![4]),
            (3, vec![5, 6]),
            (5, vec![7, 8]),
        ]);
        assert_eq!(postorder(&graph, &1), vec![&4, &2, &7, &8, &5, &6, &3, &1]);
        assert_eq!(preorder(&graph, &1), vec![&1, &2, &4, &3, &5, &7, &8, &6]);
        assert_eq!(levelorder(&graph, &1), vec![&1, &2, &3, &4, &5, &6, &7, &8]);
    }
}
