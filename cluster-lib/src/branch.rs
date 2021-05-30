use crate::graph::{Graph, VertexIndex};

impl Graph {
    // find best edge to split on in O(n + m^2) time
    // edge is positive
    // there is at least one conflict
    pub fn best_edge(&mut self) -> Option<(VertexIndex, VertexIndex)> {
        let mut best = None;
        let mut best_count = 0;

        for vertex in self.clusters() {
            for edge in self.edges(vertex).positive() {
                if edge.to >= vertex {
                    break;
                }

                // let count = self.merge_cost(vertex, edge.to);
                let count = self.conflict_edges(vertex, edge.to).count();
                // if count >= 2 {
                //     count = max(count, edge.weight as u32);
                // }
                if count > best_count {
                    best_count = count;
                    best = Some((vertex, edge.to))
                }
            }
        }
        best
    }
}
