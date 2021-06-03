use std::{
    cmp::min,
    mem::{replace, swap, take},
};

use crate::{
    branch::EdgeMod,
    graph::{Edge, Graph},
};

impl Graph {
    pub fn search_components(&mut self, best: &mut Graph) -> i32 {
        let original = self.clone();

        let mut total = 0;
        let components = self.components();

        let mut out_clusters: Vec<usize> = Vec::new();

        for component in components {
            self.clusters = component;
            let edges = self.edge_count();
            let max_edges = (self.clusters.len() * (self.clusters.len() - 1)) / 2;
            let upper = min(edges, max_edges as i32 - edges) + 1;

            let lower = self.pack();
            total += self.search_graph(lower, upper, best);

            for v1 in out_clusters.iter().copied() {
                for v2 in best.clusters.iter().copied() {
                    best.vertices[v1][v2] = Edge::none();
                    best.vertices[v2][v1] = Edge::none();
                }
            }
            out_clusters.extend(take(&mut best.clusters));
            swap(self, best);
        }

        self.clusters = out_clusters;
        self.check_easy();
        *best = replace(self, original);
        total
    }

    pub fn search_merge(&mut self, mut upper: i32, best: &mut Graph, v1: usize, v2: usize) -> i32 {
        let (v_merge, cost) = self.merge(v1, v2);
        debug_assert!(cost > 0);

        let lower = self.pack();
        if lower + cost < upper {
            let im_graph = unsafe { &*(self as *const Graph) };
            self.clusters.sort_unstable_by_key(|&v| {
                let mut count = 0;
                for pair in im_graph.all_edges(v_merge, v, 0) {
                    count += (-pair.edge1.weight ^ -pair.edge2.weight < 0) as i32;
                }
                -count + im_graph[v_merge][v].marked as i32
            });
            upper = self.merge_one(lower, upper - cost, best, 0, v_merge) + cost
        }
        self.un_merge(v1, v2, v_merge);

        upper
    }

    pub fn merge_one(
        &mut self,
        lower: i32,
        mut upper: i32,
        best: &mut Graph,
        i1: usize,
        v1: usize,
    ) -> i32 {
        let first = self.positive(v1, i1).next();
        if let Some((i2, v2)) = first {
            let edge = self.cut(v1, v2);
            let lower = self.pack();
            if lower + edge.weight < upper {
                upper = self.merge_one(lower, upper - edge.weight, best, i2, v1) + edge.weight;
            }
            self.un_cut(v1, v2, edge);

            let (v_merge_2, cost2) = self.merge(v1, v2);
            let lower = self.pack();
            if lower + cost2 < upper {
                upper = self.search_graph(lower, upper - cost2, best) + cost2;
            }
            self.un_merge(v1, v2, v_merge_2);

            upper
        } else {
            self.search_graph(lower, upper, best)
        }
    }

    pub fn search_cut(&mut self, mut upper: i32, best: &mut Graph, v1: usize, v2: usize) -> i32 {
        let edge = self.cut(v1, v2);
        let lower = self.pack();
        if lower + edge.weight < upper {
            upper = self.search_graph(lower, upper - edge.weight, best) + edge.weight;
        }
        self.un_cut(v1, v2, edge);

        upper
    }

    pub fn search_graph(&mut self, lower: i32, mut upper: i32, best: &mut Graph) -> i32 {
        match self.best_edge() {
            EdgeMod::Merge(v1, v2) => {
                upper = self.search_merge(upper, best, v1, v2);
                self.search_cut(upper, best, v1, v2)
            }
            EdgeMod::Delete(v1, v2) => {
                upper = self.search_cut(upper, best, v1, v2);
                self.search_merge(upper, best, v1, v2)
            }
            EdgeMod::Nothing => {
                // println!("{}", upper);
                self.check_easy();
                best.clone_from(self);
                lower
            }
        }
    }
}
