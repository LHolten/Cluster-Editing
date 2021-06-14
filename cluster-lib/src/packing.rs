use std::cmp::min;

use crate::{
    graph::{AllFrom, Graph},
    matrix::Matrix,
    triple::Triple,
};

#[derive(Clone)]
pub struct Packing {
    pub triples: Vec<Triple>,
    pub edge_conflicts: Matrix<u32>,
    pub edge_cost: Matrix<u32>,
    pub lower: u32,
}

impl Packing {
    pub fn new(len: usize) -> Self {
        Self {
            triples: vec![],
            edge_conflicts: Matrix::new(0, len),
            edge_cost: Matrix::new(0, len),
            lower: 0,
        }
    }

    pub fn pack(&mut self, graph: &Graph) {
        self.triples.clear();
        self.lower = 0;
        for (i1, v1) in graph.active.all(0) {
            for (_, v2) in graph.active.all(i1) {
                self.edge_conflicts[[v1, v2]] = 0;
                self.edge_cost[[v1, v2]] = 0;
            }
        }

        for (i1, v1) in graph.active.all(0) {
            for (i2, v2) in graph.active.all(i1) {
                for (_, v3) in graph.active.all(i2) {
                    self.add_triple(graph, v1, v2, v3);
                }
            }
        }
    }

    pub fn add_vertex(&mut self, graph: &Graph, v1: usize) {
        if cfg!(not(feature = "incremental")) {
            return;
        }
        for (i2, v2) in graph.active.all(0) {
            if v1 != v2 {
                for (_, v3) in graph.active.all(i2) {
                    if v3 != v1 {
                        self.add_triple(graph, v1, v2, v3);
                    }
                }
            }
        }
    }

    pub fn remove_vertex(&mut self, graph: &Graph, v1: usize) {
        if cfg!(not(feature = "incremental")) {
            return;
        }
        for i in (0..self.triples.len()).rev() {
            if self.triples[i].vertex(v1) {
                let triple = self.triples.swap_remove(i);
                self.remove_triple_cost(triple);
            }
        }

        for (i2, v2) in graph.active.all(0) {
            if v2 != v1 {
                for (_, v3) in graph.active.all(i2) {
                    if v3 != v1 {
                        self.remove_triple_conflicts(graph, v1, v2, v3);
                    }
                }
            }
        }
    }

    pub fn add_vertex_pair(&mut self, graph: &Graph, v1: usize, v2: usize) {
        if cfg!(not(feature = "incremental")) {
            return;
        }
        for (i3, v3) in graph.active.all(0) {
            if v1 != v3 && v2 != v3 {
                for (_, v4) in graph.active.all(i3) {
                    if v4 != v1 && v4 != v2 {
                        self.add_triple(graph, v1, v3, v4);
                        self.add_triple(graph, v2, v3, v4);
                    }
                }
            }
        }

        self.add_edge(graph, v1, v2);
    }

    pub fn remove_vertex_pair(&mut self, graph: &Graph, v1: usize, v2: usize) {
        if cfg!(not(feature = "incremental")) {
            return;
        }
        for i in (0..self.triples.len()).rev() {
            if self.triples[i].vertex(v1) || self.triples[i].vertex(v2) {
                let triple = self.triples.swap_remove(i);
                self.remove_triple_cost(triple);
            }
        }

        for (i3, v3) in graph.active.all(0) {
            if v1 != v3 && v2 != v3 {
                for (_, v4) in graph.active.all(i3) {
                    if v4 != v1 && v4 != v2 {
                        self.remove_triple_conflicts(graph, v1, v3, v4);
                        self.remove_triple_conflicts(graph, v2, v3, v4);
                    }
                }
            }
        }

        self.remove_edge_conflicts(graph, v1, v2)
    }

    pub fn add_edge(&mut self, graph: &Graph, v1: usize, v2: usize) {
        if cfg!(not(feature = "incremental")) {
            return;
        }
        for (_, v3) in graph.active.all(0) {
            if v1 != v3 && v2 != v3 {
                self.add_triple(graph, v1, v2, v3);
            }
        }
    }

    pub fn remove_edge(&mut self, graph: &Graph, v1: usize, v2: usize) {
        if cfg!(not(feature = "incremental")) {
            return;
        }
        for i in (0..self.triples.len()).rev() {
            if self.triples[i].edge([v1, v2]) {
                let triple = self.triples.swap_remove(i);
                self.remove_triple_cost(triple);
            }
        }

        self.remove_edge_conflicts(graph, v1, v2)
    }

    pub fn remove_edge_conflicts(&mut self, graph: &Graph, v1: usize, v2: usize) {
        for (_, v3) in graph.active.all(0) {
            if v1 != v3 && v2 != v3 {
                self.remove_triple_conflicts(graph, v1, v2, v3);
            }
        }
    }

    #[inline(always)]
    pub fn add_triple(&mut self, graph: &Graph, v1: usize, v2: usize, v3: usize) {
        let e12 = graph[[v1, v3]].weight > 0;
        let e13 = graph[[v2, v3]].weight > 0;
        let e23 = graph[[v1, v2]].weight > 0;
        if e12 as u32 + e13 as u32 + e23 as u32 != 2 {
            return;
        }
        self.edge_conflicts[[v1, v3]] += 1;
        self.edge_conflicts[[v2, v3]] += 1;
        self.edge_conflicts[[v1, v2]] += 1;

        if self.edge_cost[[v1, v2]] == graph[[v1, v2]].weight.abs() as u32
            || self.edge_cost[[v1, v3]] == graph[[v1, v3]].weight.abs() as u32
            || self.edge_cost[[v2, v3]] == graph[[v2, v3]].weight.abs() as u32
        {
            return;
        }

        let cost = min(
            graph[[v1, v2]].weight.abs() as u32 - self.edge_cost[[v1, v2]],
            min(
                graph[[v1, v3]].weight.abs() as u32 - self.edge_cost[[v1, v3]],
                graph[[v2, v3]].weight.abs() as u32 - self.edge_cost[[v2, v3]],
            ),
        );
        self.edge_cost[[v1, v3]] += cost;
        self.edge_cost[[v2, v3]] += cost;
        self.edge_cost[[v1, v2]] += cost;
        self.triples.push(Triple::new([v1, v2, v3], cost));
        self.lower += cost;
    }

    pub fn remove_triple_conflicts(&mut self, graph: &Graph, v1: usize, v2: usize, v3: usize) {
        let e12 = graph[[v1, v3]].weight > 0;
        let e13 = graph[[v2, v3]].weight > 0;
        let e23 = graph[[v1, v2]].weight > 0;
        if e12 as u32 + e13 as u32 + e23 as u32 != 2 {
            return;
        }
        self.edge_conflicts[[v1, v3]] -= 1;
        self.edge_conflicts[[v2, v3]] -= 1;
        self.edge_conflicts[[v1, v2]] -= 1;
    }

    pub fn remove_triple_cost(&mut self, triple: Triple) {
        let [v1, v2, v3] = triple.vertices;
        self.edge_cost[[v1, v3]] -= triple.cost;
        self.edge_cost[[v2, v3]] -= triple.cost;
        self.edge_cost[[v1, v2]] -= triple.cost;
        self.lower -= triple.cost;
    }
}
