use crate::graph::Graph;

impl Graph {
    // find best edge to split on in O(n + m^2) time
    // edge is positive
    // there is at least one conflict
    pub fn best_edge(&mut self) -> Option<(u32, u32)> {
        let mut best = None;
        let mut best_count = 0;

        for vertex in self.clusters() {
            for edge in self.edges(vertex) {
                if edge.to >= vertex {
                    break;
                }
                if edge.weight <= 0 || edge.version != u32::MAX {
                    continue;
                }

                let count = self.merge_cost(vertex, edge.to);
                if count > best_count {
                    best_count = count;
                    best = Some((vertex, edge.to))
                }
            }
        }
        best
    }
}
