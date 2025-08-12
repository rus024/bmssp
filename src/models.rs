use std::{cmp::Ordering, ops::Deref};

pub type Vertex = usize;

pub type Length = f32;

//NOTE: clone can be avoided
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Edge {
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
        self.length
            .partial_cmp(&other.length)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Graph(Vec<Vec<Edge>>);

impl Deref for Graph {
    type Target = Vec<Vec<Edge>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Vec<Edge>>> for Graph {
    fn from(graph: Vec<Vec<Edge>>) -> Self {
        Self(graph)
    }
}
