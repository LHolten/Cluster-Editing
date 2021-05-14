use crate::graph::Graph;

pub fn critical(graph: &mut Graph) -> u32 {
    let mut cost = 0;
    for mut vertex in graph.clusters().collect::<Vec<_>>() {
        for edge in graph.edges(vertex).positive().collect::<Vec<_>>() {
            if edge.to > vertex {
                break;
            }
            if graph.conflict_edges(vertex, edge.to).count() == 0 {
                let (new_cost, new_vertex) = graph.merge(vertex, edge.to);
                cost += new_cost;
                vertex = new_vertex;
            }
        }
    }
    cost
}

pub fn propagate(graph: &mut Graph, upper: u32) -> u32 {
    let mut cost = 0;
    for vertex in graph.clusters().collect::<Vec<_>>() {
        for edge in graph.edges(vertex).positive().collect::<Vec<_>>() {
            if edge.to > vertex {
                break;
            }
            let cost2 = graph.merge_cost(vertex, edge.to);

            if cost + cost2 >= upper {
                cost += graph.cut(vertex, edge.to);
                if cost >= upper {
                    return upper;
                }
            }
        }
    }
    cost
}
