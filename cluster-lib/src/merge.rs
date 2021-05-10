use std::{
    cmp::{self, min},
    iter::Peekable,
    ops::Neg,
    slice::Iter,
};

use crate::graph::{Edge, Graph, Vertex};

#[derive(Clone)]
pub struct MergeEdges<'a> {
    a: Peekable<Iter<'a, Edge>>,
    b: Peekable<Iter<'a, Edge>>,
}

impl<'a> MergeEdges<'a> {
    pub fn new<I>(a: I, b: I) -> Self
    where
        I: IntoIterator<Item = &'a Edge, IntoIter = Iter<'a, Edge>>,
    {
        MergeEdges {
            a: a.into_iter().peekable(),
            b: b.into_iter().peekable(),
        }
    }

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

impl Graph {
    // adds together a pair of vertices, however it does not fix the edges towards this vertex
    pub fn merge(&mut self, v1: u32, v2: u32) -> u32 {
        let lhs = &self[v1];
        let rhs = &self[v2];

        let mut edges = Vec::new();
        let mut cost = 0;
        for (a, mut b) in MergeEdges::new(&lhs.edges, &rhs.edges) {
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
                    to: a.to,
                    weight: ((lhs.size * rhs.size) as i32).neg(),
                })
            }
            let b = b.unwrap();
            if (a.weight < 0) ^ (b.weight < 0) {
                cost += min(a.weight.abs(), b.weight.abs()) as u32;
            }
            edges.push(Edge {
                weight: add_edges(a.weight, b.weight),
                to: a.to,
            })
        }

        let new_index = self.vertices.len() as u32;
        self.vertices.push(Vertex {
            merged: None,
            size: lhs.size + rhs.size,
            edges,
        });
        self.connect(v1, new_index);
        self.connect(v2, new_index);

        cost
    }
}

fn organize_edges(edges: &Vec<Edge>, graph: &Graph) -> Vec<Edge> {
    let new = Vec::new();
    for edge in edges {
        let to = graph.find(edge.to);
    }
    new
}

// -i32::MAX means that the edge is not allowed
fn add_edges(a: i32, b: i32) -> i32 {
    if a == -i32::MAX || b == -i32::MAX {
        -i32::MAX
    } else {
        a + b
    }
}
