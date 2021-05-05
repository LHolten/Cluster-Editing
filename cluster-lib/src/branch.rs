use crate::{
    graph::{EdgeIter, Graph, SetIter},
    merge::MergeEdges,
};

// find best edge to split on in O(n + m^2) time
fn best_edge(graph: Graph) -> Option<(u32, u32)> {
    let mut best = None;
    let mut best_count = 2;

    for v1 in SetIter::new(&graph) {
        for v2 in EdgeIter::new(&graph, &v1) {
            let count = MergeEdges::new(&v1.edges, &v2.edges).count_diff();
            if count > best_count {
                best_count = count;
                best = Some((v1.index, v2.index))
            }
        }
    }
    best
}
