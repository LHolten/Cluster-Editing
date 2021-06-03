use std::cmp::min;

use crate::graph::Graph;

impl Graph {
    pub fn pack(&mut self) -> i32 {
        let self_mut = unsafe { &mut *(self as *mut Graph) };
        for (i1, v1) in self.clusters(0) {
            for (_, v2) in self.clusters(i1) {
                self_mut[v1][v2].marked = false
            }
        }

        let mut cost = 0;
        for (i1, v1) in self.clusters(0) {
            for (i2, v2) in self.positive(v1, i1) {
                if self[v1][v2].marked {
                    continue;
                }

                for pair in self.conflict_edges(v1, v2, i2) {
                    if !pair.edge1.fixed && pair.edge1.marked
                        || !pair.edge2.fixed && pair.edge2.marked
                    {
                        continue;
                    }

                    self_mut[v1][pair.to].marked = true;
                    self_mut[v2][pair.to].marked = true;
                    self_mut[v1][v2].marked = true;
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
