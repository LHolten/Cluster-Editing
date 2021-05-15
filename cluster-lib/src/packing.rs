use crate::graph::Graph;

pub fn pack(graph: &mut Graph) {
    for vertex in graph.clusters() {
        for edge in graph.edges(vertex) {
            if edge.to >= vertex {
                break;
            }
            for (a, b) in graph.merge_edges(vertex, edge.to).conflicts() {
                if a.to >= edge.to {
                    break;
                }
                todo!()
            }
        }
    }
}
