use std::cmp::min;

use crate::graph::Graph;

impl Graph {
    pub fn pack(&self) -> i32 {
        for (i1, v1) in self.clusters(0) {
            for (_, v2) in self.clusters(i1) {
                self[v1][v2].marked.set(false)
            }
        }

        let mut cost = 0;
        for (i1, v1) in self.clusters(0) {
            for (i2, v2) in self.positive(v1, i1) {
                if self[v1][v2].marked.get() {
                    continue;
                }

                for pair in self.conflict_edges(v1, v2, i2) {
                    if !pair.edge1.fixed && pair.edge1.marked.get()
                        || !pair.edge2.fixed && pair.edge2.marked.get()
                    {
                        continue;
                    }

                    pair.edge1.marked.set(true);
                    pair.edge2.marked.set(true);
                    self[v1][v2].marked.set(true);
                    cost += min(
                        self[v1][v2].weight,
                        min(pair.edge1.weight.abs(), pair.edge2.weight.abs()),
                    );
                    break;
                }
            }
        }

        cost
    }
}
