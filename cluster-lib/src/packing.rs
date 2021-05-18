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
        for edge in graph.edges(vertex).positive() {
            if edge.to >= vertex {
                break;
            }
            if edge.marked.get() {
                continue;
            }

            for (a, b) in graph.conflict_edges(vertex, edge.to) {
                if a.or(b).unwrap().to >= vertex {
                    break;
                }
                if a.map(|e| e.to >= edge.to).unwrap_or(false)
                    || a.map(|e| e.marked.get()).unwrap_or(false)
                    || b.map(|e| e.marked.get()).unwrap_or(false)
                {
                    continue;
                }

                edge.marked.set(true);
                let mut new_cost = edge.weight;
                if let Some(a) = a {
                    a.marked.set(true);
                    new_cost = min(new_cost, a.weight.abs());
                }
                if let Some(b) = b {
                    b.marked.set(true);
                    new_cost = min(new_cost, b.weight.abs());
                }

                cost += new_cost;
                break;
            }
        }
    }
    cost as u32
}
