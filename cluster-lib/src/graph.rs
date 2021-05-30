use std::cell::Cell;

use std::ops::{Index, IndexMut};

use bit_set::BitSet;

#[derive(Debug, Clone)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub clusters: BitSet,
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        if self.clusters != other.clusters {
            return false;
        }
        for vertex1 in self.clusters.iter() {
            for vertex2 in self.clusters.iter() {
                if self[vertex1][vertex2] != other[vertex1][vertex2] {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vertex {
    pub size: i32,
    pub merged: Option<usize>,
    pub edges: Vec<Edge>,
    pub marked: Cell<bool>,
}

impl Vertex {
    pub fn new(size: usize) -> Self {
        Self {
            size: 1,
            merged: None,
            edges: vec![Edge::new(-1); size * 2 - 1],
            marked: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub weight: i32,
    pub deleted: bool,
}

impl Edge {
    pub fn new(weight: i32) -> Self {
        Self {
            weight,
            deleted: false,
        }
    }

    pub fn none() -> Self {
        Self {
            weight: -i32::MAX,
            deleted: true,
        }
    }
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Self {
            vertices: vec![Vertex::new(size); size * 2 - 1],
            clusters: (0..size).collect(),
        }
    }

    pub fn cut(&mut self, v1: usize, v2: usize) -> Edge {
        let edge = self[v1][v2];
        self[v1][v2] = Edge::none();
        self[v2][v1] = Edge::none();
        edge
    }

    pub fn un_cut(&mut self, v1: usize, v2: usize, edge: Edge) {
        self[v1][v2] = edge;
        self[v2][v1] = edge;
    }

    pub fn root(&self, index: usize) -> usize {
        if let Some(new_index) = self[index].merged {
            self.root(new_index)
        } else {
            index
        }
    }

    pub fn positive(&self, index: usize) -> impl '_ + Iterator<Item = usize> {
        let edges = &self[index];
        self.clusters.iter().filter(move |to| edges[*to].weight > 0)
    }
}

impl Index<usize> for Graph {
    type Output = Vertex;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl IndexMut<usize> for Graph {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vertices[index]
    }
}

impl Index<usize> for Vertex {
    type Output = Edge;

    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

impl IndexMut<usize> for Vertex {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.edges[index]
    }
}
