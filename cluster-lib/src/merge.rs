use std::{
    cmp::{self, min},
    iter::Peekable,
    ops::Neg,
};

use crate::graph::{Edge, Graph, Vertex};

impl Graph {
    // requires edge between vertices to be positive
    pub fn merge(&mut self, v1: u32, v2: u32) -> (u32, u32) {
        let mut edges = Vec::new();
        let mut cost = 0;
        for (mut a, mut b) in self.edge_pairs(v1, v2) {
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

    pub fn edge_pairs(&self, v1: u32, v2: u32) -> impl '_ + Iterator<Item = (Edge, Edge)> {
        MergeEdges {
            a: self.edges(v1).peekable(),
            b: self.edges(v2).peekable(),
        }
        .filter_map(move |(a, b)| match (a, b) {
            (None, None) => unreachable!(),
            (None, Some(b)) => {
                if b.to == v1 {
                    None
                } else {
                    Some((self.missing_edge(v1, b.to), b))
                }
            }
            (Some(a), None) => {
                if a.to == v2 {
                    None
                } else {
                    Some((a, self.missing_edge(v2, a.to)))
                }
            }
            (Some(a), Some(b)) => Some((a, b)),
        })
    }

    fn missing_edge(&self, v1: u32, v2: u32) -> Edge {
        Edge {
            weight: ((self[v1].size * self[v2].size) as i32).neg(),
            to: v2,
            version: u32::MAX,
        }
    }

    pub fn conflict_edges(
        &self,
        v1: u32,
        v2: u32,
    ) -> impl '_ + Iterator<Item = (Option<Edge>, Option<Edge>)> {
        MergeEdges {
            a: self.edges(v1).positive().peekable(),
            b: self.edges(v2).positive().peekable(),
        }
        .filter(move |(a, b)| match (a, b) {
            (None, None) => unreachable!(),
            (None, Some(b)) => b.to != v1,
            (Some(a), None) => a.to != v2,
            (Some(_), Some(_)) => false,
        })
    }

    pub fn merge_cost(&self, v1: u32, v2: u32) -> u32 {
        let mut cost = 0;
        for (mut a, mut b) in self.edge_pairs(v1, v2) {
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
        self.conflict_edges(v1, v2)
            .map(|(a, b)| a.or(b).unwrap().weight)
            .sum::<i32>() as u32
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
