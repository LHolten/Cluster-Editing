use crate::graph::Graph;

impl Graph {
    // find best edge to split on in O(n + m^2) time
    pub fn best_edge(&mut self) -> Option<(u32, u32)> {
        let mut best = None;
        let mut best_count = 0;

        for vertex in self.clusters() {
            for edge in self.edges(vertex) {
                let count = self.merge_edges(vertex, edge.to).count_diff();
                if count > best_count {
                    best_count = count;
                    best = Some((vertex, edge.to))
                }
            }
        }
        best
    }
}
