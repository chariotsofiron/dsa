//! Graph algorithms
use std::collections::HashMap;

pub mod dfs;
pub mod topological_sort;

/// A basic graph data structure.
pub type Graph<T> = HashMap<T, Vec<T>>;
