use std::usize;

use crate::{graph::AllFrom, search::Solver};

pub enum EdgeMod {
    Merge(usize, usize),
    Cut(usize, usize),
    Nothing,
}

impl Solver {
    // find best edge to split on in O(n + m^2) time
    // edge is positive
    // there is at least one conflict
    pub fn best_edge(&mut self) -> EdgeMod {
        let mut best = EdgeMod::Nothing;
        let mut best_cost = 0;

        for (i1, v1) in self.graph.active.all(0) {
            for (_, v2) in self.graph.active.all(i1) {
                if self.graph[[v1, v2]].fixed {
                    continue;
                }
                let cost = self.graph[[v1, v2]].conflicts;
                if cost > best_cost {
                    best_cost = cost;
                    if self.graph[[v1, v2]].weight > 0 {
                        best = EdgeMod::Cut(v1, v2)
                    } else {
                        best = EdgeMod::Merge(v1, v2)
                    }
                }
            }
        }
        best
    }
}
