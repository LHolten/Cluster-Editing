use std::cmp::min;

use crate::search::Solver;

impl Solver {
    pub fn pack(&mut self) -> i32 {
        for (i1, v1) in self.graph.all(0) {
            for (_, v2) in self.graph.all(i1) {
                self.edge_markers[v1][v2] = false
            }
        }

        let mut cost = 0;
        for (i1, v1) in self.graph.all(0) {
            for (i2, v2) in self.graph.positive(v1, i1) {
                if self.edge_markers[v1][v2] {
                    continue;
                }

                for (_, v3) in self.graph.conflict_edges(v1, v2, i2) {
                    if !self.graph[v1][v3].fixed && self.edge_markers[v1][v3]
                        || !self.graph[v2][v3].fixed && self.edge_markers[v2][v3]
                    {
                        continue;
                    }

                    self.edge_markers[v1][v3] = true;
                    self.edge_markers[v2][v3] = true;
                    self.edge_markers[v1][v2] = true;
                    cost += min(
                        self.graph[v1][v2].weight,
                        min(
                            self.graph[v1][v3].weight.abs(),
                            self.graph[v2][v3].weight.abs(),
                        ),
                    );
                    break;
                }
            }
        }

        cost
    }
}
