use std::cmp::Ordering;


pub type Vertex = usize;

pub type Length = f32;

#[derive(Debug, Default, PartialEq)]
pub struct Edge{
    vertex: Vertex,
    length: Length,
}

impl Edge {
    pub fn new(vertex: Vertex, length: Length) -> Self {
        Self { vertex, length }
    }
    pub fn vertex(&self) -> &Vertex {
        &self.vertex
    }
    pub fn length(&self) -> &Length {
        &self.length
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length.partial_cmp(&other.length)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub type Graph = Vec<Vec<Edge>>;
