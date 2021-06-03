use crate::{graph::Graph, search::Solver};

impl Solver {
    pub fn components(&mut self) -> Vec<Vec<usize>> {
        let mut components = Vec::new();
        for (_, v) in self.graph.all(0) {
            if !self.vertex_markers[v] {
                let mut component = Vec::new();
                add_connected(&self.graph, v, &mut component, &mut self.vertex_markers);
                components.push(component)
            }
        }
        for (_, v) in self.graph.all(0) {
            self.vertex_markers[v] = false;
        }
        components
    }
}

fn add_connected(graph: &Graph, v1: usize, component: &mut Vec<usize>, marked: &mut Vec<bool>) {
    component.push(v1);
    marked[v1] = true;
    for (_, v2) in graph.positive(v1, 0) {
        if !marked[v2] {
            add_connected(graph, v2, component, marked);
        }
    }
}
