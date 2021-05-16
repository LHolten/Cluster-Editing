use crate::{graph::Graph, packing::pack};

pub fn search_graph(graph: &mut Graph, mut upper: u32) -> u32 {
    // let lower = pack(graph);
    // if lower >= upper {
    //     return upper;
    // }
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
        println!("improved by: {}", upper);
        0
    }
}

pub fn search_graph_2(graph: &mut Graph, mut upper: u32) -> u32 {
    // let lower = pack(graph);
    // if lower >= upper {
    //     return upper;
    // }
    if let Some((v1, v2, v3)) = graph.triple() {
        graph.snapshot();
        let cost1 = graph.cut(v1, v3);
        debug_assert!(cost1 > 0);
        if cost1 < upper {
            upper = search_graph_2(graph, upper - cost1) + cost1;
        }
        graph.rollback();
        graph.snapshot();
        let cost2 = graph.cut(v2, v3);
        debug_assert!(cost2 > 0);
        if cost2 < upper {
            upper = search_graph_2(graph, upper - cost2) + cost2;
        }
        graph.rollback();
        let cost3 = graph.merge(v1, v2);
        debug_assert!(cost3 > 0);
        if cost3 < upper {
            upper = search_graph_2(graph, upper - cost3) + cost3;
        }
        upper
    } else {
        println!("improved by: {}", upper);
        0
    }
}
