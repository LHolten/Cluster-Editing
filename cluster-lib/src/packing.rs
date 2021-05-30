use std::cmp::min;

use crate::graph::Graph;

pub fn pack(graph: &Graph) -> u32 {
    for vertex in &graph.vertices {
        vertex.marked.set(false)
    }

    let mut cost = 0;
    for vertex in graph.clusters() {
        for edge in graph.edges(vertex).positive() {
            if edge.to >= vertex {
                break;
            }
            let (a_marked, b_marked) = (graph[vertex].marked.get(), graph[edge.to].marked.get());
            if a_marked && b_marked {
                continue;
            }

            for pair in graph.conflict_edges(vertex, edge.to) {
                if pair.to >= vertex {
                    break;
                }
                let c_marked = graph[pair.to].marked.get();

                if pair.a_weight > 0 && pair.to >= edge.to
                    || a_marked && pair.a_version == u32::MAX && c_marked
                    || b_marked && pair.b_version == u32::MAX && c_marked
                {
                    continue;
                }

                graph[vertex].marked.set(true);
                graph[edge.to].marked.set(true);
                graph[pair.to].marked.set(true);
                cost += min(min(edge.weight, pair.a_weight.abs()), pair.b_weight.abs());
                break;
            }
        }
    }
    cost as u32
}
