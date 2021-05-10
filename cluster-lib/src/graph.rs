use std::{
    ops,
    slice::{Iter, IterMut},
};

use std::ops::{Index, IndexMut};

pub struct Graph {
    pub vertices: Vec<Vertex>,
    versions: Vec<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct Vertex {
    merged: Option<u32>,
    pub size: u32,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub weight: i32,
    pub to: u32,
    pub version: u32,
}

impl Edge {
    pub fn new(to: u32) -> Self {
        Self {
            weight: 1,
            to,
            version: u32::MAX,
        }
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
        self.vertices.truncate(len as usize);

        for vertex in &mut self.vertices {
            if vertex.merged.is_some() && vertex.merged.unwrap() >= len {
                vertex.merged = None
            }
            let mut edge_len = vertex.edges.len();
            for (i, edge) in vertex.edges.iter_mut().enumerate() {
                if edge.to >= len {
                    edge_len = i;
                    break;
                }
                if edge.version > self.versions.len() as u32 {
                    edge.version = u32::MAX
                }
            }
            vertex.edges.truncate(edge_len)
        }
    }

    pub fn cut(&mut self, v1: u32, v2: u32) -> u32 {
        let mut weight = 0;
        let mut cost = 0;
        let version = self.versions.len() as u32;
        for edge in self.edges_mut(v1) {
            if edge.to == v2 {
                edge.version = version;
                if weight * edge.weight < 0 {
                    cost += edge.weight.abs() as u32
                }
                weight += edge.weight;
            }
        }
        for edge in self.edges_mut(v2) {
            if edge.to == v1 {
                edge.version = version;
            }
        }
        cost
    }

    pub fn clusters(&self) -> ClusterIter<'_> {
        ClusterIter {
            graph: &self,
            range: 0..self.vertices.len() as u32,
        }
    }

    pub fn connect(&mut self, v1: u32, v2: u32) {
        self[v1].merged = Some(v2)
    }

    pub fn edges<'a>(&'a self, index: u32) -> EdgeIter<'a> {
        EdgeIter {
            graph: self,
            edges: self[index].edges.iter(),
        }
    }

    pub fn edges_mut<'a>(&'a mut self, index: u32) -> EdgeIterMut<'a> {
        let graph = self as *const _;
        EdgeIterMut {
            graph: unsafe { &*graph },
            edges: self[index].edges.iter_mut(),
        }
    }
}

impl Index<u32> for Graph {
    type Output = Vertex;

    fn index(&self, index: u32) -> &Self::Output {
        &self.vertices[index as usize]
    }
}

impl IndexMut<u32> for Graph {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.vertices[index as usize]
    }
}

pub struct ClusterIter<'a> {
    graph: &'a Graph,
    range: ops::Range<u32>,
}

impl<'a> Iterator for ClusterIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.range.next()?;
            if self.graph.vertices[index as usize].merged.is_none() {
                return Some(index);
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
            if self.graph.vertices[edge.to as usize].merged.is_none() {
                return Some(edge);
            }
        }
    }
}

pub struct EdgeIterMut<'a> {
    graph: &'a Graph,
    edges: IterMut<'a, Edge>,
}

impl<'a> Iterator for EdgeIterMut<'a> {
    type Item = &'a mut Edge;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let edge = self.edges.next()?;
            if self.graph.vertices[edge.to as usize].merged.is_none() {
                return Some(edge);
            }
        }
    }
}
