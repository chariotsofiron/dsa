//! Graph traversal algorithms.
use super::{Graph, NeighborIterator};
use std::collections::VecDeque;

impl Graph {
    /// aka breadth first search (BFS) <https://en.wikipedia.org/wiki/Breadth-first_search>
    #[must_use]
    pub fn level_order(&self, start: usize) -> LevelOrderIterator {
        let mut visited = vec![false; self.len()];
        visited[start] = true;
        let queue = VecDeque::new();
        let neighbors = self.neighbors(start);
        LevelOrderIterator {
            graph: self,
            visited,
            queue,
            neighbors,
        }
    }

    /// Returns a vector of nodes in preorder traversal order.
    /// This is the same as DFS <https://en.wikipedia.org/wiki/Depth-first_search>
    /// Visit current node before children.
    #[must_use]
    pub fn pre_order(&self, start: usize) -> PreOrderIterator {
        let neighbors = (0..self.len())
            .map(|node| self.neighbors(node))
            .collect::<Vec<_>>();
        let mut visited = vec![false; self.len()];
        visited[start] = true;
        PreOrderIterator {
            stack: vec![start],
            visited,
            neighbors,
        }
    }

    /// Returns a vector of nodes in postorder traversal order.
    /// The vertices are listed in the order in which they are last visited
    /// by a DFS traversal.
    #[must_use]
    pub fn post_order(&self, start: usize) -> PostOrderIterator {
        let neighbors = (0..self.len())
            .map(|node| self.neighbors(node))
            .collect::<Vec<_>>();
        let mut visited = vec![false; self.len()];
        visited[start] = true;
        PostOrderIterator {
            stack: vec![start],
            visited,
            neighbors,
            tail: false,
        }
    }
}

/// Iterator over the nodes of a graph in postorder traversal order.
pub struct PreOrderIterator<'a> {
    /// The graph that this iterator is iterating over.
    stack: Vec<usize>,
    /// true if the node has been visited
    visited: Vec<bool>,
    /// neighbors of the current node
    neighbors: Vec<NeighborIterator<'a>>,
}

impl Iterator for PreOrderIterator<'_> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let &u = self.stack.last()?;
            for (v, e) in self.neighbors[u].by_ref() {
                if !self.visited[v] {
                    self.visited[v] = true;
                    self.stack.push(v);
                    return Some((v, e));
                }
            }
            self.stack.pop();
        }
    }
}

/// Iterator over the nodes of a graph in level order traversal order.
pub struct LevelOrderIterator<'a> {
    /// The graph that this iterator is iterating over.
    graph: &'a Graph,
    /// true if the node has been visited
    visited: Vec<bool>,
    /// queue of nodes to visit
    queue: VecDeque<usize>,
    /// neighbors of the current node
    neighbors: NeighborIterator<'a>,
}

impl Iterator for LevelOrderIterator<'_> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        if let Some((neighbor, edge)) = self.neighbors.next() {
            if !self.visited[neighbor] {
                self.visited[neighbor] = true;
                self.queue.push_back(neighbor);
            }
            Some((neighbor, edge))
        } else {
            let node = self.queue.pop_front()?;
            self.neighbors = self.graph.neighbors(node);
            self.next()
        }
    }
}

/// Iterator over the nodes of a graph in postorder traversal order.
pub struct PostOrderIterator<'a> {
    /// stack of nodes to visit
    stack: Vec<usize>,
    /// true if the node has been visited
    visited: Vec<bool>,
    /// neighbors of each node
    neighbors: Vec<NeighborIterator<'a>>,
    /// true if the last node popped from the stack was a tail node
    tail: bool,
}

impl Iterator for PostOrderIterator<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        loop {
            let &u = self.stack.last()?;
            self.tail = true;
            for (v, _) in self.neighbors[u].by_ref() {
                if !self.visited[v] {
                    self.visited[v] = true;
                    self.stack.push(v);
                    self.tail = false;
                    break;
                }
            }
            if self.tail {
                return self.stack.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Graph;

    #[test]
    fn test() {
        /* tree structure
            1
           / \
          2   3
         /   / \
        4   5   6
           / \
          7   8
         */
        let graph = Graph::from([(1, 3), (1, 2), (2, 4), (3, 6), (3, 5), (5, 8), (5, 7)]);

        assert_eq!(
            graph.pre_order(1).collect::<Vec<_>>(),
            [(2, 1), (4, 2), (3, 0), (5, 4), (7, 6), (8, 5), (6, 3)]
        );

        assert_eq!(
            graph.level_order(1).collect::<Vec<_>>(),
            [(2, 1), (3, 0), (4, 2), (5, 4), (6, 3), (7, 6), (8, 5)]
        );

        assert_eq!(
            graph.post_order(1).collect::<Vec<_>>(),
            [4, 2, 7, 8, 5, 6, 3, 1]
        );

        let graph = Graph::from([
            (1, 4),
            (1, 2),
            (2, 5),
            (3, 6),
            (3, 5),
            (4, 2),
            (5, 6),
            (5, 4),
        ]);
        assert_eq!(graph.post_order(1).collect::<Vec<_>>(), [4, 6, 5, 2, 1]);

        let graph = Graph::from([
            (1, 2),
            (2, 3),
            (2, 1),
            (4, 3),
            (4, 2),
            (5, 1),
            (6, 4),
            (6, 5),
        ]);
        assert_eq!(graph.post_order(6).collect::<Vec<_>>(), [3, 2, 1, 5, 4, 6]);
    }
}
