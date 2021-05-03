use std::{cmp, ops::Add};
use std::{iter::Peekable, slice::Iter};

use crate::graph::{Edge, Graph, Vertex};

impl Graph {
    fn critical(&self) -> Self {
        let mut new_vertices = Vec::new();
        let mut new_indices = Vec::new();
        for (i, this) in self.0.iter().enumerate() {
            for edge in this.edges {
                let j = edge.index as usize;
                if j > i {
                    new_indices.push(new_vertices.len());
                    new_vertices.push(this.clone());
                    break;
                }
                let that = &self.0[j];
                if this.edges | i == that.edges | j {
                    let index = new_indices[j];
                    new_indices.push(index);
                    new_vertices[index] = &new_vertices[index] + this;
                    break;
                }
            }
        }
        Graph(new_vertices)
    }
}

impl Add<&Vertex> for &Vertex {
    type Output = Vertex;

    fn add(self, rhs: &Vertex) -> Self::Output {
        Vertex {
            size: self.size + rhs.size,
            edges: AddEdges(MergeEdges::new(&self.edges, &rhs.edges)).collect(), // need to filter out the inner edges at some point
        }
    }
}

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
            (None, Some(v)) => Some((self.b.next().unwrap().clone(), None)),
            (Some(v), None) => Some((self.a.next().unwrap().clone(), None)),
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

struct AddEdges<'a>(MergeEdges<'a>);

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

fn add_edges(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        0
    } else {
        a + b
    }
}
