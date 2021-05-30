// use rand::{prelude::SliceRandom, thread_rng};

// use crate::graph::Graph;

// pub fn simplify(graph: &mut Graph, num_edges: usize) {
//     let mut edges = Vec::new();
//     for vertex in graph.clusters() {
//         for edge in graph.edges(vertex).positive() {
//             if edge.to >= vertex {
//                 break;
//             }
//             edges.push((vertex, edge.to))
//         }
//     }
//     let mut rng = thread_rng();
//     for (v1, v2) in edges.choose_multiple(&mut rng, edges.len() - num_edges) {
//         graph.cut(*v1, *v2);
//     }
// }
