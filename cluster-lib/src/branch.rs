use crate::{
    graph::{Graph, IterSets, Vertex, VertexKey},
    merge::MergeEdges,
};

// find best edge to split on in O(n + m^2) time
fn best_edge(graph: Graph) -> Option<(VertexKey, VertexKey)> {
    let mut best = None;
    let mut best_count = 2;

    for mut i1 in IterSets::new(&graph) {
        let v1: Vertex = graph.probe_value(i1);
        for edge in &v1.edges {
            let i2: VertexKey = graph.find(edge.index);
            let v2: Vertex = graph.probe_value(i2);
            if i2 >= i1 {
                break;
            }
            if edge.count < 0 {
                continue;
            }
            let count = MergeEdges::new(&v1.edges, &v2.edges).count_diff();
            if count > best_count {
                best_count = count;
                best = Some((i1, i2))
            }
        }
    }
    best
}
