use std::{cmp, iter::Peekable, slice::Iter};

use crate::graph::Edge;

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

pub struct AddEdges<'a>(pub MergeEdges<'a>);

impl<'a> Iterator for AddEdges<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        let (a, b) = self.0.next()?;
        if let Some(b) = b {
            Some(Edge {
                count: add_edges(a.count, b.count),
                index: a.index,
            })
        } else {
            Some(a)
        }
    }
}

// i32::MIN means that the edge is not allowed
fn add_edges(a: i32, b: i32) -> i32 {
    if a == i32::MIN || b == i32::MIN {
        i32::MIN
    } else {
        a + b
    }
}
