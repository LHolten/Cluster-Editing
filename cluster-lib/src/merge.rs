use std::{
    cmp::{self, min},
    iter::Peekable,
    ops::Neg,
};

use crate::graph::{Edge, EdgeIter, Graph, Vertex};

impl Graph {
    // requires edge between vertices to be positive
    pub fn merge(&mut self, v1: u32, v2: u32) -> (u32, u32) {
        let index = self.vertices.len() as u32;
        let mut edges = Vec::new();
        let mut cost = 0;
        for (mut a, mut b) in self.merge_edges(v1, v2) {
            match (a, b) {
                (None, Some(b)) => {
                    if b.to == v1 {
                        continue;
                    }
                    a = Some(Edge {
                        weight: ((self[v1].size * self[b.to].size) as i32).neg(),
                        ..b
                    })
                }
                (Some(a), None) => {
                    if a.to == v2 {
                        continue;
                    }
                    b = Some(Edge {
                        weight: ((self[v2].size * self[a.to].size) as i32).neg(),
                        ..a
                    })
                }
                _ => {}
            }
            let a = a.unwrap();
            let b = b.unwrap();
            if a.weight * b.weight < 0 {
                cost += min(a.weight.abs(), b.weight.abs()) as u32;
            }
            edges.push(Edge {
                weight: a.weight + b.weight,
                to: a.to,
                version: min(a.version, b.version),
            });
        }

        for edge in &edges {
            self[edge.to].edges.push(Edge {
                weight: edge.weight,
                to: index,
                version: edge.version,
            })
        }

        self.vertices.push(Vertex {
            merged: None,
            size: self[v1].size + self[v2].size,
            edges,
        });
        self[v1].merged = Some(index);
        self[v2].merged = Some(index);
        (cost, index)
    }

    pub fn merge_edges(&self, v1: u32, v2: u32) -> MergeEdges<'_> {
        MergeEdges {
            a: self.edges(v1).peekable(),
            b: self.edges(v2).peekable(),
        }
    }
}

#[derive(Clone)]
pub struct MergeEdges<'a> {
    a: Peekable<EdgeIter<'a>>,
    b: Peekable<EdgeIter<'a>>,
}

impl<'a> MergeEdges<'a> {
    pub fn count_diff(&mut self) -> u32 {
        self.filter(|(a, b)| match (a, b) {
            (None, None) => false,
            (None, Some(b)) => b.weight > 0,
            (Some(a), None) => a.weight > 0,
            (Some(a), Some(b)) => a.weight * b.weight < 0,
        })
        .count() as u32
    }
}

impl<'a> Iterator for MergeEdges<'a> {
    type Item = (Option<Edge>, Option<Edge>);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.peek(), self.b.peek()) {
            (None, None) => None,
            (None, Some(_)) => Some((None, self.b.next())),
            (Some(_), None) => Some((self.a.next(), None)),
            (Some(a), Some(b)) => match a.to.cmp(&b.to) {
                cmp::Ordering::Less => Some((self.a.next(), None)),
                cmp::Ordering::Equal => Some((self.a.next(), self.b.next())),
                cmp::Ordering::Greater => Some((None, self.b.next())),
            },
        }
    }
}
