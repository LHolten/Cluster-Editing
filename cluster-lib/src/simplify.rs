use rand::{prelude::SliceRandom, thread_rng};

use crate::graph::Graph;

pub fn simplify(graph: &mut Graph, num_edges: usize) {
    let mut edges = Vec::new();
    for (from, vertex) in graph.vertices.iter().enumerate() {
        for edge in &vertex.edges {
            if edge.to >= from as u32 {
                break;
            }
            if edge.weight <= 0 || edge.version != u32::MAX {
                continue;
            }
            edges.push((from as u32, edge.to))
        }
    }
    let mut rng = thread_rng();
    for (v1, v2) in edges.choose_multiple(&mut rng, edges.len() - num_edges) {
        graph.cut(*v1, *v2);
    }
}
