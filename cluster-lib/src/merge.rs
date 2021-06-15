use std::cmp::{max, min};

use crate::graph::{AllFrom, Edge, Graph};

impl Graph {
    // requires edge between vertices to be positive
    pub fn merge(&mut self, v1: usize, v2: usize) -> (usize, u32) {
        self.active.retain(|&v| v != v1 && v != v2);

        let vv = self.len;
        self.len += 1;

        let mut cost = max(0, -self[[v1, v2]].weight) as u32;
        for &v3 in &self.active {
            if -self[[v1, v3]].weight ^ -self[[v2, v3]].weight < 0 {
                cost += min(self[[v1, v3]].weight.abs(), self[[v2, v3]].weight.abs()) as u32;
            }
            self.edges[[vv, v3]] = if self[[v1, v3]].fixed || self[[v2, v3]].fixed {
                Edge::none()
            } else {
                Edge::new(self[[v1, v3]].weight + self[[v2, v3]].weight)
            };
        }

        self.active.push(vv);
        self.vertex_merged[v1] = Some(vv);
        self.vertex_merged[v2] = Some(vv);
        (vv, cost)
    }

    pub fn un_merge(&mut self, v1: usize, v2: usize, vv: usize) {
        self.active.retain(|&v| v != vv);
        self.active.push(v1);
        self.active.push(v2);
        self.vertex_merged[v1] = None;
        self.vertex_merged[v2] = None;
        self.len -= 1;
    }

    pub fn conflict_edges(
        &self,
        v1: usize,
        v2: usize,
        from: usize,
    ) -> impl '_ + Iterator<Item = (usize, usize)> {
        self.active
            .all(from)
            .filter(move |&(_, v3)| -self[[v1, v3]].weight ^ -self[[v2, v3]].weight < 0)
    }

    pub fn two_edges(
        &self,
        v1: usize,
        v2: usize,
        from: usize,
    ) -> impl '_ + Iterator<Item = (usize, usize)> {
        self.active
            .all(from)
            .filter(move |&(_, v3)| self[[v1, v3]].weight > 0 && self[[v2, v3]].weight > 0)
    }
}
