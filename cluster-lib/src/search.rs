use std::mem::swap;

use crate::{graph::Graph, packing::pack};

pub fn search_components(graph: &mut Graph, upper: i32, best: &mut Graph) -> i32 {
    let mut components = graph.components();
    let mut bounds = Vec::with_capacity(components.len());
    for c in &mut components {
        swap(c, &mut graph.clusters);
        bounds.push(pack(graph));
        swap(c, &mut graph.clusters);
    }
    let mut lower = bounds.iter().sum();

    if lower >= upper {
        return upper;
    }

    if components.len() == 1 {
        return search_graph(graph, upper, best);
    }

    let mut input = Graph::new(0);
    let mut input_ref = graph;
    let mut output = Graph::new(0);
    for (c, l) in components.iter_mut().zip(bounds.into_iter()) {
        lower -= l;

        swap(c, &mut input_ref.clusters);
        lower += search_graph(input_ref, upper - lower, &mut output);
        swap(c, &mut input_ref.clusters);

        if lower >= upper {
            return upper;
        }
        input = output;
        input_ref = &mut input;
        output = Graph::new(0);
    }

    *best = input;
    lower
}

pub fn search_one(graph: &mut Graph, upper: i32, best: &mut Graph) -> i32 {
    let lower = pack(graph);
    if lower >= upper {
        return upper;
    }
    search_graph(graph, upper, best)
}

pub fn search_graph(graph: &mut Graph, mut upper: i32, best: &mut Graph) -> i32 {
    if let Some((v1, v2)) = graph.best_edge() {
        let edge = graph.cut(v1, v2);
        if edge.weight < upper {
            upper = search_components(graph, upper - edge.weight, best) + edge.weight;
        }
        graph.un_cut(v1, v2, edge);

        let (v_merge, mut cost) = graph.merge(v1, v2);
        assert!(cost > 0);

        if cost >= upper {
            graph.un_merge(v1, v2, v_merge);

            return upper;
        }

        let mut vertices = graph.positive(v_merge, 0).map(|e| e.1).collect::<Vec<_>>();
        vertices.sort_unstable_by_key(|&v| -graph[v_merge][v].weight);
        let mut edges = Vec::new();
        for v3 in vertices.iter().copied() {
            let (v_merge_2, cost2) = graph.merge(v_merge, v3);
            if cost + cost2 < upper {
                upper = search_one(graph, upper - cost - cost2, best) + cost + cost2;
            }
            graph.un_merge(v_merge, v3, v_merge_2);

            let edge = graph.cut(v_merge, v3);
            cost += edge.weight;
            edges.push(edge);
            if cost >= upper {
                for (v3, edge) in vertices.into_iter().zip(edges.into_iter()) {
                    graph.un_cut(v_merge, v3, edge)
                }
                graph.un_merge(v1, v2, v_merge);

                return upper;
            }
        }

        upper = search_one(graph, upper - cost, best) + cost;

        for (v3, edge) in vertices.into_iter().zip(edges.into_iter()) {
            graph.un_cut(v_merge, v3, edge)
        }
        graph.un_merge(v1, v2, v_merge);

        upper
    } else {
        *best = graph.clone();
        0
    }
}
