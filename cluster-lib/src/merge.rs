use std::{
    cmp::{self, min},
    iter::Peekable,
    ops::Neg,
};

use crate::graph::{Edge, EdgeIter, Graph, Vertex};

impl Graph {
    // requires edge between vertices to be positive
    pub fn merge(&mut self, v1: u32, v2: u32) -> (u32, u32) {
        let mut edges = Vec::new();
        let mut cost = 0;
        for (mut a, mut b) in self.merge_edges(v1, v2) {
            if a.is_none() {
                let b = b.unwrap();
                if b.to == v1 {
                    continue;
                }
                a = Some(Edge {
                    weight: ((self[v1].size * self[b.to].size) as i32).neg(),
                    ..b
                })
            }
            if b.is_none() {
                let a = a.unwrap();
                if a.to == v2 {
                    continue;
                }
                b = Some(Edge {
                    weight: ((self[v2].size * self[a.to].size) as i32).neg(),
                    ..a
                })
            }
            let mut a = a.unwrap();
            let mut b = b.unwrap();
            edges.push(Edge {
                weight: a.weight + b.weight,
                to: a.to,
                version: min(a.version, b.version),
            });

            if a.version != u32::MAX {
                a.weight = -i32::MAX
            }
            if b.version != u32::MAX {
                b.weight = -i32::MAX
            }
            if (a.weight <= 0) ^ (b.weight <= 0) {
                cost += min(a.weight.abs(), b.weight.abs()) as u32;
            }
        }

        let index = self.vertices.len() as u32;
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

    pub fn un_merge(&mut self, v1: u32, v2: u32) {
        let v3 = self.vertices.pop().unwrap();
        for edge in v3.edges {
            self[edge.to].edges.pop().unwrap();
        }
        self[v1].merged = None;
        self[v2].merged = None;
    }

    pub fn merge_edges(&self, v1: u32, v2: u32) -> MergeEdges<EdgeIter<'_>> {
        MergeEdges {
            a: self.edges(v1).peekable(),
            b: self.edges(v2).peekable(),
        }
    }

    pub fn conflict_edges(
        &self,
        v1: u32,
        v2: u32,
    ) -> impl '_ + Iterator<Item = (Option<Edge>, Option<Edge>)> {
        MergeEdges {
            a: self.edges(v1).not_none().peekable(),
            b: self.edges(v2).not_none().peekable(),
        }
        .filter(move |(a, b)| match (a, b) {
            (None, None) => unreachable!(),
            (None, Some(b)) => b.weight > 0 && b.to != v1,
            (Some(a), None) => a.weight > 0 && a.to != v2,
            (Some(a), Some(b)) => (a.weight > 0) ^ (b.weight > 0),
        })
    }

    // does not calculate the actual cost of merging
    pub fn merge_cost(&self, v1: u32, v2: u32) -> u32 {
        let mut cost = 0;
        for (a, b) in self.conflict_edges(v1, v2) {
            if a.is_none() {
                cost += b.unwrap().weight;
            } else if b.is_none() {
                cost += a.unwrap().weight;
            } else {
                // this needs to be at least one
                cost += min(a.unwrap().weight.abs(), b.unwrap().weight.abs())
            }
        }
        cost as u32
    }
}

#[derive(Clone)]
pub struct MergeEdges<T: Iterator<Item = Edge>> {
    a: Peekable<T>,
    b: Peekable<T>,
}

impl<T: Iterator<Item = Edge>> Iterator for MergeEdges<T> {
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
