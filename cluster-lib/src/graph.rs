use std::{ops, slice};

use ena::unify::{NoError, PersistentUnificationTable, UnifyKey, UnifyValue};

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub index: u32,
    pub count: i32,
}

impl Edge {
    pub fn new(index: u32) -> Self {
        Edge { index, count: 1 }
    }
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub index: u32,
    pub size: u32,
    pub cost: u32,
    pub edges: Vec<Edge>,
}

impl Vertex {
    pub fn new(index: u32) -> Self {
        Vertex {
            index,
            size: 1,
            cost: 0,
            edges: Vec::new(),
        }
    }
}

impl UnifyValue for Vertex {
    type Error = NoError;

    fn unify_values(value1: &Self, value2: &Self) -> Result<Self, Self::Error> {
        Ok(value1 + value2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VertexKey(u32);

impl UnifyKey for VertexKey {
    type Value = Vertex;

    fn index(&self) -> u32 {
        self.0
    }

    fn from_index(u: u32) -> Self {
        VertexKey(u)
    }

    fn tag() -> &'static str {
        "VertexKey"
    }
}

impl From<u32> for VertexKey {
    fn from(u: u32) -> Self {
        VertexKey::from_index(u)
    }
}

pub type Graph = PersistentUnificationTable<VertexKey>;

pub struct SetIter<'a> {
    graph: &'a Graph,
    range: ops::Range<u32>,
}

impl<'a> SetIter<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        SetIter::up_to(graph, graph.len() as u32)
    }

    pub fn up_to(graph: &'a Graph, index: u32) -> Self {
        SetIter {
            graph,
            range: 0..index,
        }
    }
}

// can maybe be reverted
impl<'a> Iterator for SetIter<'a> {
    type Item = Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.range.next()?;
            let vertex: Vertex = self.graph.inlined_probe_value(index);
            if index == vertex.index {
                return Some(vertex);
            }
        }
    }
}

pub struct EdgeIter<'a> {
    graph: &'a Graph,
    index: u32,
    edges: slice::Iter<'a, Edge>,
}

impl<'a> EdgeIter<'a> {
    pub fn new(graph: &'a Graph, vertex: &'a Vertex) -> Self {
        EdgeIter {
            graph,
            index: vertex.index,
            edges: vertex.edges.iter(),
        }
    }
}

impl<'a> Iterator for EdgeIter<'a> {
    type Item = Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let edge = self.edges.next()?;
            let vertex: Vertex = self.graph.inlined_probe_value(edge.index);
            if vertex.index > self.index {
                return None;
            }
            if edge.count < 0 {
                continue;
            }
            return Some(vertex);
        }
    }
}
