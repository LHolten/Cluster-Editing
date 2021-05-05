use crate::{graph::Graph, merge::MergeEdges};

// find best edge to split on in O(n + m^2) time
fn best_edge(graph: Graph) -> Option<(u32, u32)> {
    let mut best = None;
    let mut best_count = 2;

    for mut this in graph.all_sets() {
        let (i1, v1) = this.next().unwrap();
        for edge in &v1.edges {
            let (i2, v2) = graph.set(edge.index as usize).next().unwrap();
            if i2 >= i1 {
                break;
            }
            if edge.count < 0 {
                break;
            }
            let count = MergeEdges::new(&v1.edges, &v2.edges).count_diff();
            if count > best_count {
                best_count = count;
                best = Some((i1 as u32, i2 as u32))
            }
        }
    }
    best
}
