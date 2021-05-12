use crate::graph::Graph;

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
                let (new_vertex, new_cost) = graph.merge(vertex, edge.to);
                vertex = new_vertex;
                cost += new_cost;
            }
        }
    }
    cost
}
