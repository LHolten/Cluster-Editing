use crate::{critical::MergeEdges, graph::Graph};

impl Graph {
    // find best edge to split on in O(n + m^2) time
    fn best_edge(&self) -> Option<(u32, u32)> {
        let mut best = None;
        let mut best_count = 2;
        for (vertex_id, vertex) in self.0.iter().enumerate() {
            for edge in vertex.edges {
                if edge.index > vertex_id as u32 {
                    break;
                }
                // todo need to check that the edge is a real edge
                let vertex2 = &self.0[edge.index as usize];
                let count = MergeEdges::new(&vertex.edges, &vertex2.edges).count_diff();
                if count > best_count {
                    best_count = count;
                    best = Some((vertex_id as u32, edge.index))
                }
            }
        }
        best
    }
}
