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

            for (a, b) in graph.conflict_edges(vertex, edge.to) {
                let edge2_to = a.or(b).unwrap().to;
                if edge2_to >= vertex {
                    break;
                }
                let c_marked = graph[edge2_to].marked.get();

                if a.is_some() && edge2_to >= edge.to
                    || a_marked && a.is_some() && c_marked
                    || b_marked && b.is_some() && c_marked
                {
                    continue;
                }

                graph[vertex].marked.set(true);
                graph[edge.to].marked.set(true);
                graph[edge2_to].marked.set(true);
                let mut new_cost = edge.weight;
                if let Some(a) = a {
                    new_cost = min(new_cost, a.weight.abs());
                }
                if let Some(b) = b {
                    new_cost = min(new_cost, b.weight.abs());
                }

                cost += new_cost;
                break;
            }
        }
    }
    cost as u32
}
