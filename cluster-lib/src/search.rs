use crate::{graph::Graph, packing::pack};

pub fn search_graph(graph: &mut Graph, mut upper: u32) -> u32 {
    let lower = pack(graph);
    if lower >= upper {
        return upper;
    }
    if let Some((v1, v2)) = graph.best_edge() {
        graph.snapshot();
        let cost1 = graph.cut(v1, v2);
        debug_assert!(cost1 > 0);
        if cost1 < upper {
            upper = search_graph(graph, upper - cost1) + cost1;
        }
        graph.rollback();
        let mut cost = graph.merge_cost(v1, v2);
        let v3 = graph.merge(v1, v2);
        debug_assert!(cost > 0);

        if cost >= upper {
            return upper;
        }

        for edge in graph.edges(v3).positive().cloned().collect::<Vec<_>>() {
            graph.snapshot();
            let cost2 = graph.merge_cost(v3, edge.to);
            if cost + cost2 < upper {
                graph.merge(v3, edge.to);
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
