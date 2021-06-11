#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Triple {
    pub vertices: [usize; 3],
    pub cost: i32,
}

impl Triple {
    pub fn new(mut vertices: [usize; 3], cost: i32) -> Self {
        vertices.sort_unstable();
        debug_assert!(cost > 0);
        Self { vertices, cost }
    }

    pub fn edge(&self, mut vertices: [usize; 2]) -> bool {
        vertices.sort_unstable();
        let e11 = vertices[0] == self.vertices[0];
        let e12 = vertices[0] == self.vertices[1];
        let e22 = vertices[1] == self.vertices[1];
        let e23 = vertices[1] == self.vertices[2];
        (e11 | e12) & (e22 | e23)
    }

    pub fn vertex(&self, v1: usize) -> bool {
        let e11 = v1 == self.vertices[0];
        let e12 = v1 == self.vertices[1];
        let e13 = v1 == self.vertices[2];
        e11 | e12 | e13
    }
}
