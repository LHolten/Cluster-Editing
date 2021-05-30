use std::cmp::min;

use crate::graph::{Edge, Graph, Vertex};

impl Graph {
    // requires edge between vertices to be positive
    pub fn merge(&mut self, v1: usize, v2: usize) -> (usize, i32) {
        let index = self.clusters.iter().last().unwrap() + 1;
        self.clusters.remove(v1);
        self.clusters.remove(v2);

        let mut cost = 0;
        let graph = self as *mut Graph;

        // let mut total_positive = 0;
        for pair in self.all_edges(v1, v2) {
            if (pair.edge1.weight > 0) ^ (pair.edge2.weight > 0) {
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

        self.clusters.insert(index);
        self[v1].merged = Some(index);
        self[v2].merged = Some(index);
        (index, cost)
    }

    pub fn un_merge(&mut self, v1: usize, v2: usize, index: usize) {
        self[v1].merged = None;
        self[v2].merged = None;

        self.clusters.remove(index);
        self.clusters.insert(v1);
        self.clusters.insert(v2);
    }

    pub fn all_edges(&self, v1: usize, v2: usize) -> impl '_ + Iterator<Item = EdgePair> {
        AllEdges {
            vertex1: &self[v1],
            vertex2: &self[v2],
            clusters: self.clusters.iter(),
        }
    }

    pub fn conflict_edges(&self, v1: usize, v2: usize) -> impl '_ + Iterator<Item = EdgePair> {
        self.all_edges(v1, v2)
            .filter(|pair| (pair.edge1.weight > 0) ^ (pair.edge2.weight > 0))
    }

    pub fn two_edges(&self, v1: usize, v2: usize) -> impl '_ + Iterator<Item = EdgePair> {
        self.all_edges(v1, v2)
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
    clusters: bit_set::Iter<'a, u32>,
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
        let to = self.clusters.next()?;
        Some(EdgePair {
            to,
            edge1: self.vertex1[to],
            edge2: self.vertex2[to],
        })
    }
}
