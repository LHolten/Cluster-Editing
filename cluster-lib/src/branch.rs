use std::cmp::min;

use crate::graph::Graph;

impl Graph {
    // find best edge to split on in O(n + m^2) time
    // edge is positive
    // there is at least one conflict
    pub fn best_edge(&mut self) -> Option<(u32, u32)> {
        let mut best = None;
        let mut best_count = 0;

        for vertex in self.clusters() {
            for edge in self.edges(vertex).positive() {
                if edge.to >= vertex {
                    break;
                }

                let count = self.merge_edges(vertex, edge.to).conflicts().count();
                // let count = self.merge_cost(vertex, edge.to);
                // if count >= 2 {
                //     count = max(count, edge.weight as u32);
                // }
                if count > best_count {
                    best_count = count;
                    best = Some((vertex, edge.to))
                }
            }
        }
        best
    }

    pub fn triple(&mut self) -> Option<(u32, u32, u32, i32)> {
        let mut best = None;
        let mut best_count = 0;

        for vertex in self.clusters() {
            for edge in self.edges(vertex).negative() {
                if edge.to >= vertex {
                    break;
                }
                let mut edge = edge.clone();
                if edge.version != u32::MAX {
                    edge.weight = -i32::MAX
                }

                for (a, b) in self.merge_edges(vertex, edge.to).two_edges() {
                    let count = min(edge.weight.abs(), min(a.weight.abs(), b.weight.abs()));
                    if count > best_count {
                        best_count = count;
                        best = Some((vertex, edge.to, a.to, edge.weight))
                    }
                }
            }
        }
        best
    }
}
