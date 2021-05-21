use crate::graph::Graph;

pub fn critical(graph: &mut Graph) -> u32 {
    let mut cost = 0;
    for mut vertex in graph.clusters().collect::<Vec<_>>() {
        if graph[vertex].merged.is_some() {
            continue;
        }
        // let mut same = Vec::new();
        for edge in graph.edges(vertex).positive().cloned().collect::<Vec<_>>() {
            if edge.to >= vertex {
                break;
            }
            if graph.conflict_edges(vertex, edge.to).next().is_none() {
                cost += graph.merge_cost(vertex, edge.to);
                vertex = graph.merge(vertex, edge.to);
                // same.push(edge.to);
            }
        }
        // for vv in same.chunks_exact(2) {
        //     cost += graph.merge_cost(vertex, vv[0]);
        //     vertex = graph.merge(vertex, vv[0]);
        //     cost += graph.merge_cost(vertex, vv[1]);
        //     vertex = graph.merge(vertex, vv[1]);
        // }
    }
    cost
}

pub fn propagate(graph: &mut Graph) {
    for vertex in graph.clusters().collect::<Vec<_>>() {
        for edge in graph.edges(vertex).negative().cloned().collect::<Vec<_>>() {
            if edge.to >= vertex {
                break;
            }

            if graph.merge_edges(vertex, edge.to).count() <= 1 {
                graph.cut(vertex, edge.to);
            }

            // if -edge.weight as u32 >= graph.cut_cost(vertex, edge.to) {
            //     graph.cut(vertex, edge.to);
            // }
        }
    }
}
