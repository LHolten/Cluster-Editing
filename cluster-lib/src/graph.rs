use std::{
    mem::replace,
    ops::{Deref, DerefMut},
};

use crate::matrix::Matrix;

#[derive(Debug)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edges: Matrix<Edge>,
    pub active: Vec<usize>,
    pub len: usize,
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        let mut self_clusters = self.active.clone();
        let mut other_clusters = other.active.clone();
        self_clusters.sort_unstable();
        other_clusters.sort_unstable();
        if self_clusters != other_clusters {
            return false;
        }
        for (i1, v1) in self.active.all(0) {
            for (_, v2) in self.active.all(i1) {
                if self[[v1, v2]] != other[[v1, v2]] {
                    return false;
                }
            }
        }
        true
    }
}

impl Clone for Graph {
    fn clone(&self) -> Self {
        Self {
            vertices: self.vertices.clone(),
            edges: self.edges.clone(),
            active: self.active.clone(),
            len: self.len,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.vertices.copy_from_slice(&source.vertices);
        self.edges.clone_from(&source.edges);
        self.active.clone_from(&source.active);
        self.len = source.len;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Vertex {
    pub merged: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub weight: i32,
    pub fixed: bool,
}

impl Edge {
    pub fn new(weight: i32) -> Self {
        Self {
            weight,
            fixed: false,
        }
    }

    pub fn none() -> Self {
        Self {
            weight: -i32::MAX,
            fixed: true,
        }
    }
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Self {
            vertices: vec![Vertex::default(); size * 2],
            edges: Matrix::new(Edge::new(-1), size * 2),
            active: (0..size).collect(),
            len: size,
        }
    }

    pub fn positive(&self, v1: usize, from: usize) -> impl '_ + Iterator<Item = (usize, usize)> {
        self.active
            .all(from)
            .filter(move |&(_, v2)| self[[v1, v2]].weight > 0)
    }

    pub fn cut(&mut self, v1: usize, v2: usize) -> Edge {
        replace(&mut self[[v1, v2]], Edge::none())
    }

    pub fn un_cut(&mut self, v1: usize, v2: usize, edge: Edge) {
        self[[v1, v2]] = edge;
    }

    pub fn root(&self, index: usize) -> usize {
        if let Some(new_index) = self.vertices[index].merged {
            self.root(new_index)
        } else {
            index
        }
    }
}

impl Deref for Graph {
    type Target = Matrix<Edge>;

    fn deref(&self) -> &Self::Target {
        &self.edges
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.edges
    }
}

#[derive(Clone, Copy)]
pub struct Active<'a> {
    from: usize,
    active: &'a [usize],
}

impl<'a> Iterator for Active<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let res = *self.active.get(self.from)?;
        self.from += 1;
        Some((self.from, res))
    }
}

pub trait AllFrom {
    type Iter;
    fn all(self, from: usize) -> Self::Iter;
}

impl<'a> AllFrom for &'a [usize] {
    type Iter = Active<'a>;

    fn all(self, from: usize) -> Self::Iter {
        Active { from, active: self }
    }
}
