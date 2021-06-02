use std::usize;

use crate::graph::Graph;

pub enum EdgeMod {
    Merge(usize, usize),
    Delete(usize, usize),
    Nothing,
}

impl Graph {
    // find best edge to split on in O(n + m^2) time
    // edge is positive
    // there is at least one conflict
    pub fn best_edge(&mut self) -> EdgeMod {
        let mut best = EdgeMod::Nothing;
        let mut best_count = 2;

        for (i1, v1) in self.clusters(0) {
            for (_, v2) in self.positive(v1, i1) {
                let mut count = 0;
                for pair in self.all_edges(v1, v2, 0) {
                    count += (-pair.edge1.weight ^ -pair.edge2.weight < 0) as i32;
                }
                if count - self[v1][v2].marked.get() as i32 > best_count {
                    best_count = count - self[v1][v2].marked.get() as i32;
                    best = EdgeMod::Delete(v1, v2)
                }
            }
        }
        best
    }
}
