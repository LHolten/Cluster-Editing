use crate::graph::{Edge, Graph};

pub fn kernelize(graph: &mut Graph) -> u32 {
    let mut cost = 0;
    for mut vertex in 0..graph.vertices.len() as u32 {
        if graph[vertex].merged.is_some() {
            // also possible to merge only with vertices before
            continue;
        }
        let inner = graph.edges(vertex).positive().collect::<Vec<_>>();
        let mut rho = 0;
        for edge in &inner {
            rho += graph.merge_cost(vertex, edge.to)
        }
        if rho <= inner.len() as u32 {
            for edge in inner {
                let (new_cost, new_vertex) = graph.merge(vertex, edge.to);
                vertex = new_vertex;
                cost += new_cost;
            }
        }
    }
    cost
}

pub fn kernel2(graph: &mut Graph) -> u32 {
    let mut cost = 0;
    for mut vertex in 0..graph.vertices.len() as u32 {
        for edge in graph.edges(vertex).collect::<Vec<_>>() {
            let conflicts = graph.conflict_edges(vertex, edge.to).collect::<Vec<_>>();
            if conflicts.len() == 1 {
                let (v1, v2) = conflicts[0];
                let v3 = v1.or(v2).unwrap().to;
                let conflict_count = graph.conflict_edges(vertex, v3).count();
                if conflict_count == 0 {
                    graph[vertex].edges.push(Edge::new(v3));
                    graph[vertex].edges.sort_by_key(|e| e.to);
                    graph[v3].edges.push(Edge::new(vertex));
                    graph[v3].edges.sort_by_key(|e| e.to);
                    cost += 1;
                } else if conflict_count == graph.edges(vertex).count() {
                    // graph.cut(vertex, v2)
                }
            }
        }
    }
    cost
}
