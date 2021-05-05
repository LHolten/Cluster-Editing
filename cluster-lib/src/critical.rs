use crate::graph::{Graph, Vertex};

fn critical(graph: Graph) -> Graph {
    let mut new_vertices = Vec::new();
    let mut new_indices = Vec::new();
    for (i, this) in graph.iter().enumerate() {
        for edge in this.edges {
            let j = edge.index as usize;
            if j > i {
                new_indices.push(new_vertices.len());
                new_vertices.push(this.clone());
                break;
            }
            let that = &graph[j];
            if this.edges | i == that.edges | j {
                let index = new_indices[j];
                new_indices.push(index);
                new_vertices[index] = &new_vertices[index] + this;
                break;
            }
        }
    }
    new_vertices
}

fn propagate(graph: &mut Graph) {}

// impl Add<&Vertex> for &Vertex {
//     type Output = Vertex;

//     fn add(self, rhs: &Vertex) -> Self::Output {
//         Vertex {
//             size: self.size + rhs.size,
//             edges: AddEdges(MergeEdges::new(&self.edges, &rhs.edges)).collect(), // need to filter out the inner edges at some point
//         }
//     }
// }
