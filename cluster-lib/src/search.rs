use std::{
    cmp::min,
    mem::{swap, take},
};

use crate::{
    branch::EdgeMod,
    graph::{Edge, Graph},
    packing::pack,
};

impl Graph {
    pub fn search_components(&mut self, best: &mut Graph) -> i32 {
        let mut total = 0;
        let components = self.components();

        let mut input = Graph::new(0);
        let mut input_ref = self;
        let mut output = Graph::new(0);

        let mut out_clusters = Vec::new();

        for mut component in components {
            swap(&mut input_ref.clusters, &mut component);

            let edges = input_ref.edge_count();
            let max_edges = (input_ref.clusters.len() * (input_ref.clusters.len() - 1)) / 2;
            let upper = min(edges, max_edges as i32 - edges) + 1;

            let lower = pack(input_ref);
            total += input_ref.search_graph(lower, upper, &mut output);

            swap(&mut input_ref.clusters, &mut component);

            for v1 in out_clusters.iter().copied() {
                for v2 in output.clusters.clone() {
                    output[v1][v2] = Edge::none();
                    output[v2][v1] = Edge::none();
                }
            }
            out_clusters.extend(take(&mut output.clusters));

            input = output;
            input_ref = &mut input;
            output = Graph::new(0);
        }

        input.clusters = out_clusters;
        input.check_easy();
        *best = input;
        total
    }

    pub fn search_merge(&mut self, mut upper: i32, best: &mut Graph, v1: usize, v2: usize) -> i32 {
        let (v_merge, cost) = self.merge(v1, v2);
        debug_assert!(cost > 0);

        let lower = pack(self);
        if lower + cost < upper {
            let im_graph = unsafe { &*(self as *const Graph) };
            self.clusters.sort_unstable_by_key(|&v| {
                let mut count = 0;
                for pair in im_graph.all_edges(v_merge, v, 0) {
                    count += (-pair.edge1.weight ^ -pair.edge2.weight < 0) as i32;
                }
                -count + im_graph[v_merge][v].marked.get() as i32
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
            let lower = pack(self);
            if lower + edge.weight < upper {
                upper = self.merge_one(lower, upper - edge.weight, best, i2, v1) + edge.weight;
            }
            self.un_cut(v1, v2, edge);

            let (v_merge_2, cost2) = self.merge(v1, v2);
            let lower = pack(self);
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
        let lower = pack(self);
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
                *best = self.clone();
                lower
            }
        }
    }
}
