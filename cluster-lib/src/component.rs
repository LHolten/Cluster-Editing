use crate::graph::{AllFrom, Graph};

#[derive(Clone)]
pub struct Components {
    un_active: Vec<usize>,
    vertex_markers: Vec<bool>,
}

impl Components {
    pub fn new(len: usize) -> Self {
        Self {
            un_active: vec![],
            vertex_markers: vec![false; len * len],
        }
    }

    pub fn isolate_component(&mut self, graph: &mut Graph) -> usize {
        for (_, v) in graph.active.all(0) {
            self.vertex_markers[v] = false;
        }
        add_connected(graph, graph.active[0], &mut self.vertex_markers);
        let mut count = 0;
        for i in (0..graph.active.len()).rev() {
            if !self.vertex_markers[graph.active[i]] {
                self.un_active.push(graph.active.swap_remove(i));
                count += 1;
            }
        }
        count
    }

    pub fn other_component(&mut self, active: &mut Vec<usize>, len: usize) -> usize {
        let active_len = active.len();
        let un_active_len = self.un_active.len();
        if active_len > len {
            self.un_active[un_active_len - len..].swap_with_slice(&mut active[..active_len - len]);
            self.un_active.extend(active.drain(active_len - len..));
        } else {
            let split = un_active_len - len + active_len;
            self.un_active[un_active_len - len..split].swap_with_slice(active);
            active.extend(self.un_active.drain(split..))
        }
        active_len
    }

    pub fn all_components(&mut self, active: &mut Vec<usize>, len: usize) {
        active.extend(self.un_active.drain(self.un_active.len() - len..))
    }
}

fn add_connected(graph: &Graph, v1: usize, vertex_markers: &mut Vec<bool>) {
    vertex_markers[v1] = true;
    for (_, v2) in graph.positive(v1, 0) {
        if !vertex_markers[v2] {
            add_connected(graph, v2, vertex_markers);
        }
    }
}
