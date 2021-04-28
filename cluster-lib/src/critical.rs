use crate::graph::Graph;

impl Graph {
    fn critical(&self) -> Self {
        let mut new_vertices = Vec::new();
        let mut new_indices = Vec::new();
        for (i, this) in self.0.iter().enumerate() {
            for edge in this.edges {
                let j = edge.index as usize;
                if j > i {
                    new_indices.push(new_vertices.len());
                    new_vertices.push(this.clone());
                    break;
                }
                let that = self.0[j];
                if this.edges | i == that.edges | j {
                    let index = new_indices[j];
                    new_indices.push(index);
                    new_vertices[index] += this;
                    break;
                }
            }
        }
        Graph(new_vertices)
    }
}
