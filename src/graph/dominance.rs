//! Dominance algorithms
//! <https://en.wikipedia.org/wiki/Dominator_(graph_theory)>
//! <https://www.cs.rice.edu/~keith/EMBED/dom.pdf>
//! <https://github.com/static-analysis-engineering/CodeHawk-Binary/blob/master/chb/app/Cfg.py>
// use super::Graph;

use super::Graph;

/// Computes the reverse-postorder of a graph.
fn reverse_post_order(graph: &Graph, start: usize) -> Vec<usize> {
    let mut ordering = graph.post_order(start).collect::<Vec<_>>();
    ordering.reverse();
    ordering
}

/// Finds the nearest common dominator of two nodes using the dominators array
fn common_dominator(
    mut node_a: usize,
    mut node_b: usize,
    doms: &[Option<usize>],
    rpo: &[usize],
) -> usize {
    let mut ordering = vec![0; rpo.len()];
    let mut i: usize = 0;
    for &b in rpo {
        ordering[b] = i;
        i += 1;
    }

    while node_a != node_b {
        // The paper describes comparisons on postorder numbers; we're using
        // the reverse-postorder numbers, so we need to flip the comparison
        while ordering[node_a] > ordering[node_b] {
            node_a = doms[node_a].unwrap();
        }
        while ordering[node_b] > ordering[node_a] {
            node_b = doms[node_b].unwrap();
        }
    }
    node_a
}

#[must_use]
pub fn dominators(graph: &Graph, start: usize) -> Vec<Option<usize>> {
    let rpo = reverse_post_order(graph, start);
    let transpose = graph.transpose();
    // Initialize the dominators array
    let mut idoms: Vec<Option<usize>> = vec![None; graph.len()];

    idoms[start] = Some(start);
    let mut changed = true;
    while changed {
        changed = false;
        // Iterate over the nodes in reverse postorder (except for the start node)
        for &b in &rpo {
            if b == start {
                continue;
            }
            let allpreds = transpose.neighbors(b).collect::<Vec<_>>();
            let mut new_idom = None;

            // Find the first (processed) predecessor of b and set it as the initial new_idom
            for &(pred, _) in &allpreds {
                if idoms[pred].is_some() {
                    new_idom = Some(pred);
                    break;
                }
            }
            let mut new_idom = new_idom.unwrap();

            // Find the nearest common dominator of b's other predecessors
            for &(p, _) in &allpreds {
                if idoms[p].is_some() {
                    new_idom = common_dominator(p, new_idom, &idoms, &rpo);
                }
            }
            // Update the dominator of b if necessary
            if idoms[b] != Some(new_idom) {
                idoms[b] = Some(new_idom);
                changed = true;
            }
        }
    }
    idoms
}

#[cfg(test)]
mod tests {
    use crate::graph::{dominance::reverse_post_order, Graph};

    #[test]
    fn test_reverse_postorder() {
        let graph = Graph::from([(0, 2), (0, 1), (2, 3), (1, 3)]);
        let rpo = reverse_post_order(&graph, 0);
        assert_eq!(rpo, vec![0, 2, 1, 3]);

        let graph = Graph::from([
            (6, 4),
            (6, 5),
            (4, 3),
            (4, 2),
            (5, 1),
            (1, 2),
            (2, 3),
            (2, 1),
            (3, 2),
        ]);
        let rpo = reverse_post_order(&graph, 6);
        assert_eq!(rpo, [6, 4, 5, 1, 2, 3]);
    }

    // #[test]
    // fn test() {
    //     let graph = Graph::from([(1, 2), (2, 3), (2, 4), (3, 5), (4, 5), (2, 6), (5, 2)]);

    //     let idoms = dominator(&graph, 1);
    //     println!("{:?}", idoms);
    // }
}
