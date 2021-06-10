use std::{cmp::min, mem::swap};

use crate::{
    graph::{Active, AllFrom, Graph, GraphData},
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
    pub fn add_vertex(&mut self, v1: usize, active: Active) {
        for (_, v2) in active {
            if v1 != v2 {
                self.add_edge(v1, v2, active);
            }
        }
    }

    pub fn remove_vertex(&mut self, v1: usize, active: Active) {
        for i in (0..self.triples.len()).rev() {
            if self.triples[i].vertex(v1) {
                let triple = self.triples.swap_remove(i);
                let [v1, v2, v3] = triple.vertices;
                self[v1][v3].marked += triple.cost;
                self[v3][v1].marked += triple.cost;
                self[v2][v3].marked += triple.cost;
                self[v3][v2].marked += triple.cost;
                self[v1][v2].marked += triple.cost;
                self[v2][v1].marked += triple.cost;
                self.lower -= triple.cost;
            }
        }

        for (_, v2) in active {
            if v1 != v2 {
                self.remove_edge(v1, v2, active);
            }
        }
    }

    pub fn add_edge(&mut self, v1: usize, v2: usize, active: Active) {
        for (_, v3) in active {
            if v3 != v1 && v3 != v2 {
                self.add_triple(v1, v2, v3);
            }
        }
    }

    pub fn remove_edge(&mut self, mut v1: usize, mut v2: usize, active: Active) {
        if v1 > v2 {
            swap(&mut v1, &mut v2);
        }
        for i in (0..self.triples.len()).rev() {
            if self.triples[i].edge(v1, v2) {
                let triple = self.triples.swap_remove(i);
                let [v1, v2, v3] = triple.vertices;
                self[v1][v3].marked += triple.cost;
                self[v3][v1].marked += triple.cost;
                self[v2][v3].marked += triple.cost;
                self[v3][v2].marked += triple.cost;
                self[v1][v2].marked += triple.cost;
                self[v2][v1].marked += triple.cost;
                self.lower -= triple.cost;
            }
        }

        for (_, v3) in active {
            if v3 != v1 && v3 != v2 {
                self.remove_triple(v1, v2, v3);
            }
        }
    }

    pub fn add_triple(&mut self, v1: usize, v2: usize, v3: usize) {
        let e12 = self[v1][v2].weight > 0;
        let e13 = self[v1][v3].weight > 0;
        let e23 = self[v2][v3].weight > 0;
        if e12 as i32 + e13 as i32 + e23 as i32 != 2 {
            return;
        }
        self[v1][v3].conflicts += 1;
        self[v3][v1].conflicts += 1;
        self[v2][v3].conflicts += 1;
        self[v3][v2].conflicts += 1;
        self[v1][v2].conflicts += 1;
        self[v2][v1].conflicts += 1;

        if self[v1][v2].marked == 0 || self[v1][v3].marked == 0 || self[v2][v3].marked == 0 {
            return;
        }
        debug_assert_eq!(self[v1][v3].marked, self[v3][v1].marked);
        debug_assert_eq!(self[v2][v3].marked, self[v3][v2].marked);
        debug_assert_eq!(self[v2][v1].marked, self[v1][v2].marked);
        let cost = min(
            self[v1][v2].marked,
            min(self[v1][v3].marked, self[v2][v3].marked),
        );
        self[v1][v3].marked -= cost;
        self[v3][v1].marked -= cost;
        self[v2][v3].marked -= cost;
        self[v3][v2].marked -= cost;
        self[v1][v2].marked -= cost;
        self[v2][v1].marked -= cost;
        self.triples.push(Triple::new([v1, v2, v3], cost));
        self.lower += cost;
    }

    pub fn remove_triple(&mut self, v1: usize, v2: usize, v3: usize) {
        let e12 = self[v1][v2].weight > 0;
        let e13 = self[v1][v3].weight > 0;
        let e23 = self[v2][v3].weight > 0;
        if e12 as i32 + e13 as i32 + e23 as i32 != 2 {
            return;
        }
        self[v1][v3].conflicts -= 1;
        self[v3][v1].conflicts -= 1;
        self[v2][v3].conflicts -= 1;
        self[v3][v2].conflicts -= 1;
        self[v1][v2].conflicts -= 1;
        self[v2][v1].conflicts -= 1;
    }
}
