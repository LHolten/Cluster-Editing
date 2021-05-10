use std::{ops, slice};

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
            for edge in &mut vertex.edges {
                if edge.version > self.versions.len() as u32 {
                    edge.version = u32::MAX
                }
            }
        }
    }

    pub fn cut(&mut self, v1: u32, v2: u32) -> u32 {
        let v1 = self.find(v1);
        let v2 = self.find(v2);
        let mut weight = 0;
        let mut cost = 0;
        for edge in &mut self.vertices[v1 as usize].edges {
            if self.find(edge.to) == v2 {
                edge.version = self.versions.len() as u32;
                if weight * edge.weight < 0 {
                    cost += edge.weight.abs() as u32
                }
                weight += edge.weight;
            }
        }
        for edge in &mut self.vertices[v2 as usize].edges {
            if self.find(edge.to) == v1 {
                edge.version = self.versions.len() as u32;
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

    pub fn find(&self, index: u32) -> u32 {
        if let Some(new_index) = self.vertices[index as usize].merged {
            self.find(new_index)
        } else {
            index
        }
    }

    pub fn connect(&mut self, v1: u32, v2: u32) {
        self.vertices[self.find(v1) as usize].merged = Some(v2)
    }
}

impl Index<u32> for Graph {
    type Output = Vertex;

    fn index(&self, index: u32) -> &Self::Output {
        &self.vertices[self.find(index) as usize]
    }
}

impl IndexMut<u32> for Graph {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        let index = self.find(index);
        if &index >= self.versions.last().unwrap() {
            &mut self.vertices[index as usize]
        } else {
            self.vertices.push(self.vertices[index as usize].clone());
            self.vertices.last_mut().unwrap()
        }
    }
}

pub struct ClusterIter<'a> {
    graph: &'a Graph,
    range: ops::Range<u32>,
}

// can maybe be reverted
impl<'a> Iterator for ClusterIter<'a> {
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.range.next()?;
            let vertex = &self.graph[index];
            if vertex.merged.is_none() {
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

// impl<'a> EdgeIter<'a> {
//     pub fn new(graph: &'a Cell<Graph>, vertex: &'a Vertex) -> Self {
//         EdgeIter {
//             graph,
//             index: vertex.index,
//             edges: vertex.edges.iter(),
//         }
//     }
// }

// impl<'a> Iterator for EdgeIter<'a> {
//     type Item = Vertex;

//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//             let edge = self.edges.next()?;
//             let mut graph: Graph = self.graph.take();
//             let vertex: Vertex = graph.inlined_probe_value(edge.index);
//             self.graph.set(graph);
//             if vertex.index > self.index {
//                 return None;
//             }
//             if edge.count < 0 {
//                 continue;
//             }
//             return Some(vertex);
//         }
//     }
// }
