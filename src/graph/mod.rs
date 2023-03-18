//! Graph algorithms
use std::collections::HashMap;

pub mod topological_sort;
pub mod traversal;

/// A simple directed graph data structure.
/// May be disconnected.
pub type Graph<T> = HashMap<T, Vec<T>>;
