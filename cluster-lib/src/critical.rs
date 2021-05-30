// use crate::graph::Graph;

// pub fn critical(graph: &mut Graph) {
//     for mut vertex in graph.clusters().collect::<Vec<_>>() {
//         if graph[vertex].merged.is_some() {
//             continue;
//         }
//         // let mut same = Vec::new();
//         for edge in graph.edges(vertex).positive().copied().collect::<Vec<_>>() {
//             if graph.conflict_edges(vertex, edge.to).next().is_none() {
//                 vertex = graph.merge(vertex, edge.to);
//                 // same.push(edge.to);
//             }
//         }
//         // for vv in same.chunks_exact(2) {
//         //     cost += graph.merge_cost(vertex, vv[0]);
//         //     vertex = graph.merge(vertex, vv[0]);
//         //     cost += graph.merge_cost(vertex, vv[1]);
//         //     vertex = graph.merge(vertex, vv[1]);
//         // }
//     }
// }
