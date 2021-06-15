use std::cmp::max;

use crate::{
    branch::EdgeMod, component::Components, disk::finish_solve, graph::Graph, packing::Packing,
};

#[derive(Clone)]
pub struct Solver {
    pub graph: Graph,
    pub packing: Packing,
    pub components: Components,
    pub upper: u32,
    pub best: Graph,
}

impl Solver {
    pub fn new(graph: Graph) -> Self {
        let len = graph.vertices.len();
        let mut packing = Packing::new(len);
        packing.pack(&graph);
        Self {
            graph: graph.clone(),
            packing,
            components: Components::new(len),
            upper: u32::MAX,
            best: graph,
        }
    }

    pub fn search_components(&mut self) {
        let count = self.components.isolate_component(&mut self.graph);
        if count == 0 {
            return self.search_graph();
        }

        let upper_before = self.upper;
        self.search_graph();
        let diff = self.upper - self.packing.lower;
        self.upper = upper_before;

        let count = self
            .components
            .other_component(&mut self.graph.active, count);

        let old_len = self.graph.len;
        self.graph.len = self.best.len;
        self.packing.lower += diff;
        self.search_components();
        self.packing.lower -= diff;
        self.graph.len = old_len;

        self.components
            .all_components(&mut self.graph.active, count);
    }

    pub fn search_merge(&mut self, v1: usize, v2: usize) {
        self.packing.remove_vertex_pair(&self.graph, v1, v2);
        let (vv, cost) = self.graph.merge(v1, v2);
        self.packing.add_vertex(&self.graph, vv);

        if cfg!(not(feature = "incremental")) {
            self.packing.pack(&self.graph)
        }
        if self.packing.lower + (cost as u32) < self.upper {
            self.upper -= cost;
            self.search_graph();
            self.upper += cost;
        }
        self.packing.remove_vertex(&self.graph, vv);
        self.graph.un_merge(v1, v2, vv);
        self.packing.add_vertex_pair(&self.graph, v1, v2);
    }

    pub fn search_cut(&mut self, v1: usize, v2: usize) {
        self.packing.remove_edge(&self.graph, v1, v2);
        let edge = self.graph.cut(v1, v2);
        self.packing.add_edge(&self.graph, v1, v2);
        let cost = max(0, edge.weight) as u32;

        if cfg!(not(feature = "incremental")) {
            self.packing.pack(&self.graph)
        }
        if self.packing.lower + cost < self.upper {
            self.upper -= cost;
            self.search_graph();
            self.upper += cost;
        }
        self.packing.remove_edge(&self.graph, v1, v2);
        self.graph.un_cut(v1, v2, edge);
        self.packing.add_edge(&self.graph, v1, v2);
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
                finish_solve(&mut self.best);
                self.upper = self.packing.lower
            }
        }
    }
}
