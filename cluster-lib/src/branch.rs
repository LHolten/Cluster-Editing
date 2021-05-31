use crate::graph::Graph;

impl Graph {
    // find best edge to split on in O(n + m^2) time
    // edge is positive
    // there is at least one conflict
    pub fn best_edge(&mut self) -> Option<(usize, usize)> {
        let mut best = None;
        let mut best_count = 2;

        for (i1, v1) in self.clusters(0) {
            for (_, v2) in self.positive(v1, i1) {
                // let count = self.merge_cost(vertex, edge.to);
                let mut count = 0;
                for pair in self.all_edges(v1, v2, 0) {
                    count += ((pair.edge1.weight > 0) ^ (pair.edge2.weight > 0)) as u32;
                }
                // if count >= 2 {
                //     count = max(count, edge.weight as u32);
                // }
                if count > best_count {
                    best_count = count;
                    best = Some((v1, v2))
                }
            }
        }
        best
    }
}
