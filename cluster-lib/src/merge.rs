use std::{
    borrow::Borrow,
    cmp::{self, min},
    iter::Peekable,
    ops::{self, Neg},
    slice::Iter,
};

use crate::graph::{Edge, Vertex};

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
        self.filter(|(a, b)| b.is_none() || (a.count <= 0) ^ (b.unwrap().count <= 0))
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
            (Some(a), Some(b)) => match a.index.cmp(&b.index) {
                cmp::Ordering::Less => Some((*self.b.next().unwrap(), None)),
                cmp::Ordering::Equal => {
                    Some((*self.a.next().unwrap(), Some(*self.b.next().unwrap())))
                }
                cmp::Ordering::Greater => Some((*self.a.next().unwrap(), None)),
            },
        }
    }
}

impl<V: Borrow<Vertex>> ops::Add<V> for &Vertex {
    type Output = Vertex;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.borrow();
        let edges = Vec::new();
        let mut cost = self.cost + rhs.cost;
        for (a, mut b) in MergeEdges::new(&self.edges, &rhs.edges) {
            if b.is_none() {
                if a.index == self.index {
                    if a.count < 0 {
                        cost += a.count.neg() as u32;
                    }
                    continue;
                }
                if a.index == rhs.index {
                    continue;
                }
                b = Some(Edge {
                    index: a.index,
                    count: ((self.size * rhs.size) as i32).neg(),
                })
            }
            let b = b.unwrap();
            if (a.count < 0) ^ (b.count < 0) {
                cost += min(a.count.abs(), b.count.abs()) as u32;
            }
            edges.push(Edge {
                count: add_edges(a.count, b.count),
                index: a.index,
            })
        }

        Vertex {
            index: min(self.index, rhs.index),
            size: self.size + rhs.size,
            cost,
            edges,
        }
    }
}

// -i32::MAX means that the edge is not allowed
fn add_edges(a: i32, b: i32) -> i32 {
    if a == -i32::MAX || b == -i32::MAX {
        -i32::MAX
    } else {
        a + b
    }
}
