use std::usize;

use crate::search::Solver;

pub enum EdgeMod {
    Merge(usize, usize),
    Delete(usize, usize),
    Nothing,
}

impl Solver {
    // find best edge to split on in O(n + m^2) time
    // edge is positive
    // there is at least one conflict
    pub fn best_edge(&mut self) -> EdgeMod {
        let mut best = EdgeMod::Nothing;
        let mut best_count = 2;

        for (i1, v1) in self.graph.all(0) {
            for (_, v2) in self.graph.positive(v1, i1) {
                let mut count = 0;
                for (_, v3) in self.graph.all(0) {
                    count += (-self.graph[v1][v3].weight ^ -self.graph[v2][v3].weight < 0) as i32;
                }
                if count - self.edge_markers[v1][v2] as i32 > best_count {
                    best_count = count - self.edge_markers[v1][v2] as i32;
                    best = EdgeMod::Delete(v1, v2)
                }
            }
        }
        best
    }
}
