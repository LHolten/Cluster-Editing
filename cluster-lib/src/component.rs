use crate::graph::Graph;

impl Graph {
    pub fn components(&self) -> Vec<Vec<usize>> {
        let mut components = Vec::new();
        for (_, v) in self.clusters(0) {
            if !self[v].marked.get() {
                let mut component = Vec::new();
                self.add_connected(v, &mut component);
                components.push(component)
            }
        }
        for (_, v) in self.clusters(0) {
            self[v].marked.set(false)
        }
        components
    }

    fn add_connected(&self, v1: usize, component: &mut Vec<usize>) {
        component.push(v1);
        self[v1].marked.set(true);
        for (_, v2) in self.positive(v1, 0) {
            if !self[v2].marked.get() {
                self.add_connected(v2, component);
            }
        }
    }
}
