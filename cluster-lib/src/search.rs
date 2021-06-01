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
    let (v_merge, cost) = graph.merge(v1, v2);
    debug_assert!(cost > 0);

    let lower = pack(graph);
    if lower + cost < upper {
        let im_graph = unsafe { &*(graph as *const Graph) };
        graph.clusters.sort_unstable_by_key(|&v| {
            let mut count = 0;
            for pair in im_graph.all_edges(v_merge, v, 0) {
                count += (-pair.edge1.weight ^ -pair.edge2.weight < 0) as i32;
            }
            -count + im_graph[v_merge][v].marked.get() as i32
        });
        upper = merge_one(graph, upper - cost, best, 0, v_merge) + cost
    }
    graph.un_merge(v1, v2, v_merge);

    upper
}

pub fn merge_one(graph: &mut Graph, mut upper: i32, best: &mut Graph, i1: usize, v1: usize) -> i32 {
    let first = graph.positive(v1, i1).next();
    if let Some((i2, v2)) = first {
        let edge = graph.cut(v1, v2);
        let lower = pack(graph);
        if lower + edge.weight < upper {
            upper = merge_one(graph, upper - edge.weight, best, i2, v1) + edge.weight;
        }
        graph.un_cut(v1, v2, edge);

        let (v_merge_2, cost2) = graph.merge(v1, v2);
        if cost2 < upper {
            upper = search_graph(graph, upper - cost2, best) + cost2;
        }
        graph.un_merge(v1, v2, v_merge_2);

        upper
    } else {
        search_graph(graph, upper, best)
    }
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
            // println!("{}", upper);
            *best = graph.clone();
            lower
        }
    }
}
