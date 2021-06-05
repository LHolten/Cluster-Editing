use std::{cmp::min, usize};

use crate::search::Solver;

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
        let mut best_cost = i32::MAX;

        for (i1, v1) in self.graph.all(0) {
            for (_, v2) in self.graph.all(i1) {
                let e12 = self.graph[v1][v2].weight > 0;
                if self.graph[v1][v2].fixed {
                    continue;
                }

                let cost = self.edge_markers[v1][v2] - self.edge_two[v1][v2];
                if cost >= 0 {
                    continue;
                }
                // cost += self.graph[v1][v2].weight.abs();

                if cost < best_cost {
                    best_cost = cost;
                    if e12 {
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
