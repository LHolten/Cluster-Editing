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
        self.filter(|(a, b)| b.is_none()).count() as u32
    }
}

impl<'a> Iterator for MergeEdges<'a> {
    type Item = (Edge, Option<Edge>);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.peek(), self.b.peek()) {
            (None, None) => None,
            (None, Some(_)) => Some((self.b.next().unwrap().clone(), None)),
            (Some(_), None) => Some((self.a.next().unwrap().clone(), None)),
            (Some(a), Some(b)) => match a.index.cmp(&b.index) {
                cmp::Ordering::Less => Some((self.b.next().unwrap().clone(), None)),
                cmp::Ordering::Equal => Some((
                    self.a.next().unwrap().clone(),
                    Some(self.b.next().unwrap().clone()),
                )),
                cmp::Ordering::Greater => Some((self.a.next().unwrap().clone(), None)),
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
                number: add_edges(a.number, b.number),
                index: a.index,
            })
        } else {
            Some(a)
        }
    }
}

// 0 means that the edge is not allowed
// this will give problems
fn add_edges(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        0
    } else {
        a + b
    }
}
