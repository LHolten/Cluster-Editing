use std::cell::Cell;

use crate::graph::{EdgeIter, Graph, SetIter};

// fn critical(graph: Graph) -> Graph {
//     let mut new_vertices = Vec::new();
//     let mut new_indices = Vec::new();
//     for (i, this) in graph.iter().enumerate() {
//         for edge in this.edges {
//             let j = edge.index as usize;
//             if j > i {
//                 new_indices.push(new_vertices.len());
//                 new_vertices.push(this.clone());
//                 break;
//             }
//             let that = &graph[j];
//             if this.edges | i == that.edges | j {
//                 let index = new_indices[j];
//                 new_indices.push(index);
//                 new_vertices[index] = &new_vertices[index] + this;
//                 break;
//             }
//         }
//     }
//     new_vertices
// }

fn propagate(graph: &mut Graph) {
    let graph_cell = Cell::from_mut(graph);
    for v1 in SetIter::new(graph_cell) {
        for v2 in EdgeIter::new(graph_cell, &v1) {
            for v3 in EdgeIter::new(graph_cell, &(&v1 + v2)) {}
        }
    }
}
