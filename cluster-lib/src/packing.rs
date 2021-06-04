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
            for (i2, v2) in self.graph.all(i1) {
                if self.graph[v1][v2].weight.abs() == self.edge_markers[v1][v2] {
                    continue;
                }

                for (_, v3) in self.graph.all(i2) {
                    let e12 = self.graph[v1][v2].weight > 0;
                    let e13 = self.graph[v1][v3].weight > 0;
                    let e23 = self.graph[v2][v3].weight > 0;
                    if !(e12 | e13 | e23) | (e12 ^ e13 ^ e23) {
                        continue;
                    }

                    let new_cost = min(
                        self.graph[v1][v2].weight.abs() - self.edge_markers[v1][v2],
                        min(
                            self.graph[v1][v3].weight.abs() - self.edge_markers[v1][v3],
                            self.graph[v2][v3].weight.abs() - self.edge_markers[v2][v3],
                        ),
                    );
                    self.edge_markers[v1][v3] += new_cost;
                    self.edge_markers[v2][v3] += new_cost;
                    self.edge_markers[v1][v2] += new_cost;
                    cost += new_cost;

                    if self.graph[v1][v2].weight.abs() == self.edge_markers[v1][v2] {
                        break;
                    }
                }
            }
        }

        cost
    }
}
