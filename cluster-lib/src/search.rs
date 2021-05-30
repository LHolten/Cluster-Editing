use crate::{graph::Graph, packing::pack};

pub fn search_graph(graph: &mut Graph, mut upper: i32, count: &mut usize, best: &mut Graph) -> i32 {
    *count += 1;

    let lower = pack(graph);
    if lower >= upper {
        return upper;
    }
    if let Some((v1, v2)) = graph.best_edge() {
        let before = graph.clone();
        let edge = graph.cut(v1, v2);
        if edge.weight < upper {
            upper = search_graph(graph, upper - edge.weight, count, best) + edge.weight;
        }
        graph.un_cut(v1, v2, edge);
        debug_assert_eq!(graph, &before);

        let (v_merge, mut cost) = graph.merge(v1, v2);
        assert!(cost > 0);

        if cost >= upper {
            graph.un_merge(v1, v2, v_merge);
            debug_assert_eq!(graph, &before);

            return upper;
        }

        let vertices = graph.positive(v_merge).collect::<Vec<_>>();
        let mut edges = Vec::new();
        for v3 in vertices.iter().copied() {
            let before2 = graph.clone();
            let (v_merge_2, cost2) = graph.merge(v_merge, v3);
            if cost + cost2 < upper {
                upper = search_graph(graph, upper - cost - cost2, count, best) + cost + cost2;
            }
            graph.un_merge(v_merge, v3, v_merge_2);
            debug_assert_eq!(graph, &before2);

            let edge = graph.cut(v_merge, v3);
            edges.push(edge);
            cost += edge.weight;
            if cost >= upper {
                for (v3, edge) in vertices.into_iter().zip(edges.into_iter()) {
                    graph.un_cut(v_merge, v3, edge)
                }
                graph.un_merge(v1, v2, v_merge);
                debug_assert_eq!(graph, &before);

                return upper;
            }
        }

        upper = search_graph(graph, upper - cost, count, best) + cost;

        for (v3, edge) in vertices.into_iter().zip(edges.into_iter()) {
            graph.un_cut(v_merge, v3, edge)
        }
        graph.un_merge(v1, v2, v_merge);
        debug_assert_eq!(graph, &before);

        upper
    } else {
        *best = graph.clone();
        0
    }
}
