use std::collections::HashSet;

use crate::{
    heaps::{BlockHeap, Entry, Heap},
    models::{Graph, Length, Vertex},
};

struct Pivots {
    p: Vec<Vertex>,
    w: Vec<Vertex>,
}
impl Pivots {
    pub fn new(p: Vec<Vertex>, w: Vec<Vertex>) -> Self {
        Self { p, w }
    }
}

#[derive(Debug, Default)]
pub struct ShortestPath {
    //G_
    graph: Graph,
    t: usize,
    k: usize,
    dhat: Vec<Length>,

    // find pivots attributes
    prev: Vec<Option<Vertex>>,
    tree_size: Vec<Option<usize>>,
    f: Vec<Vec<Vertex>>,
}

impl ShortestPath {
    pub fn new(graph: impl Into<Graph>) -> Self {
        Self {
            graph: graph.into(),
            ..Default::default()
        }
    }
    pub fn get(&mut self, s: Vertex) -> Vec<Length> {
        let n = self.graph.len();

        // NOTE: initialize to avoid error out of bounds
        self.dhat = vec![Length::INFINITY; n];
        self.dhat[s] = 0.0;
        self.prev = vec![None; n];
        self.tree_size = vec![None; n];
        self.f = vec![Vec::new(); n];

        let n = n as f64;
        let t = (n.log2().powf(2.0 / 3.0)).floor();
        self.k = (n.log2().powf(1.0 / 3.0)).ceil() as usize;
        self.t = t as usize;
        let l = (n.log2() / t).ceil() as usize;

        let mut source_set = Vec::new();
        source_set.push(s);
        self.bmssp(l, Length::INFINITY, &source_set);

        self.dhat.clone()
    }

    fn bmssp(&mut self, l: usize, b: Length, s: &[Vertex]) -> Entry {
        if l == 0 {
            return self.base_case(b, s);
        }

        let pivots = self.find_pivots(b, s);

        let m = 2_usize.pow(((l - 1) * self.t) as u32);
        let mut d = BlockHeap::new(m, b);
        let mut bd = Length::INFINITY;

        for &u in &pivots.p {
            d.insert(u, self.dhat[u]);
            bd = bd.min(self.dhat[u]);
        }

        let mut u_set = HashSet::new();

        while u_set.len() < self.k * 2_usize.pow((l * self.t) as u32) && !d.is_empty() {
            let entry = d.pull();
            let b_entry = self.bmssp(l - 1, entry.b(), entry.u_set());

            for &u in b_entry.u_set() {
                u_set.insert(u);
            }

            let mut k_vec = Vec::new();
            for &u in b_entry.u_set() {
                for edge in &self.graph[u] {
                    let v = *edge.vertex();
                    let w = *edge.length();

                    if self.dhat[v] >= self.dhat[u] + w {
                        let new_dist = self.dhat[u] + w;
                        self.dhat[v] = new_dist;
                        if entry.b() <= new_dist && new_dist < b {
                            d.insert(v, new_dist);
                        } else if b_entry.b() <= new_dist && new_dist < entry.b() {
                            k_vec.push((v, new_dist));
                        }
                    }
                }
            }

            for &u in entry.u_set() {
                if b_entry.b() <= self.dhat[u] && self.dhat[u] < entry.b() {
                    k_vec.push((u, self.dhat[u]));
                }
            }

            d.batch_prepend(&k_vec);
            bd = b_entry.b();
        }

        bd = bd.min(b);
        for &u in &pivots.w {
            if self.dhat[u] < bd {
                u_set.insert(u);
            }
        }

        Entry::new(bd, u_set.into_iter().collect())
    }

    fn base_case(&mut self, b: Length, s: &[Vertex]) -> Entry {
        assert_eq!(s.len(), 1);

        let x = s[0];
        let mut u0 = HashSet::new();
        u0.extend(s.iter());

        let mut h = Heap::new();
        h.push(x, self.dhat[x]);

        while let Some(edge) = h.pop()
            && u0.len() < self.k + 1
        {
            let u = *edge.vertex();

            u0.insert(u);

            for graph_edge in &self.graph[u] {
                let v = *graph_edge.vertex();
                let w = *graph_edge.length();

                if self.dhat[v] >= self.dhat[u] + w && self.dhat[u] + w < b {
                    self.dhat[v] = self.dhat[u] + w;
                    h.push(v, self.dhat[v]);
                }
            }
        }

        if u0.len() <= self.k {
            Entry::new(b, u0.into_iter().collect())
        } else {
            let mut bd = Length::MIN;
            for &u in &u0 {
                bd = bd.max(self.dhat[u]);
            }

            //TODO: use a filter map an so on
            let mut u_vec = Vec::new();
            for &u in &u0 {
                if self.dhat[u] < bd {
                    u_vec.push(u);
                }
            }
            Entry::new(bd, u_vec)
        }
    }

    fn find_pivots(&mut self, b: Length, s: &[Vertex]) -> Pivots {
        let mut w = HashSet::new();
        let mut wp = HashSet::new();

        w.extend(s.iter().copied());
        wp.extend(s.iter().copied());

        for &v in &w {
            self.prev[v] = None;
        }

        for _ in 0..self.k {
            let mut wi = HashSet::new();
            for &u in &wp {
                for graph_edge in &self.graph[u] {
                    let v = *graph_edge.vertex();
                    let w = *graph_edge.length();

                    if self.dhat[v] >= self.dhat[u] + w && self.dhat[u] + w < b{
                        self.dhat[v] = self.dhat[u] + w;
                        self.prev[v] = Some(u);
                        wi.insert(v);
                    }
                }
            }

            w.extend(&wi);

            if w.len() >= self.k * s.len() {
                return Pivots::new(s.to_vec(), w.into_iter().collect());
            }
            wp = wi;
        }

        for &v in &w {
            self.tree_size[v] = None;
            self.f[v].clear();
        }

        for &v in &w {
            if let Some(u) = self.prev[v] {
                self.f[u].push(v);
            }
        }

        let mut p = Vec::new();
        for &u in s {
            if find_tree_size(u, &mut self.tree_size, &self.f) >= self.k && self.prev[u].is_none() {
                p.push(u);
            }
        }

        Pivots::new(p, w.into_iter().collect())
    }
}

fn find_tree_size(u: usize, tree_size: &mut [Option<usize>], f: &[Vec<Vertex>]) -> usize {
    if let Some(v) = tree_size[u] {
        return v;
    }

    let mut res = 1;
    for &v in &f[u] {
        res += find_tree_size(v, tree_size, f);
    }

    tree_size[u] = Some(res);
    res
}
