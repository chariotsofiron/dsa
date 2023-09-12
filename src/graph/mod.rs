//! Basic graph module without explicit support for deletion.
//!
//! # Panics
//!
//! All methods will panic if given an out-of-bounds element index.
pub mod cycle;
pub mod dijkstra;
pub mod dominance;
pub mod topological_sort;
pub mod traversal;

/// A compact directed-graph representation. Edges are numbered in order of insertion.
/// Each adjacency list consists of all edges pointing out from a given vertex.
/// Doesn't support node/edge deletions?
/// Space: O(|V| + |E|)
pub struct Graph {
    // more performant than a hashmap?
    /// Maps a vertex id to the first edge in its adjacency list.
    first: Vec<Option<usize>>,
    /// Maps an edge id to the next edge in the same adjacency list.
    next_edge: Vec<Option<usize>>,
    /// Maps an edge id to the vertex that it points to.
    end_vertex: Vec<usize>,
}

impl Graph {
    /// Initializes a graph with `vmax` vertices and no edges. To reduce
    /// unnecessary allocations, `emax_hint` should be close to the number of
    /// edges that will be inserted.
    #[must_use]
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            first: vec![None; vmax],
            next_edge: Vec::with_capacity(emax_hint),
            end_vertex: Vec::with_capacity(emax_hint),
        }
    }

    /// Returns the number of vertices in the graph.
    #[must_use]
    pub fn len(&self) -> usize {
        self.first.len()
    }

    /// Returns true if the graph has no vertices.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.first.is_empty()
    }

    /// Returns the number of edges in the graph.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.end_vertex.len()
    }

    /// Adds a directed edge from `from` to `to`.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.next_edge.push(self.first[from]);
        self.first[from] = Some(self.end_vertex.len());
        self.end_vertex.push(to);
    }

    /// Gets vertex `node`'s neighbors. These are returned in reverse order.
    #[must_use]
    pub fn neighbors(&self, node: usize) -> NeighborIterator {
        NeighborIterator {
            graph: self,
            next_edge: self.first[node],
        }
    }

    /// Returns a transposed version of the graph.
    /// <https://en.wikipedia.org/wiki/Transpose_graph>
    /// Time complexity: O(|V| + |E|)
    #[must_use]
    pub fn transpose(&self) -> Self {
        let mut graph = Self::new(self.len(), self.edge_count());
        for &entry in &self.first {
            if let Some(node) = entry {
                for (v, _) in self.neighbors(node) {
                    graph.add_edge(v, node);
                }
            }
        }
        graph
    }
}

/// An iterator for convenient adjacency list traversal.
pub struct NeighborIterator<'a> {
    /// The graph that this iterator is iterating over.
    graph: &'a Graph,
    /// The next edge in the adjacency list.
    next_edge: Option<usize>,
}

impl<'a> Iterator for NeighborIterator<'a> {
    type Item = (usize, usize);

    /// Produces an outgoing edge and vertex.
    fn next(&mut self) -> Option<Self::Item> {
        self.next_edge.map(|e| {
            let v = self.graph.end_vertex[e];
            self.next_edge = self.graph.next_edge[e];
            (v, e)
        })
    }
}

impl<const N: usize> From<[(usize, usize); N]> for Graph {
    fn from(edges: [(usize, usize); N]) -> Self {
        let vmax = edges
            .iter()
            .map(|&(u, v)| u.max(v))
            .max()
            .unwrap_or_default();
        let mut graph = Self::new(vmax.saturating_add(1), edges.len());
        for (u, v) in edges {
            graph.add_edge(u, v);
        }
        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let graph = Graph::from([(2, 3), (2, 4), (4, 1), (1, 2)]);

        assert_eq!(graph.len(), 5);
        assert_eq!(graph.edge_count(), 4);
        assert_eq!(graph.neighbors(2).collect::<Vec<_>>(), [(4, 1), (3, 0)]);
    }

    #[test]
    fn test_transpose() {
        let graph = Graph::from([(2, 3), (2, 4), (1, 3)]);
        assert_eq!(graph.neighbors(2).collect::<Vec<_>>(), [(4, 1), (3, 0)]);
        let transpose = graph.transpose();
        assert_eq!(transpose.neighbors(3).collect::<Vec<_>>(), [(1, 2), (2, 1)]);
    }
}
