use std::cmp::min;

use crate::{
    graph::{AllFrom, Graph, GraphData},
    triple::Triple,
};

impl Graph {
    pub fn pack(&mut self) {
        for (i1, v1) in self.active.all(0) {
            for (i2, v2) in self.active.all(i1) {
                for (_, v3) in self.active.all(i2) {
                    self.data.add_triple(v1, v2, v3);
                }
            }
        }
    }
}

impl GraphData {
    pub fn add_vertex(&mut self, v1: usize, active: &[usize]) {
        for (i2, v2) in active.all(0) {
            if v1 != v2 {
                for (_, v3) in active.all(i2) {
                    if v3 != v1 {
                        self.add_triple(v1, v2, v3);
                    }
                }
            }
        }
    }

    pub fn remove_vertex(&mut self, v1: usize, active: &[usize]) {
        for i in (0..self.triples.len()).rev() {
            if self.triples[i].vertex(v1) {
                let triple = self.triples.swap_remove(i);
                self.remove_triple_cost(triple);
            }
        }

        for (i2, v2) in active.all(0) {
            if v2 != v1 {
                for (_, v3) in active.all(i2) {
                    if v3 != v1 {
                        self.remove_triple_conflicts(v1, v2, v3);
                    }
                }
            }
        }
    }

    pub fn add_vertex_pair(&mut self, v1: usize, v2: usize, active: &[usize]) {
        for (i3, v3) in active.all(0) {
            if v1 != v3 && v2 != v3 {
                for (_, v4) in active.all(i3) {
                    if v4 != v1 && v4 != v2 {
                        self.add_triple(v1, v3, v4);
                        self.add_triple(v2, v3, v4);
                    }
                }
            }
        }

        self.add_edge(v1, v2, active);
    }

    pub fn remove_vertex_pair(&mut self, v1: usize, v2: usize, active: &[usize]) {
        for i in (0..self.triples.len()).rev() {
            if self.triples[i].vertex(v1) || self.triples[i].vertex(v2) {
                let triple = self.triples.swap_remove(i);
                self.remove_triple_cost(triple);
            }
        }

        for (i3, v3) in active.all(0) {
            if v1 != v3 && v2 != v3 {
                for (_, v4) in active.all(i3) {
                    if v4 != v1 && v4 != v2 {
                        self.remove_triple_conflicts(v1, v3, v4);
                        self.remove_triple_conflicts(v2, v3, v4);
                    }
                }
            }
        }

        self.remove_edge_conflicts(v1, v2, active)
    }

    pub fn add_edge(&mut self, v1: usize, v2: usize, active: &[usize]) {
        for (_, v3) in active.all(0) {
            if v1 != v3 && v2 != v3 {
                self.add_triple(v1, v2, v3);
            }
        }
    }

    pub fn remove_edge(&mut self, v1: usize, v2: usize, active: &[usize]) {
        for i in (0..self.triples.len()).rev() {
            if self.triples[i].edge([v1, v2]) {
                let triple = self.triples.swap_remove(i);
                self.remove_triple_cost(triple);
            }
        }

        self.remove_edge_conflicts(v1, v2, active)
    }

    pub fn remove_edge_conflicts(&mut self, v1: usize, v2: usize, active: &[usize]) {
        for (_, v3) in active.all(0) {
            if v1 != v3 && v2 != v3 {
                self.remove_triple_conflicts(v1, v2, v3);
            }
        }
    }

    pub fn add_triple(&mut self, v1: usize, v2: usize, v3: usize) {
        let e12 = self[[v1, v3]].weight > 0;
        let e13 = self[[v2, v3]].weight > 0;
        let e23 = self[[v1, v2]].weight > 0;
        if e12 as i32 + e13 as i32 + e23 as i32 != 2 {
            return;
        }
        self[[v1, v3]].conflicts += 1;
        self[[v2, v3]].conflicts += 1;
        self[[v1, v2]].conflicts += 1;

        if self[[v1, v2]].marked == 0 || self[[v1, v3]].marked == 0 || self[[v2, v3]].marked == 0 {
            return;
        }

        let cost = min(
            self[[v1, v2]].marked,
            min(self[[v1, v3]].marked, self[[v2, v3]].marked),
        );
        self[[v1, v3]].marked -= cost;
        self[[v2, v3]].marked -= cost;
        self[[v1, v2]].marked -= cost;
        self.triples.push(Triple::new([v1, v2, v3], cost));
        self.lower += cost;
    }

    pub fn remove_triple_conflicts(&mut self, v1: usize, v2: usize, v3: usize) {
        let e12 = self[[v1, v3]].weight > 0;
        let e13 = self[[v2, v3]].weight > 0;
        let e23 = self[[v1, v2]].weight > 0;
        if e12 as i32 + e13 as i32 + e23 as i32 != 2 {
            return;
        }
        self[[v1, v3]].conflicts -= 1;
        self[[v2, v3]].conflicts -= 1;
        self[[v1, v2]].conflicts -= 1;
    }

    pub fn remove_triple_cost(&mut self, triple: Triple) {
        let [v1, v2, v3] = triple.vertices;
        self[[v1, v3]].marked += triple.cost;
        self[[v2, v3]].marked += triple.cost;
        self[[v1, v2]].marked += triple.cost;
        self.lower -= triple.cost;
    }
}
