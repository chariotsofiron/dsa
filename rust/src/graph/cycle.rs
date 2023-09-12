use super::Graph;

impl Graph {
    /// Returns true if the graph is a directed acyclic graph.
    #[must_use]
    pub fn is_dag(&self) -> bool {
        let mut visited = vec![false; self.len()];
        let mut stack = vec![];
        for v in 0..self.len() {
            if !visited[v] {
                stack.push(v);
                while let Some(v) = stack.pop() {
                    if visited[v] {
                        return false;
                    }
                    visited[v] = true;
                    for (w, _) in self.neighbors(v) {
                        stack.push(w);
                    }
                }
            }
        }
        true
    }
}
