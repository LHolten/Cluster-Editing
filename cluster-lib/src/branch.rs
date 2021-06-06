use std::usize;

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
        let mut best_cost = 0;

        for (i1, v1) in self.graph.all(0) {
            for (_, v2) in self.graph.all(i1) {
                let e12 = self.graph[v1][v2].weight > 0;
                if self.graph[v1][v2].fixed {
                    continue;
                }
                // if self.edge_two[v1][v2] == 0 {
                //     continue;
                // }

                // let cost = self.edge_one[v1][v2] - self.edge_three[v1][v2];

                // if cost > best_cost {
                //     best_cost = cost;
                //     best = EdgeMod::Cut(v1, v2)
                // }

                // if -cost > best_cost {
                //     best_cost = -cost;
                //     best = EdgeMod::Merge(v1, v2)
                // }

                let cost = self.graph[v1][v2].weight.abs()
                    - self.edge_markers[v1][v2]
                    - self.deletion[v1][v2];
                if cost < best_cost {
                    best_cost = cost;
                    if e12 {
                        best = EdgeMod::Cut(v1, v2)
                    } else {
                        best = EdgeMod::Merge(v1, v2)
                    }
                }

                // let resolved = self.graph[v1][v2].weight.abs() - self.edge_markers[v1][v2];

                // let extra_cut;
                // let extra_merge;
                // if e12 {
                //     extra_cut = 0;
                //     extra_merge = self.edge_two[v1][v2];
                // } else {
                //     extra_cut = self.edge_two[v1][v2];
                //     extra_merge = 0;
                // }

                // if cost >= 0 {
                //     continue;
                // }
                // let mut merge_cost = cost;
                // let mut cut_cost = cost;
                // if e12 {
                //     merge_cost -= self.edge_three[v1][v2]
                // } else {
                //     cut_cost -= self.edge_one[v1][v2]
                // }

                // if extra_cut > best_cost {
                //     best_cost = extra_cut;
                //     best = EdgeMod::Merge(v1, v2)
                // }
                // if extra_merge > best_cost {
                //     best_cost = extra_merge;
                //     best = EdgeMod::Cut(v1, v2)
                // }
            }
        }
        best
    }
}
