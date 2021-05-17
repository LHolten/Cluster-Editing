use std::fs::File;

use crate::disk::write;
use crate::{graph::Graph, packing::pack};

pub fn search_graph(graph: &mut Graph, mut upper: u32, count: &mut usize) -> u32 {
    *count += 1;
    let lower = pack(graph);
    if lower >= upper {
        return upper;
    }
    if let Some((v1, v2)) = graph.best_edge() {
        graph.snapshot();
        let cost1 = graph.cut(v1, v2);
        if cost1 < upper {
            upper = search_graph(graph, upper - cost1, count) + cost1;
        }
        graph.rollback();
        let mut cost = graph.merge_cost(v1, v2);
        let v3 = graph.merge(v1, v2);
        assert!(cost > 0);

        if cost >= upper {
            return upper;
        }

        for edge in graph.edges(v3).positive().cloned().collect::<Vec<_>>() {
            graph.snapshot();
            let cost2 = graph.merge_cost(v3, edge.to);
            if cost + cost2 < upper {
                graph.merge(v3, edge.to);
                upper = search_graph(graph, upper - cost - cost2, count) + cost + cost2;
            }
            graph.rollback();
            cost += graph.cut(v3, edge.to);

            if cost >= upper {
                return upper;
            }
        }

        search_graph(graph, upper - cost, count) + cost
    } else {
        // println!("improved by: {}", upper);
        0
    }
}

// pub fn search_graph_2(graph: &mut Graph, mut upper: u32, count: &mut usize) -> u32 {
//     *count += 1;
//     let lower = pack(graph);
//     if lower >= upper {
//         return upper;
//     }
//     if let Some((v1, v2)) = graph.best_edge() {
//         graph.snapshot();
//         let cost1 = graph.cut(v1, v2);
//         assert!(cost1 > 0);
//         if cost1 < upper {
//             upper = search_graph_2(graph, upper - cost1, count) + cost1;
//         }
//         graph.rollback();
//         let mut cost = graph.merge_cost(v1, v2);
//         let v3 = graph.merge(v1, v2);
//         // assert!(cost > 0);

//         if cost >= upper {
//             return upper;
//         }

//         search_graph_2(graph, upper - cost, count) + cost
//     } else {
//         // println!("improved by: {}", upper);
//         0
//     }
// }

pub fn search_graph_2(graph: &mut Graph, mut upper: u32, count: &mut usize) -> u32 {
    assert_ne!(upper, 0);
    *count += 1;
    let lower = pack(graph);
    if lower >= upper {
        return upper;
    }
    if let Some((v1, v2, v3, weight)) = graph.triple() {
        if weight != -i32::MAX {
            graph.snapshot();
            let mut cost3 = weight.abs() as u32;
            debug_assert!(cost3 > 0);
            cost3 += graph.merge_cost(v1, v3);
            let v13 = graph.merge(v1, v3);
            cost3 += graph.merge_cost(v13, v2);
            graph.merge(v13, v2);
            if cost3 < upper {
                upper = search_graph_2(graph, upper - cost3, count) + cost3;
            }
            graph.rollback();
        }

        graph.snapshot();
        let cost1 = graph.cut(v1, v2);
        if cost1 < upper {
            upper = search_graph_2(graph, upper - cost1, count) + cost1;
        }
        graph.rollback();

        graph.snapshot();
        let cost2 = graph.cut(v2, v3);
        if cost2 < upper {
            upper = search_graph_2(graph, upper - cost2, count) + cost2;
        }
        graph.rollback();

        upper
    } else {
        0
    }
}
