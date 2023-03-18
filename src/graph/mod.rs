//! Graph algorithms
//! The graphs operate on node ids since that's much more efficient
//! Just use a hashmap to map node ids to node data if you need that.
use std::{collections::HashMap, num::NonZeroUsize};

pub mod topological_sort;
pub mod traversal;

/// A simple directed graph data structure.
/// May be disconnected.
pub type Graph<T> = HashMap<T, Vec<T>>;
