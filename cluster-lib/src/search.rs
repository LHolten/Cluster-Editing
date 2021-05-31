use crate::{graph::Graph, packing::pack};

pub fn search_graph(graph: &mut Graph, mut upper: i32, best: &mut Graph) -> i32 {
    let lower = pack(graph);
    if lower >= upper {
        return upper;
    }
    if let Some((v1, v2)) = graph.best_edge() {
        let edge = graph.cut(v1, v2);
        if edge.weight < upper {
            upper = search_graph(graph, upper - edge.weight, best) + edge.weight;
        }
        graph.un_cut(v1, v2, edge);

        let (v_merge, mut cost) = graph.merge(v1, v2);
        assert!(cost > 0);

        if cost >= upper {
            graph.un_merge(v1, v2, v_merge);

            return upper;
        }

        let vertices = graph.positive(v_merge, 0).collect::<Vec<_>>();
        let mut edges = Vec::new();
        for (_, v3) in vertices.iter().copied() {
            let (v_merge_2, cost2) = graph.merge(v_merge, v3);
            if cost + cost2 < upper {
                upper = search_graph(graph, upper - cost - cost2, best) + cost + cost2;
            }
            graph.un_merge(v_merge, v3, v_merge_2);

            let edge = graph.cut(v_merge, v3);
            edges.push(edge);
            cost += edge.weight;
            if cost >= upper {
                for ((_, v3), edge) in vertices.into_iter().zip(edges.into_iter()) {
                    graph.un_cut(v_merge, v3, edge)
                }
                graph.un_merge(v1, v2, v_merge);

                return upper;
            }
        }

        upper = search_graph(graph, upper - cost, best) + cost;

        for ((_, v3), edge) in vertices.into_iter().zip(edges.into_iter()) {
            graph.un_cut(v_merge, v3, edge)
        }
        graph.un_merge(v1, v2, v_merge);

        upper
    } else {
        *best = graph.clone();
        0
    }
}
