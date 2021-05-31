use std::cmp::min;

use crate::graph::Graph;

pub fn pack(graph: &Graph) -> i32 {
    let mut cost = 0;
    for (i1, v1) in graph.clusters(0) {
        for (i2, v2) in graph.positive(v1, i1) {
            if graph[v1][v2].marked.get() {
                continue;
            }

            for pair in graph.conflict_edges(v1, v2, i2) {
                if !pair.edge1.deleted && pair.edge1.marked.get()
                    || !pair.edge2.deleted && pair.edge2.marked.get()
                {
                    continue;
                }

                pair.edge1.marked.set(true);
                pair.edge2.marked.set(true);
                graph[v1][v2].marked.set(true);
                cost += min(
                    graph[v1][v2].weight,
                    min(pair.edge1.weight.abs(), pair.edge2.weight.abs()),
                );
                break;
            }
        }
    }

    for (i1, v1) in graph.clusters(0) {
        for (_, v2) in graph.clusters(i1) {
            graph[v1][v2].marked.set(false)
        }
    }

    cost
}
