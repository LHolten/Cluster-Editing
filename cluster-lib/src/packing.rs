use std::cmp::min;

use crate::graph::Graph;

pub fn pack(graph: &Graph) -> u32 {
    for vertex in &graph.vertices {
        for edge in &vertex.edges {
            edge.marked.set(false);
        }
    }

    let mut cost = 0;
    for vertex in graph.clusters() {
        for edge in graph.edges(vertex) {
            if edge.to >= vertex {
                break;
            }
            if edge.marked.get() {
                continue;
            }
            let iter = if edge.version == u32::MAX && edge.weight > 0 {
                graph
                    .merge_edges(vertex, edge.to)
                    .conflicts()
                    .collect::<Vec<_>>()
            } else {
                graph
                    .merge_edges(vertex, edge.to)
                    .two_edges()
                    .collect::<Vec<_>>()
            };
            for (a, b) in iter {
                if a.to >= edge.to {
                    break;
                }
                if a.marked.get() || b.marked.get() {
                    continue;
                }
                edge.marked.set(edge.version == u32::MAX);
                a.marked.set(a.version == u32::MAX);
                b.marked.set(b.version == u32::MAX);

                cost += min(edge.weight.abs(), min(a.weight.abs(), b.weight.abs()));
                break;
            }
        }
    }
    cost as u32
}
