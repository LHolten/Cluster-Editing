use std::{
    cmp::{max, min},
    mem::{replace, swap, take},
};

use crate::{
    branch::EdgeMod,
    graph::{Edge, Graph},
};

#[derive(Clone)]
pub struct Solver {
    pub graph: Graph,
    pub upper: i32,
    pub best: Graph,
    pub vertex_markers: Vec<bool>,
}

impl Solver {
    pub fn new(graph: Graph) -> Self {
        let len = graph.vertices.len();
        Self {
            graph: graph.clone(),
            upper: i32::MAX,
            best: graph,
            vertex_markers: vec![false; len],
        }
    }

    pub fn search_components(&mut self) {
        let original = self.graph.clone();

        let mut total = 0;
        let components = self.components();

        let mut out_clusters: Vec<usize> = Vec::new();

        for component in components {
            self.graph.active = component;

            let edges = self.graph.edge_count();
            let max_edges = (self.graph.active.len() * (self.graph.active.len() - 1)) / 2;
            self.upper = min(edges, max_edges as i32 - edges) + 1;

            self.graph.pack();
            self.search_graph();
            total += self.upper;

            for v1 in out_clusters.iter().copied() {
                for v2 in self.best.active.iter().copied() {
                    self.best.data[[v1, v2]] = Edge::none();
                }
            }
            out_clusters.extend(take(&mut self.best.active));
            swap(&mut self.graph, &mut self.best);
        }

        self.best = replace(&mut self.graph, original);
        self.best.active = out_clusters;
        // self.best.check_easy();
        self.upper = total;
    }

    pub fn search_merge(&mut self, v1: usize, v2: usize) {
        let (vv, cost) = self.graph.merge(v1, v2);
        if self.graph.lower + cost < self.upper {
            self.upper -= cost;
            self.search_graph();
            self.upper += cost;
        }
        self.graph.un_merge(v1, v2, vv);
    }

    pub fn search_cut(&mut self, v1: usize, v2: usize) {
        let edge = self.graph.cut(v1, v2);
        if self.graph.lower + max(0, edge.weight) < self.upper {
            self.upper -= max(0, edge.weight);
            self.search_graph();
            self.upper += max(0, edge.weight);
        }
        self.graph.un_cut(v1, v2, edge);
    }

    pub fn search_graph(&mut self) {
        match self.best_edge() {
            EdgeMod::Merge(v1, v2) => {
                self.search_merge(v1, v2);
                self.search_cut(v1, v2)
            }
            EdgeMod::Cut(v1, v2) => {
                self.search_cut(v1, v2);
                self.search_merge(v1, v2)
            }
            EdgeMod::Nothing => {
                // println!("{}", upper);
                self.best.clone_from(&self.graph);
                self.best.check_easy();
                self.upper = self.best.lower
            }
        }
    }
}
