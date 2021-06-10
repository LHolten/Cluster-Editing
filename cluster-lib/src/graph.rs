use std::mem::replace;
use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::triple::Triple;

#[derive(Debug)]
pub struct GraphData {
    pub vertices: Vec<Vertex>,
    pub triples: Vec<Triple>,
    pub lower: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub data: GraphData,
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
                if self[v1][v2] != other[v1][v2] {
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
            data: GraphData {
                vertices: self.vertices.clone(),
                triples: self.triples.clone(),
                lower: self.lower,
            },
            active: self.active.clone(),
            len: self.len,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.vertices.clone_from_slice(&source.vertices);
        self.active.clone_from(&source.active);
        self.len = source.len;
        // triples is not cloned
        self.lower = source.lower;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Vertex {
    pub size: i32,
    pub merged: Option<usize>,
    pub edges: Vec<Edge>,
}

impl Clone for Vertex {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            merged: self.merged,
            edges: self.edges.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.size = source.size;
        self.merged = source.merged;
        self.edges.copy_from_slice(&source.edges);
    }
}

impl Vertex {
    pub fn new(size: usize) -> Self {
        Self {
            size: 1,
            merged: None,
            edges: vec![Edge::new(-1); size * 2],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub weight: i32,
    pub fixed: bool,
    pub marked: i32,
    pub conflicts: i32,
}

impl Edge {
    pub fn new(weight: i32) -> Self {
        Self {
            weight,
            fixed: false,
            marked: weight.abs(),
            conflicts: 0,
        }
    }

    pub fn none() -> Self {
        Self {
            weight: -i32::MAX,
            fixed: true,
            marked: i32::MAX,
            conflicts: 0,
        }
    }
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Self {
            data: GraphData {
                vertices: vec![Vertex::new(size); size * 2],
                triples: vec![],
                lower: 0,
            },
            active: (0..size).collect(),
            len: size,
        }
    }

    pub fn positive(&self, v1: usize, from: usize) -> impl '_ + Iterator<Item = (usize, usize)> {
        self.active
            .all(from)
            .filter(move |&(_, v2)| self[v1][v2].weight > 0)
    }

    pub fn cut(&mut self, v1: usize, v2: usize) -> Edge {
        self.data.remove_edge(v1, v2, self.active.all(0));
        let edge = replace(&mut self[v1][v2], Edge::none());
        self[v2][v1] = Edge::none();
        self.data.add_edge(v1, v2, self.active.all(0));
        debug_assert_eq!(
            self[v1][v2].conflicts,
            self.two_edges(v1, v2, 0)
                .filter(|&(_, v)| v != v1 && v != v2)
                .count() as i32
        );
        edge
    }

    pub fn un_cut(&mut self, v1: usize, v2: usize, edge: Edge) {
        self.data.remove_edge(v1, v2, self.active.all(0));
        self[v1][v2] = edge;
        self[v2][v1] = edge;
        self.data.add_edge(v1, v2, self.active.all(0));
    }

    pub fn root(&self, index: usize) -> usize {
        if let Some(new_index) = self[index].merged {
            self.root(new_index)
        } else {
            index
        }
    }
}

impl Deref for Graph {
    type Target = GraphData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Index<usize> for GraphData {
    type Output = Vertex;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.vertices.get_unchecked(index) }
    }
}

impl IndexMut<usize> for GraphData {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.vertices.get_unchecked_mut(index) }
    }
}

impl Index<usize> for Vertex {
    type Output = Edge;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.edges.get_unchecked(index) }
    }
}

impl IndexMut<usize> for Vertex {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.edges.get_unchecked_mut(index) }
    }
}

#[derive(Clone, Copy)]
pub struct Active<'a> {
    from: usize,
    active: &'a Vec<usize>,
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

impl<'a> AllFrom for &'a Vec<usize> {
    type Iter = Active<'a>;

    fn all(self, from: usize) -> Self::Iter {
        Active { from, active: self }
    }
}
