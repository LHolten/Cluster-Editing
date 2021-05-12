use crate::graph::{Edge, Graph};

pub fn search_graph(graph: &mut Graph, mut upper: u32) -> u32 {
    if let Some((v1, v2)) = graph.best_edge() {
        graph.snapshot();
        let cost = graph.cut(v1, v2);
        if cost < upper {
            upper = search_graph(graph, upper - cost) + cost;
        }
        graph.rollback();
        let (mut cost, v3) = graph.merge(v1, v2);
        debug_assert!(cost > 0);

        if cost >= upper {
            return upper;
        }

        for edge in graph.edges(v3).collect::<Vec<Edge>>() {
            if edge.weight <= 0 || edge.version != u32::MAX {
                continue;
            }

            graph.snapshot();
            let (cost2, _) = graph.merge(v3, edge.to);
            if cost + cost2 < upper {
                upper = search_graph(graph, upper - cost - cost2) + cost + cost2;
            }
            graph.rollback();
            cost += graph.cut(v3, edge.to);

            if cost >= upper {
                return upper;
            }
        }

        search_graph(graph, upper - cost) + cost
    } else {
        0
    }
}
