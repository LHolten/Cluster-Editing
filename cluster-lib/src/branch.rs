use crate::{
    graph::{EdgeIter, Graph},
    merge::MergeEdges,
};

// find best edge to split on in O(n + m^2) time
// fn best_edge(graph: &mut Graph) -> Option<(u32, u32)> {
//     let mut best = None;
//     let mut best_count = 2;

//     for v1 in graph.clusters() {
//         for v2 in graph.edges(v1) {
//             let count = MergeEdges::new(&v1.edges, &v2.edges).count_diff();
//             if count > best_count {
//                 best_count = count;
//                 best = Some((v1.index, v2.index))
//             }
//         }
//     }
//     best
// }
