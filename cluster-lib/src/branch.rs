use crate::graph::Graph;

impl Graph {
    // find best edge to split on in O(n + m^2) time
    // edge is positive
    // there is at least one conflict
    pub fn best_edge(&mut self) -> Option<(usize, usize)> {
        let mut best = None;
        let mut best_count = 2;

        for vertex in self.clusters.iter() {
            for vertex2 in self.positive(vertex) {
                if vertex2 >= vertex {
                    break;
                }

                // let count = self.merge_cost(vertex, edge.to);
                let count = self.conflict_edges(vertex, vertex2).count();
                // if count >= 2 {
                //     count = max(count, edge.weight as u32);
                // }
                if count > best_count {
                    best_count = count;
                    best = Some((vertex, vertex2))
                }
            }
        }
        best
    }
}
