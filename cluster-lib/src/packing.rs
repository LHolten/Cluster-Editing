use std::cmp::min;

use crate::graph::Graph;

pub fn pack(graph: &Graph) -> i32 {
    let mut cost = 0;
    for (i1, v1) in graph.clusters(0) {
        for (i2, v2) in graph.positive(v1, i1) {
            let (marked1, marked2) = (graph[v1].marked.get(), graph[v2].marked.get());
            if marked1 && marked2 {
                continue;
            }

            for pair in graph.conflict_edges(v1, v2, i2) {
                let marked3 = graph[pair.to].marked.get();

                if marked3 && (marked1 && !pair.edge1.deleted || marked2 && !pair.edge2.deleted) {
                    continue;
                }

                graph[v1].marked.set(true);
                graph[v2].marked.set(true);
                graph[pair.to].marked.set(true);
                cost += min(
                    graph[v1][v2].weight,
                    min(pair.edge1.weight.abs(), pair.edge2.weight.abs()),
                );
                break;
            }
        }
    }

    for (_, v) in graph.clusters(0) {
        graph[v].marked.set(false)
    }

    cost
}
