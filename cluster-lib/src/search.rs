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
    pub edge_markers: Vec<Vec<i32>>,
    pub vertex_markers: Vec<bool>,
    pub edge_one: Vec<Vec<i32>>,
    pub edge_two: Vec<Vec<i32>>,
    pub edge_three: Vec<Vec<i32>>,
    pub deletion: Vec<Vec<i32>>,
    pub max: i32,
}

impl Solver {
    pub fn new(graph: Graph) -> Self {
        let len = graph.vertices.len();
        Self {
            graph: graph.clone(),
            upper: i32::MAX,
            best: graph,
            edge_markers: vec![vec![0; len]; len],
            vertex_markers: vec![false; len],
            edge_one: vec![vec![0; len]; len],
            edge_two: vec![vec![0; len]; len],
            edge_three: vec![vec![0; len]; len],
            deletion: vec![vec![0; len]; len],
            max: 0,
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

            let lower = self.pack();
            self.search_graph(lower);
            total += self.upper;

            for v1 in out_clusters.iter().copied() {
                for v2 in self.best.active.iter().copied() {
                    self.best.vertices[v1][v2] = Edge::none();
                    self.best.vertices[v2][v1] = Edge::none();
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
        // debug_assert!(cost > 0);

        let lower = self.pack();
        if lower + cost < self.upper {
            self.upper -= cost;
            // self.merge_one(lower, vv);
            self.search_graph(lower);
            self.upper += cost;
        }
        self.graph.un_merge(v1, v2, vv);
    }

    // pub fn search_merge_2(&mut self, v1: usize, v2: usize) {
    //     let (vv, cost) = self.graph.merge(v1, v2);
    //     let lower = self.pack();
    //     if lower + cost < self.upper {
    //         self.upper -= cost;
    //         self.search_graph(lower);
    //         self.upper += cost;
    //     }
    //     self.graph.un_merge(v1, v2, vv);
    // }

    // pub fn merge_one(&mut self, lower: i32, v1: usize) {
    //     let mut best = EdgeMod::Nothing;
    //     let mut best_cost = 0;

    //     for (i1, v2) in self.graph.all(0) {
    //         if v1 == v2 {
    //             for (_, v2) in self.graph.all(i1) {
    //                 if self.edge_two[v1][v2] > best_cost {
    //                     debug_assert!(!self.graph[v1][v2].fixed);
    //                     best_cost = self.edge_two[v1][v2];
    //                     if self.graph[v1][v2].weight > 0 {
    //                         best = EdgeMod::Cut(v1, v2)
    //                     } else {
    //                         best = EdgeMod::Merge(v1, v2)
    //                     }
    //                 }
    //             }
    //             break;
    //         }
    //         let (v1, v2) = (v2, v1);
    //         if self.edge_two[v1][v2] > best_cost {
    //             debug_assert!(!self.graph[v1][v2].fixed);
    //             best_cost = self.edge_two[v1][v2];
    //             if self.graph[v1][v2].weight > 0 {
    //                 best = EdgeMod::Cut(v1, v2)
    //             } else {
    //                 best = EdgeMod::Merge(v1, v2)
    //             }
    //         }
    //     }

    //     match best {
    //         EdgeMod::Merge(v1, v2) => {
    //             self.search_merge_2(v1, v2);
    //             self.search_cut_2(v1, v2)
    //         }
    //         EdgeMod::Cut(v1, v2) => {
    //             self.search_cut_2(v1, v2);
    //             self.search_merge_2(v1, v2)
    //         }
    //         EdgeMod::Nothing => self.search_graph(lower),
    //     }
    // }

    // pub fn search_cut_2(&mut self, v1: usize, v2: usize) {
    //     let edge = self.graph.cut(v1, v2);
    //     let lower = self.pack();
    //     if lower + max(0, edge.weight) < self.upper {
    //         self.upper -= max(0, edge.weight);
    //         self.merge_one(lower, v1);
    //         self.upper += max(0, edge.weight);
    //     }
    //     self.graph.un_cut(v1, v2, edge);
    // }

    pub fn search_cut(&mut self, v1: usize, v2: usize) {
        let edge = self.graph.cut(v1, v2);
        let lower = self.pack();
        if lower + max(0, edge.weight) < self.upper {
            self.upper -= max(0, edge.weight);
            self.search_graph(lower);
            self.upper += max(0, edge.weight);
        }
        self.graph.un_cut(v1, v2, edge);
    }

    pub fn search_graph(&mut self, lower: i32) {
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
                // assert_eq!(lower, self.max);
                // self.best.check_easy();
                self.upper = lower
            }
        }
    }
}
