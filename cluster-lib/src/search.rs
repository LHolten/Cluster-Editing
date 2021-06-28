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
        let len = graph.vertex_merged.len();
        let mut packing = Packing::new(len);
        packing.pack(&graph);
        Self {
            graph: graph.clone(),
            packing,
            components: Components::new(len),
            upper: graph.edge_count(),
            best: graph,
        }
    }

    pub fn search_components(&mut self) {
        let other_count = self.components.isolate_component(&mut self.graph);
        if other_count == 0 {
            return self.search_graph();
        }

        let upper_both = self.upper;
        let mut cost_other = 0;

        if cfg!(not(feature = "incremental")) {
            let lower_both = self.packing.lower;
            self.packing.pack(&self.graph);
            cost_other = lower_both - self.packing.lower;
        }

        self.upper -= cost_other;
        self.search_graph();
        self.upper += cost_other;

        if self.upper == upper_both {
            self.components
                .all_components(&mut self.graph.active, other_count);
            return;
        }
        assert!(upper_both > self.upper);

        let count = self
            .components
            .other_component(&mut self.graph.active, other_count);

        let mut cost = self.upper - self.packing.lower; // how much the component costs on top of the lower bound

        if cfg!(not(feature = "incremental")) {
            cost = self.upper - cost_other;
            self.packing.pack(&self.graph); // i do not know why it has to be recalculated here
                                            // self.packing.lower = cost_other;
        }

        self.upper = upper_both; // the upper bound stays the same because we have not yet solved all components

        let old_len = self.graph.len;
        self.graph.len = self.best.len; // we need to add the solutions
        self.upper -= cost; // remove the cost of the first component
        self.search_components();
        self.upper += cost;
        self.graph.len = old_len; // from here we will overwrite solutions again

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
            if cfg!(feature = "branch-comp") {
                self.search_components()
            } else {
                self.search_graph();
            }
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
            if cfg!(feature = "branch-comp") {
                self.search_components()
            } else {
                self.search_graph();
            }
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
