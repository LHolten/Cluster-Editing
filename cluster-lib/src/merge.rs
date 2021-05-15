use std::{
    cmp::{self, min},
    iter::Peekable,
};

use crate::graph::{Edge, Graph, Vertex};

impl Graph {
    // requires edge between vertices to be positive
    pub fn merge(&mut self, v1: u32, v2: u32) -> (u32, u32) {
        let mut edges = Vec::new();
        for (a, b) in self.merge_edges(v1, v2) {
            edges.push(Edge {
                weight: a.weight + b.weight,
                to: a.to,
                version: min(a.version, b.version),
            });
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
        (self.merge_cost(v1, v2), index)
    }

    pub fn un_merge(&mut self, v1: u32, v2: u32) {
        self.vertices.pop().unwrap();
        self[v1].merged = None;
        self[v2].merged = None;
    }

    pub fn merge_edges(&self, v1: u32, v2: u32) -> MergeEdges<impl '_ + Iterator<Item = Edge>> {
        MergeEdges {
            a: self.edges(v1).peekable(),
            b: self.edges(v2).peekable(),
        }
    }

    pub fn merge_cost(&self, v1: u32, v2: u32) -> u32 {
        let mut cost = 0;
        for (mut a, mut b) in self.merge_edges(v1, v2) {
            if a.version != u32::MAX {
                a.weight = -i32::MAX
            }
            if b.version != u32::MAX {
                b.weight = -i32::MAX
            }
            if (a.weight <= 0) ^ (b.weight <= 0) {
                cost += min(a.weight.abs(), b.weight.abs());
            }
        }
        cost as u32
    }

    pub fn merge_rho(&self, v1: u32, v2: u32) -> u32 {
        self.merge_edges(v1, v2)
            .conflicts()
            .map(|(a, b)| b.weight.abs())
            .sum::<i32>() as u32
    }
}

#[derive(Clone)]
pub struct MergeEdges<T: Iterator<Item = Edge>> {
    a: Peekable<T>,
    b: Peekable<T>,
}

impl<T: Iterator<Item = Edge>> Iterator for MergeEdges<T> {
    type Item = (Edge, Edge);

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

impl<T: Iterator<Item = Edge>> MergeEdges<T> {
    pub fn conflicts(self) -> impl Iterator<Item = (Edge, Edge)> {
        self.filter(|(a, b)| {
            (a.version == u32::MAX && a.weight > 0) ^ (b.version == u32::MAX && b.weight > 0)
        })
    }
}
