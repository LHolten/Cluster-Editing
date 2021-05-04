use ena::unify::{NoError, UnifyKey, UnifyValue};

use crate::merge::{AddEdges, MergeEdges};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VertexKey(u32);

impl UnifyKey for VertexKey {
    type Value = Vertex;

    fn index(&self) -> u32 {
        self.0
    }

    fn from_index(u: u32) -> Self {
        Self(u)
    }

    fn tag() -> &'static str {
        "VertexKey"
    }
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub size: u32,
    pub edges: Vec<Edge>,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
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

#[derive(Clone, Debug)]
pub struct Edge {
    pub number: i32,
    pub index: u32,
}
