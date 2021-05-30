use std::{
    cmp::{self, min},
    mem::replace,
    slice::Iter,
};

use crate::graph::{Edge, Graph, Vertex, VertexIndex};

impl Graph {
    // requires edge between vertices to be positive
    pub fn merge(&mut self, v1: VertexIndex, v2: VertexIndex) -> VertexIndex {
        let mut edges = Vec::new();
        let mut total_positive = 0;
        for pair in self.all_edges(v1, v2) {
            let weight = pair.a_weight + pair.b_weight;
            if weight > 0 {
                total_positive += weight;
            }
            edges.push(Edge {
                weight,
                to: pair.to,
                version: min(pair.a_version, pair.b_version),
            });
        }
        // for edge in &mut edges {
        //     if edge.weight <= -total_positive {
        //         edge.version = 0;
        //     }
        // }

        let index = VertexIndex(self.vertices.len() as u32);
        for edge in &edges {
            self[edge.to].edges.push(Edge {
                weight: edge.weight,
                to: index,
                version: edge.version,
            })
        }

        self.vertices.push(Vertex {
            size: self[v1].size + self[v2].size,
            merged: None,
            edges,
            marked: Default::default(),
        });
        self[v1].merged = Some(index);
        self[v2].merged = Some(index);
        index
    }

    pub fn all_edges(
        &self,
        v1: VertexIndex,
        v2: VertexIndex,
    ) -> impl '_ + Iterator<Item = EdgePair> {
        let (mut a, mut b) = (self[v1].edges.iter(), self[v2].edges.iter());
        AllEdges {
            graph: self,
            a_index: v1,
            a_size: self[v1].size,
            a_next: a.next().unwrap_or(NO_EDGE),
            a,
            b_index: v2,
            b_size: self[v2].size,
            b_next: b.next().unwrap_or(NO_EDGE),
            b,
        }
    }

    pub fn conflict_edges(
        &self,
        v1: VertexIndex,
        v2: VertexIndex,
    ) -> impl '_ + Iterator<Item = EdgePair> {
        self.all_edges(v1, v2)
            .filter(move |pair| (pair.a_weight > 0) ^ (pair.b_weight > 0))
    }

    pub fn merge_cost(&self, v1: VertexIndex, v2: VertexIndex) -> u32 {
        let mut cost = 0;
        for pair in self.conflict_edges(v1, v2) {
            cost += min(pair.a_weight.abs(), pair.b_weight.abs());
        }
        cost as u32
    }
}

const NO_EDGE: &Edge = &Edge {
    weight: 1,
    to: VertexIndex(u32::MAX),
    version: u32::MAX,
};

pub struct AllEdges<'a> {
    graph: &'a Graph,
    a_index: VertexIndex,
    a_size: i32,
    a_next: &'a Edge,
    a: Iter<'a, Edge>,
    b_index: VertexIndex,
    b_size: i32,
    b_next: &'a Edge,
    b: Iter<'a, Edge>,
}

pub struct EdgePair {
    pub to: VertexIndex,
    pub a_weight: i32,
    pub a_version: u32,
    pub b_weight: i32,
    pub b_version: u32,
}

impl<'a> AllEdges<'a> {
    pub fn edge_pair(&self, a: Option<&Edge>, b: Option<&Edge>) -> Option<EdgePair> {
        if a.map_or(false, |e| e.to == self.b_index) || b.map_or(false, |e| e.to == self.a_index) {
            return None;
        }
        let to = a.or(b).unwrap().to;
        let c = &self.graph[to];
        if c.merged.is_some() {
            return None;
        }
        Some(EdgePair {
            to,
            a_weight: a
                .filter(|e| e.version == u32::MAX)
                .map_or(-self.a_size * c.size, |e| e.weight),
            a_version: a.map_or(u32::MAX, |e| e.version),
            b_weight: b
                .filter(|e| e.version == u32::MAX)
                .map_or(-self.b_size * c.size, |e| e.weight),
            b_version: b.map_or(u32::MAX, |e| e.version),
        })
    }
}

impl<'a> Iterator for AllEdges<'a> {
    type Item = EdgePair;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.a_next.to.cmp(&self.b_next.to) {
                cmp::Ordering::Equal => {
                    if self.a_next.to == NO_EDGE.to {
                        return None;
                    }
                    let a = replace(&mut self.a_next, self.a.next().unwrap_or(NO_EDGE));
                    let b = replace(&mut self.b_next, self.b.next().unwrap_or(NO_EDGE));
                    if let Some(pair) = self.edge_pair(Some(a), Some(b)) {
                        return Some(pair);
                    };
                }
                cmp::Ordering::Less => {
                    let a = replace(&mut self.a_next, self.a.next().unwrap_or(NO_EDGE));
                    if let Some(pair) = self.edge_pair(Some(a), None) {
                        return Some(pair);
                    };
                }
                cmp::Ordering::Greater => {
                    let b = replace(&mut self.b_next, self.b.next().unwrap_or(NO_EDGE));
                    if let Some(pair) = self.edge_pair(None, Some(b)) {
                        return Some(pair);
                    };
                }
            }
        }
    }
}
