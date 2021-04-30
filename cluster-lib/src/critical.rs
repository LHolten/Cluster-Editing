use std::{
    cmp,
    ops::{Add, AddAssign},
};

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
            edges: MergeEdges {
                a: self.edges.clone(),
                b: rhs.edges.clone(),
            }
            .collect(), // need to filter out the inner edges at some point
        }
    }
}

struct MergeEdges {
    a: Vec<Edge>,
    b: Vec<Edge>,
}

impl Iterator for MergeEdges {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        // assert!(self.a.iter().map(|v| v.index).is_sorted());
        let a = self.a.pop();
        let b = self.b.pop();
        if a.is_none() {
            return b;
        }
        if b.is_none() {
            return a;
        }
        let a = a.unwrap();
        let b = b.unwrap();
        match a.index.cmp(&b.index) {
            cmp::Ordering::Less => {
                self.a.push(a);
                Some(b)
            }
            cmp::Ordering::Equal => add_edges(a.number, b.number)
                .map(|number| Edge { number, ..a })
                .or_else(|| self.next()),
            cmp::Ordering::Greater => {
                self.b.push(b);
                Some(a)
            }
        }
    }
}

fn add_edges(a: u32, b: u32) -> Option<u32> {
    if a == 0 || b == 0 {
        Some(0)
    } else {
        let total = a + b;
        if total == 0 {
            None
        } else {
            Some(total)
        }
    }
}
