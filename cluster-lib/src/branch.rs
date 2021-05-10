use crate::{
    graph::{EdgeIter, Graph},
    merge::MergeEdges,
};

// find best edge to split on in O(n + m^2) time
fn best_edge(graph: &mut Graph) -> Option<(u32, u32)> {
    let mut best = None;
    let mut best_count = 2;

    for vertex in graph.clusters() {
        for edge in graph.edges(vertex) {
            let count = graph.merge_edges(vertex, edge.to).count_diff();
            if count > best_count {
                best_count = count;
                best = Some((vertex, edge.to))
            }
        }
    }
    best
}
