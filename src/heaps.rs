use crate::models::{Edge, Length, Vertex};
use std::collections::{BTreeSet, HashMap};


#[derive(Debug, Default)]
pub struct Entry {
    b: Length,
    u_set: Vec<Vertex>,
}

impl Entry{
    pub fn new(b: Length, u_set: Vec<Vertex>) -> Self {
        Self { b, u_set }
    }
    pub fn b(&self) -> Length {
        self.b
    }
    pub fn u_set(&self) -> &[Vertex] {
        &self.u_set
    }
}


#[derive(Debug, Default)]
pub struct Heap {
    que: BTreeSet<Edge>,
    d: HashMap<Vertex, Length>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            que: BTreeSet::new(),
            d: HashMap::new(),
        }
    }

    //TODO: we probably want to use edges always
    pub fn push(&mut self, v: Vertex, l: Length) {
        if let Some(&existing_dist) = self.d.get(&v) {
            if existing_dist < l {
                return;
            } else if existing_dist >= l {
                self.que.remove(&Edge::new(v, existing_dist));
            }
        }
        self.que.insert(Edge::new(v, l));
        self.d.insert(v, l);
    }

    pub fn top(&self) -> Option<&Edge> {
        self.que.first()
    }

    pub fn pop(&mut self) -> Option<Edge> {
        self.que.pop_first().map(|edge| {
            self.d.remove(edge.vertex());
            edge
        })
    }

    pub fn is_empty(&self) -> bool {
        self.que.is_empty()
    }
}

pub struct BlockHeap {
    m: usize,
    b: Length,
    que: BTreeSet<Edge>,
    d: HashMap<Vertex, Length>,
}

impl BlockHeap {
    pub fn new(m: usize, b: Length) -> Self {
        Self {
            m,
            b,
            que: BTreeSet::new(),
            d: HashMap::new(),
        }
    }

    pub fn insert(&mut self, v: Vertex, l: Length) {
        if let Some(&existing_dist) = self.d.get(&v) {
            if existing_dist < l {
                return;
            } else if existing_dist >= l {
                self.que.remove(&Edge::new(v, existing_dist));
            }
        }
        self.que.insert(Edge::new(v, l));
        self.d.insert(v, l);
    }

    pub fn batch_prepend(&mut self, l: &[(Vertex, Length)]) {
        for &(vertex, length) in l {
            self.insert(vertex, length);
        }
    }

    pub fn pull(&mut self) -> Entry {
        let mut s = Vec::new();

        for _ in 0..self.m {
            if let Some(edge) = self.que.pop_first() {
                self.d.remove(edge.vertex());
                s.push(*edge.vertex());
            } else {
                break;
            }
        }

        let b = self.que.first().map_or(self.b, |edge| *edge.length());

        Entry::new(b, s)
    }

    pub fn is_empty(&self) -> bool {
        self.que.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_basic_operations() {
        let mut heap = Heap::new();
        assert!(heap.is_empty());

        heap.push(1, 10.0);
        heap.push(2, 5.0);
        heap.push(3, 15.0);

        assert!(!heap.is_empty());
        assert_eq!(heap.top(), Some(&Edge::new(2, 5.0)));

        let edge = heap.pop().unwrap();
        assert_eq!(edge, Edge::new(2, 5.0));
        assert_eq!(heap.top(), Some(&Edge::new(1, 10.0)));
    }

    #[test]
    fn test_heap_update_existing() {
        let mut heap = Heap::new();
        heap.push(1, 10.0);
        heap.push(1, 5.0); // Should update to better distance
        assert_eq!(heap.top(), Some(&Edge::new(1, 5.0)));

        heap.push(1, 15.0); // Should not update (worse distance)
        assert_eq!(heap.top(), Some(&Edge::new(1, 5.0)));
    }

    #[test]
    fn test_block_heap_pull() {
        let mut block_heap = BlockHeap::new(2, 100.0);
        block_heap.insert(1, 10.0);
        block_heap.insert(2, 5.0);
        block_heap.insert(3, 15.0);

        let entry = block_heap.pull();
        assert_eq!(entry.b, 10.0);
    }
}
