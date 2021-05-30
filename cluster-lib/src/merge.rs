use std::{
    cmp::{self, min},
    iter::Peekable,
    mem::replace,
    slice::Iter,
};

use crate::graph::{Edge, EdgeIter, Graph, Vertex, VertexIndex};

impl Graph {
    // requires edge between vertices to be positive
    pub fn merge(&mut self, v1: VertexIndex, v2: VertexIndex) -> VertexIndex {
        let mut edges = Vec::new();
        let mut total_positive = 0;
        for (a, b) in self.merge_edges(v1, v2) {
            let weight = a.weight + b.weight;
            if weight > 0 {
                total_positive += weight;
            }
            edges.push(Edge {
                weight,
                to: a.to,
                version: min(a.version, b.version),
            });
        }
        edges.retain(|e| e.weight > -total_positive);

        let index = VertexIndex(self.vertices.len() as u32);
        for edge in &edges {
            self[edge.to].edges.push(Edge {
                weight: edge.weight,
                to: index,
                version: edge.version,
            })
        }

        self.vertices.push(Vertex {
            merged: None,
            edges,
            marked: Default::default(),
        });
        self[v1].merged = Some(index);
        self[v2].merged = Some(index);
        index
    }

    pub fn merge_edges(&self, v1: VertexIndex, v2: VertexIndex) -> MergeEdges<'_> {
        MergeEdges {
            a: self.edges(v1).peekable(),
            b: self.edges(v2).peekable(),
        }
    }

    pub fn conflict_edges(
        &self,
        v1: VertexIndex,
        v2: VertexIndex,
    ) -> impl '_ + Iterator<Item = EdgePair> {
        let (mut a, mut b) = (self[v1].edges.iter(), self[v2].edges.iter());
        AllEdges {
            a_next: a.next().unwrap_or(NO_EDGE),
            a,
            b_next: b.next().unwrap_or(NO_EDGE),
            b,
        }
        .filter(move |pair| {
            self[pair.to].merged.is_none()
                && (pair.a_weight.map(|w| w > 0).unwrap_or(false)
                    ^ pair.b_weight.map(|w| w > 0).unwrap_or(false))
                && pair.to != v1
                && pair.to != v2
        })
    }

    pub fn merge_cost(&self, v1: VertexIndex, v2: VertexIndex) -> u32 {
        let mut cost = 0;
        for pair in self.conflict_edges(v1, v2) {
            let mut new_cost = i32::MAX;
            if let Some(a_weight) = pair.a_weight {
                new_cost = a_weight.abs()
            }
            if let Some(b_weight) = pair.b_weight {
                new_cost = min(new_cost, b_weight.abs())
            }
            cost += new_cost;
        }
        cost as u32
    }

    pub fn cut_cost(&self, v1: VertexIndex, v2: VertexIndex) -> u32 {
        let mut cost = 0;
        for (a, b) in self.merge_edges(v1, v2).two_edges() {
            cost += min(a.weight, b.weight);
        }
        cost as u32
    }

    // pub fn merge_rho(&self, v1: VertexIndex, v2: VertexIndex) -> u32 {
    //     self.merge_edges(v1, v2)
    //         .conflicts()
    //         .map(|(_, b)| b.weight.abs())
    //         .sum::<i32>() as u32
    // }
}

#[derive(Clone)]
pub struct MergeEdges<'a> {
    a: Peekable<EdgeIter<'a>>,
    b: Peekable<EdgeIter<'a>>,
}

impl<'a> Iterator for MergeEdges<'a> {
    type Item = (&'a Edge, &'a Edge);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            break match (self.a.peek(), self.b.peek()) {
                (Some(a), Some(b)) => match a.to.cmp(&b.to) {
                    cmp::Ordering::Equal => Some((self.a.next().unwrap(), self.b.next().unwrap())),
                    cmp::Ordering::Less => {
                        self.a.next();
                        continue;
                    }
                    cmp::Ordering::Greater => {
                        self.b.next();
                        continue;
                    }
                },
                _ => None,
            };
        }
    }
}

impl<'a> MergeEdges<'a> {
    pub fn two_edges(self) -> impl Iterator<Item = (&'a Edge, &'a Edge)> {
        self.filter(|(a, b)| a.weight > 0 && b.weight > 0)
    }
}

const NO_EDGE: &Edge = &Edge {
    weight: 1,
    to: VertexIndex(u32::MAX),
    version: u32::MAX,
};

pub struct AllEdges<'a> {
    a_next: &'a Edge,
    a: Iter<'a, Edge>,
    b_next: &'a Edge,
    b: Iter<'a, Edge>,
}

pub struct EdgePair {
    pub to: VertexIndex,
    pub a_weight: Option<i32>,
    pub b_weight: Option<i32>,
}

impl EdgePair {
    pub fn new(a: Option<&Edge>, b: Option<&Edge>) -> Self {
        Self {
            to: a.or(b).unwrap().to,
            a_weight: a.and_then(|e| {
                if e.version == u32::MAX {
                    Some(e.weight)
                } else {
                    None
                }
            }),
            b_weight: b.and_then(|e| {
                if e.version == u32::MAX {
                    Some(e.weight)
                } else {
                    None
                }
            }),
        }
    }
}

impl<'a> Iterator for AllEdges<'a> {
    type Item = EdgePair;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.a_next.to.cmp(&self.b_next.to) {
            cmp::Ordering::Equal => {
                if self.a_next.to == VertexIndex(u32::MAX) {
                    return None;
                }
                let a = replace(&mut self.a_next, self.a.next().unwrap_or(NO_EDGE));
                let b = replace(&mut self.b_next, self.b.next().unwrap_or(NO_EDGE));
                Some(EdgePair::new(Some(a), Some(b)))
            }
            cmp::Ordering::Less => {
                let a = replace(&mut self.a_next, self.a.next().unwrap_or(NO_EDGE));
                Some(EdgePair::new(Some(a), None))
            }
            cmp::Ordering::Greater => {
                let b = replace(&mut self.b_next, self.b.next().unwrap_or(NO_EDGE));
                Some(EdgePair::new(None, Some(b)))
            }
        }
    }
}
