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

                if pair.a_weight.is_some() && pair.to >= edge.to
                    || a_marked && pair.a_weight.is_some() && c_marked
                    || b_marked && pair.b_weight.is_some() && c_marked
                {
                    continue;
                }

                graph[vertex].marked.set(true);
                graph[edge.to].marked.set(true);
                graph[pair.to].marked.set(true);
                let mut new_cost = edge.weight;
                if let Some(a_weight) = pair.a_weight {
                    new_cost = min(new_cost, a_weight.abs());
                }
                if let Some(b_weight) = pair.b_weight {
                    new_cost = min(new_cost, b_weight.abs());
                }

                cost += new_cost;
                break;
            }
        }
    }
    cost as u32
}
