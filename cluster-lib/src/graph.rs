use std::ops;

use ena::unify::{NoError, PersistentUnificationTable, UnifyKey, UnifyValue};

use crate::merge::{AddEdges, MergeEdges};

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
    pub size: u32,
    pub edges: Vec<Edge>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            size: 1,
            edges: Vec::new(),
        }
    }
}

impl UnifyValue for Vertex {
    type Error = NoError;

    fn unify_values(value1: &Self, value2: &Self) -> Result<Self, Self::Error> {
        Ok(Vertex {
            size: value1.size + value2.size,
            edges: AddEdges(MergeEdges::new(&value1.edges, &value2.edges)).collect(), // need to filter out the inner edges at some point
        })
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
        VertexKey::from(u)
    }
}

pub type Graph = PersistentUnificationTable<VertexKey>;

pub struct IterSets<'a> {
    graph: &'a Graph,
    range: ops::Range<u32>,
}

impl<'a> IterSets<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        IterSets {
            graph,
            range: 0..graph.len() as u32,
        }
    }
}

impl<'a> Iterator for IterSets<'a> {
    type Item = VertexKey;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.range.next()?;
            let root: VertexKey = self.graph.find(index);
            if root == index.into() {
                return Some(root);
            }
        }
    }
}
