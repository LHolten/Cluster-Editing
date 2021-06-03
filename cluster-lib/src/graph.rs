use std::cell::Cell;
use std::mem::replace;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub clusters: Vec<usize>,
    pub len: usize,
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        let mut self_clusters = self.clusters.clone();
        let mut other_clusters = other.clusters.clone();
        self_clusters.sort_unstable();
        other_clusters.sort_unstable();
        if self_clusters != other_clusters {
            return false;
        }
        for (i1, v1) in self.clusters(0) {
            for (_, v2) in self.clusters(i1) {
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
            vertices: self.vertices.clone(),
            clusters: self.clusters.clone(),
            len: self.len,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.vertices.clone_from_slice(&source.vertices);
        self.clusters.clone_from(&source.clusters);
        self.len = source.len;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Vertex {
    pub size: i32,
    pub merged: Option<usize>,
    pub edges: Vec<Edge>,
    pub marked: Cell<bool>,
}

impl Clone for Vertex {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            merged: self.merged,
            edges: self.edges.clone(),
            marked: self.marked.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.size = source.size;
        self.merged = source.merged;
        self.edges.copy_from_slice(&source.edges);
        self.marked = source.marked.clone()
    }
}

impl Vertex {
    pub fn new(size: usize) -> Self {
        Self {
            size: 1,
            merged: None,
            edges: vec![Edge::new(-1); size * 2],
            marked: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub weight: i32,
    pub fixed: bool,
    pub marked: bool,
}

impl Edge {
    pub fn new(weight: i32) -> Self {
        Self {
            weight,
            fixed: false,
            marked: Default::default(),
        }
    }

    pub fn none() -> Self {
        Self {
            weight: -i32::MAX,
            fixed: true,
            marked: Default::default(),
        }
    }
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Self {
            vertices: vec![Vertex::new(size); size * 2],
            clusters: (0..size).collect(),
            len: size,
        }
    }

    pub fn cut(&mut self, v1: usize, v2: usize) -> Edge {
        let edge = replace(&mut self[v1][v2], Edge::none());
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

    pub fn positive(&self, index: usize, from: usize) -> impl '_ + Iterator<Item = (usize, usize)> {
        let edges = &self[index];
        self.clusters(from)
            .filter(move |(_, to)| edges[*to].weight > 0)
    }

    pub fn clusters(&self, from: usize) -> Clusters {
        Clusters {
            from,
            clusters: &self.clusters,
        }
    }
}

impl Index<usize> for Graph {
    type Output = Vertex;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.vertices.get_unchecked(index) }
    }
}

impl IndexMut<usize> for Graph {
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

pub struct Clusters<'a> {
    from: usize,
    clusters: &'a Vec<usize>,
}

impl<'a> Iterator for Clusters<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let res = *self.clusters.get(self.from)?;
        self.from += 1;
        Some((self.from, res))
    }
}
