use std::collections::HashMap;

mod dfs;
mod topological_sort;

/// A basic graph data structure.
pub type Graph<T> = HashMap<T, Vec<T>>;
