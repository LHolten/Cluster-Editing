use partitions::PartitionVec;

pub type Vertex = Vec<u32>;

pub type Graph = Vec<Vertex>;

#[derive(Debug, Clone, Copy)]
pub struct WeightedEdge {
    pub index: u32,
    pub count: i32,
}

pub struct WeightedVertex {
    pub size: u32,
    pub edges: Vec<WeightedEdge>,
}

pub type Solution = PartitionVec<WeightedVertex>;
