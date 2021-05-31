use std::cmp::{max, min};

use crate::graph::{Clusters, Edge, Graph, Vertex};

impl Graph {
    // requires edge between vertices to be positive
    pub fn merge(&mut self, v1: usize, v2: usize) -> (usize, i32) {
        let mut index = 0;
        self.clusters.retain(|&v| {
            index = max(index, v);
            v != v1 && v != v2
        });
        index += 1;

        let mut cost = 0;
        let graph = self as *mut Graph;

        // let mut total_positive = 0;
        for pair in self.all_edges(v1, v2, 0) {
            if -pair.edge1.weight ^ -pair.edge2.weight < 0 {
                cost += min(pair.edge1.weight.abs(), pair.edge2.weight.abs());
            }
            let edge = if (pair.edge1.deleted) || (pair.edge2.deleted) {
                Edge::none()
            } else {
                Edge {
                    weight: pair.edge1.weight + pair.edge2.weight,
                    deleted: pair.edge1.deleted || pair.edge2.deleted,
                }
            };
            // if weight > 0 {
            //     total_positive += weight;
            // }
            unsafe {
                (&mut *graph)[index][pair.to] = edge;
                (&mut *graph)[pair.to][index] = edge;
            }
        }
        // for edge in &mut edges {
        //     if edge.weight <= -total_positive {
        //         edge.version = 0;
        //     }
        // }
        self.clusters.push(index);
        self[v1].merged = Some(index);
        self[v2].merged = Some(index);
        (index, cost)
    }

    pub fn un_merge(&mut self, v1: usize, v2: usize, index: usize) {
        self[v1].merged = None;
        self[v2].merged = None;

        let pos = self.clusters(0).find(|(_, v)| *v == index).unwrap();
        self.clusters.swap_remove(pos.0 - 1);
        self.clusters.push(v1);
        self.clusters.push(v2);
    }

    pub fn all_edges(
        &self,
        v1: usize,
        v2: usize,
        from: usize,
    ) -> impl '_ + Iterator<Item = EdgePair> {
        AllEdges {
            vertex1: &self[v1],
            vertex2: &self[v2],
            clusters: self.clusters(from),
        }
    }

    pub fn conflict_edges(
        &self,
        v1: usize,
        v2: usize,
        from: usize,
    ) -> impl '_ + Iterator<Item = EdgePair> {
        self.all_edges(v1, v2, from)
            .filter(|pair| -pair.edge1.weight ^ -pair.edge2.weight < 0)
    }

    pub fn two_edges(
        &self,
        v1: usize,
        v2: usize,
        from: usize,
    ) -> impl '_ + Iterator<Item = EdgePair> {
        self.all_edges(v1, v2, from)
            .filter(|pair| pair.edge1.weight > 0 && pair.edge2.weight > 0)
    }

    // pub fn merge_cost(&self, v1: VertexIndex, v2: VertexIndex) -> u32 {
    //     let mut cost = 0;
    //     for pair in self.conflict_edges(v1, v2) {
    //         cost += min(pair.edge1.weight.abs(), pair.edge2.weight.abs());
    //     }
    //     cost as u32
    // }
}

pub struct AllEdges<'a> {
    vertex1: &'a Vertex,
    vertex2: &'a Vertex,
    clusters: Clusters<'a>,
}

pub struct EdgePair {
    pub to: usize,
    pub edge1: Edge,
    pub edge2: Edge,
}

impl<'a> Iterator for AllEdges<'a> {
    type Item = EdgePair;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let (_, to) = self.clusters.next()?;
        Some(EdgePair {
            to,
            edge1: unsafe { *self.vertex1.edges.get_unchecked(to) },
            edge2: unsafe { *self.vertex2.edges.get_unchecked(to) },
        })
    }
}
