use std::cmp::min;

use crate::search::Solver;

impl Solver {
    pub fn pack(&mut self) -> i32 {
        for (i1, v1) in self.graph.all(0) {
            for (_, v2) in self.graph.all(i1) {
                self.edge_markers[v1][v2] = 0
            }
        }

        let mut cost = 0;
        for (i1, v1) in self.graph.all(0) {
            for (i2, v2) in self.graph.positive(v1, i1) {
                if self.graph[v1][v2].weight == self.edge_markers[v1][v2] {
                    continue;
                }

                for (_, v3) in self.graph.conflict_edges(v1, v2, i2) {
                    let new_cost = min(
                        self.graph[v1][v2].weight - self.edge_markers[v1][v2],
                        min(
                            self.graph[v1][v3].weight.abs() - self.edge_markers[v1][v3],
                            self.graph[v2][v3].weight.abs() - self.edge_markers[v2][v3],
                        ),
                    );
                    self.edge_markers[v1][v3] += new_cost;
                    self.edge_markers[v2][v3] += new_cost;
                    self.edge_markers[v1][v2] += new_cost;
                    cost += new_cost;

                    if self.graph[v1][v2].weight == self.edge_markers[v1][v2] {
                        break;
                    }
                }
            }
        }

        cost
    }
}
