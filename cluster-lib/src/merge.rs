use std::{
    cmp::{self, min},
    iter::Peekable,
    ops::Neg,
};

use crate::graph::{Edge, EdgeIter, Graph, Vertex};

impl Graph {
    pub fn merge(&mut self, v1: u32, v2: u32) -> u32 {
        let index = self.vertices.len() as u32;
        let mut edges = Vec::new();
        let mut cost = 0;
        for (a, mut b) in self.merge_edges(v1, v2) {
            if b.is_none() {
                if a.to == v1 {
                    if a.weight < 0 {
                        cost += a.weight.neg() as u32;
                    }
                    continue;
                }
                if a.to == v2 {
                    continue;
                }
                b = Some(Edge {
                    weight: ((self[v1].size * self[v2].size) as i32).neg(),
                    to: a.to,
                    version: u32::MAX,
                })
            }
            let b = b.unwrap();
            if (a.weight < 0) ^ (b.weight < 0) {
                cost += min(a.weight.abs(), b.weight.abs()) as u32;
            }
            edges.push(Edge {
                weight: a.weight + b.weight,
                to: a.to,
                version: min(a.version, b.version),
            });
            self[a.to].edges.push(Edge {
                weight: a.weight + b.weight,
                to: index,
                version: min(a.version, b.version),
            })
        }

        self.vertices.push(Vertex {
            merged: None,
            size: self[v1].size + self[v2].size,
            edges,
        });
        self.connect(v1, index);
        self.connect(v2, index);

        cost
    }

    fn merge_edges(&self, v1: u32, v2: u32) -> Vec<(Edge, Option<Edge>)> {
        MergeEdges {
            a: self.edges(v1).peekable(),
            b: self.edges(v2).peekable(),
        }
        .collect()
    }
}

#[derive(Clone)]
pub struct MergeEdges<'a> {
    a: Peekable<EdgeIter<'a>>,
    b: Peekable<EdgeIter<'a>>,
}

impl<'a> MergeEdges<'a> {
    // TODO check this
    pub fn count_diff(&mut self) -> u32 {
        self.filter(|(a, b)| b.is_none() || (a.weight <= 0) ^ (b.unwrap().weight <= 0))
            .count() as u32
    }
}

impl<'a> Iterator for MergeEdges<'a> {
    type Item = (Edge, Option<Edge>);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.peek(), self.b.peek()) {
            (None, None) => None,
            (None, Some(_)) => Some((*self.b.next().unwrap(), None)),
            (Some(_), None) => Some((*self.a.next().unwrap(), None)),
            (Some(a), Some(b)) => match a.to.cmp(&b.to) {
                cmp::Ordering::Less => Some((*self.a.next().unwrap(), None)),
                cmp::Ordering::Equal => {
                    Some((*self.a.next().unwrap(), Some(*self.b.next().unwrap())))
                }
                cmp::Ordering::Greater => Some((*self.b.next().unwrap(), None)),
            },
        }
    }
}
