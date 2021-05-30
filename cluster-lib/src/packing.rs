use std::cmp::min;

use crate::graph::Graph;

pub fn pack(graph: &Graph) -> i32 {
    let mut cost = 0;
    for vertex in graph.clusters.iter() {
        for vertex2 in graph.positive(vertex) {
            if vertex2 >= vertex {
                break;
            }
            let (marked1, marked2) = (graph[vertex].marked.get(), graph[vertex2].marked.get());
            if marked1 && marked2 {
                continue;
            }

            for pair in graph.conflict_edges(vertex, vertex2) {
                if pair.to >= vertex2 {
                    break;
                }
                let marked3 = graph[pair.to].marked.get();

                if marked3 && (marked1 && !pair.edge1.deleted || marked2 && !pair.edge2.deleted) {
                    continue;
                }

                graph[vertex].marked.set(true);
                graph[vertex2].marked.set(true);
                graph[pair.to].marked.set(true);
                cost += min(
                    graph[vertex][vertex2].weight,
                    min(pair.edge1.weight.abs(), pair.edge2.weight.abs()),
                );
                break;
            }
        }
    }

    for vertex in graph.clusters.iter() {
        graph[vertex].marked.set(false)
    }

    cost
}
