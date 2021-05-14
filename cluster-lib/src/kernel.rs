use std::mem::swap;

use crate::graph::{Edge, Graph};

pub fn kernelize(graph: &mut Graph) -> u32 {
    let mut cost = 0;
    for vertex in 0..graph.vertices.len() as u32 {
        if graph[vertex].merged.is_some() {
            // also possible to merge only with vertices before
            continue;
        }
        let inner = graph.edges(vertex).positive().collect::<Vec<_>>();
        let mut rho = 0;
        for edge in &inner {
            rho += graph.merge_rho(vertex, edge.to)
        }
        if rho <= inner.len() as u32 {
            for edge in inner {
                cost += graph.merge(vertex, edge.to).0;
            }
        }
    }
    cost
}

// only works on graph with no merged vertices
pub fn kernel2(graph: &mut Graph) -> u32 {
    let mut cost = 0;
    for vertex in 0..graph.vertices.len() as u32 {
        for edge in graph.edges(vertex).positive().collect::<Vec<_>>() {
            let mut v1 = vertex;
            let mut v2 = edge.to;
            let conflicts = graph.conflict_edges(v1, v2).collect::<Vec<_>>();
            if conflicts.len() == 1 {
                let (a, b) = conflicts[0];
                if a.is_some() {
                    swap(&mut v1, &mut v2);
                }
                let v3 = a.or(b).unwrap().to;
                // now we have v1 -- v2 -- v3

                let conflict_count = graph.conflict_edges(v1, v3).count();
                if conflict_count == 0 {
                    graph[v1].edges.push(Edge::new(v3));
                    graph[v1].edges.sort_by_key(|e| e.to);
                    graph[v3].edges.push(Edge::new(v1));
                    graph[v3].edges.sort_by_key(|e| e.to);
                    cost += 1;
                } else if conflict_count == graph.edges(v3).count() - 1 {
                    cost += graph.cut(v2, v3);
                }
            }
        }
    }
    cost
}
