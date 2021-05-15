use std::mem::swap;

use crate::graph::Graph;

pub fn kernelize(graph: &mut Graph) -> u32 {
    let mut cost = 0;
    for vertex in 0..graph.vertices.len() as u32 {
        if graph[vertex].merged.is_some() {
            // also possible to merge only with vertices before
            continue;
        }
        let inner = graph.edges(vertex).positive().cloned().collect::<Vec<_>>();
        let mut rho = 0;
        for edge in &inner {
            rho += graph.merge_rho(vertex, edge.to)
        }
        if rho <= inner.len() as u32 {
            for edge in inner {
                cost += graph.merge_cost(vertex, edge.to);
                graph.merge(vertex, edge.to);
            }
        }
    }
    cost
}

// only works on graph with no merged vertices
pub fn kernel2(graph: &mut Graph) -> u32 {
    let mut cost = 0;
    loop {
        let mut new_cost = 0;
        for vertex in 0..graph.vertices.len() as u32 {
            for edge in graph.edges(vertex).positive().cloned().collect::<Vec<_>>() {
                let mut v1 = vertex;
                let mut v2 = edge.to;
                let conflicts = graph.merge_edges(v1, v2).conflicts().collect::<Vec<_>>();
                if conflicts.len() == 1 {
                    let (mut a, mut b) = conflicts[0];
                    if a.weight > 0 && a.version == u32::MAX {
                        swap(&mut v1, &mut v2);
                        swap(&mut a, &mut b);
                    }
                    let v3 = a.to;
                    // now we have v1 -- v2 -- v3

                    let conflict_count = graph.merge_edges(v1, v3).conflicts().count();
                    if conflict_count == 0 {
                        debug_assert!(a.weight < 0 && a.version == u32::MAX);
                        new_cost += a.weight.abs() as u32;
                        graph.merge(v1, v3);
                    } else if conflict_count
                        == graph.edges(v1).positive().count() + graph.edges(v3).positive().count()
                            - 2
                    {
                        new_cost += graph.cut(v2, v3);
                    }
                }
            }
        }
        cost += new_cost;
        if new_cost == 0 {
            break;
        }
    }
    cost
}
