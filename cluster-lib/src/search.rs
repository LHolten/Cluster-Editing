use std::{
    cmp::min,
    mem::{swap, take},
};

use crate::{branch::EdgeMod, graph::Graph, packing::pack};

pub fn search_components(graph: &mut Graph, best: &mut Graph) -> i32 {
    let mut total = 0;
    let components = graph.components();

    let mut input = Graph::new(0);
    let mut input_ref = graph;
    let mut output = Graph::new(0);

    let mut out_clusters = Vec::new();

    for mut component in components {
        swap(&mut input_ref.clusters, &mut component);

        let edges = input_ref.edge_count();
        let max_edges = (input_ref.clusters.len() * (input_ref.clusters.len() - 1)) / 2;
        let upper = min(edges, max_edges as i32 - edges) + 1;

        total += search_graph(input_ref, upper, &mut output);

        swap(&mut input_ref.clusters, &mut component);
        out_clusters.extend(take(&mut output.clusters));

        input = output;
        input_ref = &mut input;
        output = Graph::new(0);
    }

    input.clusters = out_clusters;
    *best = input;
    total
}

pub fn search_merge(
    graph: &mut Graph,
    mut upper: i32,
    best: &mut Graph,
    v1: usize,
    v2: usize,
) -> i32 {
    let (v_merge, mut cost) = graph.merge(v1, v2);
    debug_assert!(cost > 0);

    let lower = pack(graph);
    if lower + cost >= upper {
        graph.un_merge(v1, v2, v_merge);

        return upper;
    }

    let mut vertices = graph.positive(v_merge, 0).map(|e| e.1).collect::<Vec<_>>();
    vertices.sort_unstable_by_key(|&v| -graph[v_merge][v].weight);
    let mut edges = Vec::new();
    for v3 in vertices.iter().copied() {
        let (v_merge_2, cost2) = graph.merge(v_merge, v3);
        if cost + cost2 < upper {
            upper = search_graph(graph, upper - cost - cost2, best) + cost + cost2;
        }
        graph.un_merge(v_merge, v3, v_merge_2);

        let edge = graph.cut(v_merge, v3);
        cost += edge.weight;
        edges.push(edge);

        let lower = pack(graph);
        if lower + cost >= upper {
            for (v3, edge) in vertices.into_iter().zip(edges.into_iter()) {
                graph.un_cut(v_merge, v3, edge)
            }
            graph.un_merge(v1, v2, v_merge);

            return upper;
        }
    }

    upper = search_graph(graph, upper - cost, best) + cost;

    for (v3, edge) in vertices.into_iter().zip(edges.into_iter()) {
        graph.un_cut(v_merge, v3, edge)
    }
    graph.un_merge(v1, v2, v_merge);

    upper
}

pub fn search_cut(
    graph: &mut Graph,
    mut upper: i32,
    best: &mut Graph,
    v1: usize,
    v2: usize,
) -> i32 {
    let edge = graph.cut(v1, v2);
    if edge.weight < upper {
        upper = search_graph(graph, upper - edge.weight, best) + edge.weight;
    }
    graph.un_cut(v1, v2, edge);

    upper
}

pub fn search_graph(graph: &mut Graph, mut upper: i32, best: &mut Graph) -> i32 {
    let lower = pack(graph);
    if lower >= upper {
        return upper;
    }
    match graph.best_edge() {
        EdgeMod::Merge(v1, v2) => {
            upper = search_merge(graph, upper, best, v1, v2);
            search_cut(graph, upper, best, v1, v2)
        }
        EdgeMod::Delete(v1, v2) => {
            upper = search_cut(graph, upper, best, v1, v2);
            search_merge(graph, upper, best, v1, v2)
        }
        EdgeMod::Nothing => {
            *best = graph.clone();
            lower
        }
    }
}
