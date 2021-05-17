use std::{cell::Cell, ops, slice::Iter};

use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VertexIndex(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    versions: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vertex {
    pub merged: Option<VertexIndex>,
    pub size: u32,
    pub edges: Vec<Edge>,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            merged: None,
            size: 1,
            edges: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub weight: i32,
    pub to: VertexIndex,
    pub version: u32,
    pub marked: Cell<bool>,
}

impl Edge {
    pub fn new(to: VertexIndex) -> Self {
        Self {
            weight: 1,
            to,
            version: u32::MAX,
            marked: Default::default(),
        }
    }

    pub fn positive(&self) -> bool {
        self.weight > 0 && self.version == u32::MAX
    }

    pub fn negative(&self) -> bool {
        self.weight <= 0 || self.version != u32::MAX
    }
}

impl Graph {
    pub fn new(size: u32) -> Self {
        Self {
            vertices: vec![Vertex::default(); size as usize],
            versions: vec![0],
        }
    }

    pub fn snapshot(&mut self) {
        self.versions.push(self.vertices.len() as u32)
    }

    pub fn rollback(&mut self) {
        let len = self.versions.pop().unwrap();
        let version = self.versions.len() as u32;
        self.vertices.truncate(len as usize);

        for vertex in &mut self.vertices {
            if vertex.merged.is_some() && vertex.merged.unwrap().0 >= len {
                vertex.merged = None
            }
            let mut edge_len = vertex.edges.len();
            for (i, edge) in vertex.edges.iter_mut().enumerate() {
                if edge.to.0 >= len {
                    edge_len = i;
                    break;
                }
                if edge.version > version {
                    edge.version = u32::MAX
                }
            }
            vertex.edges.truncate(edge_len)
        }
    }

    // requires edge between vertices to be positive
    pub fn cut(&mut self, v1: VertexIndex, v2: VertexIndex) -> u32 {
        let version = self.versions.len() as u32;
        let edges = &mut self[v1].edges;
        let index = edges.binary_search_by_key(&v2, |e| e.to).unwrap();
        edges[index].version = version;
        let edges = &mut self[v2].edges;
        let index = edges.binary_search_by_key(&v1, |e| e.to).unwrap();
        edges[index].version = version;
        debug_assert!(edges[index].weight > 0);
        edges[index].weight as u32
    }

    pub fn clusters(&self) -> ClusterIter<'_> {
        ClusterIter {
            graph: &self,
            range: 0..self.vertices.len() as u32,
        }
    }

    pub fn edges(&self, index: VertexIndex) -> EdgeIter<'_> {
        EdgeIter {
            graph: self,
            edges: self[index].edges.iter(),
        }
    }
}

impl Index<VertexIndex> for Graph {
    type Output = Vertex;

    fn index(&self, index: VertexIndex) -> &Self::Output {
        &self.vertices[index.0 as usize]
    }
}

impl IndexMut<VertexIndex> for Graph {
    fn index_mut(&mut self, index: VertexIndex) -> &mut Self::Output {
        &mut self.vertices[index.0 as usize]
    }
}

pub struct ClusterIter<'a> {
    graph: &'a Graph,
    range: ops::Range<u32>,
}

impl<'a> Iterator for ClusterIter<'a> {
    type Item = VertexIndex;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.range.next()?;
            if self.graph.vertices[index as usize].merged.is_none() {
                return Some(VertexIndex(index));
            }
        }
    }
}

#[derive(Clone)]
pub struct EdgeIter<'a> {
    graph: &'a Graph,
    edges: Iter<'a, Edge>,
}

impl<'a> Iterator for EdgeIter<'a> {
    type Item = &'a Edge;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let edge = self.edges.next()?;
            if self.graph[edge.to].merged.is_none() {
                return Some(edge);
            }
        }
    }
}

impl<'a> EdgeIter<'a> {
    pub fn positive(self) -> impl 'a + Iterator<Item = &'a Edge> {
        self.filter(|e| e.positive())
    }

    pub fn negative(self) -> impl 'a + Iterator<Item = &'a Edge> {
        self.filter(|e| e.negative())
    }
}
