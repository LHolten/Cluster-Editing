use std::{
    cmp::{self, min},
    iter::Peekable,
};

use crate::graph::{Edge, EdgeIter, Graph, Vertex, VertexIndex};

impl Graph {
    // requires edge between vertices to be positive
    pub fn merge(&mut self, v1: VertexIndex, v2: VertexIndex) -> VertexIndex {
        let mut edges = Vec::new();
        for (a, b) in self.merge_edges(v1, v2) {
            edges.push(Edge {
                weight: a.weight + b.weight,
                to: a.to,
                version: min(a.version, b.version),
                marked: Default::default(),
            });
        }

        let index = VertexIndex(self.vertices.len() as u32);
        for edge in &edges {
            self[edge.to].edges.push(Edge {
                weight: edge.weight,
                to: index,
                version: edge.version,
                marked: Default::default(),
            })
        }

        self.vertices.push(Vertex {
            merged: None,
            size: self[v1].size + self[v2].size,
            edges,
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
    ) -> impl '_ + Iterator<Item = (Option<&'_ Edge>, Option<&'_ Edge>)> {
        AllEdges {
            a: self.edges(v1).peekable(),
            b: self.edges(v2).peekable(),
        }
        .filter(|(a, b)| {
            a.map(|e| e.weight > 0).unwrap_or(false) ^ b.map(|e| e.weight > 0).unwrap_or(false)
        })
        .filter(move |(a, b)| {
            a.map(|e| e.to != v2).unwrap_or(true) && b.map(|e| e.to != v1).unwrap_or(true)
        })
    }

    pub fn merge_cost(&self, v1: VertexIndex, v2: VertexIndex) -> u32 {
        let mut cost = 0;
        for (a, b) in self.conflict_edges(v1, v2) {
            let mut new_cost = i32::MAX;
            if let Some(a) = a {
                new_cost = a.weight.abs()
            }
            if let Some(b) = b {
                new_cost = min(new_cost, b.weight.abs())
            }
            cost += new_cost;
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

pub struct AllEdges<'a> {
    a: Peekable<EdgeIter<'a>>,
    b: Peekable<EdgeIter<'a>>,
}

impl<'a> Iterator for AllEdges<'a> {
    type Item = (Option<&'a Edge>, Option<&'a Edge>);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.peek(), self.b.peek()) {
            (None, None) => None,
            (None, Some(_)) => Some((None, self.b.next())),
            (Some(_), None) => Some((self.a.next(), None)),
            (Some(a), Some(b)) => match a.to.cmp(&b.to) {
                cmp::Ordering::Equal => Some((self.a.next(), self.b.next())),
                cmp::Ordering::Less => Some((self.a.next(), None)),
                cmp::Ordering::Greater => Some((None, self.b.next())),
            },
        }
    }
}
