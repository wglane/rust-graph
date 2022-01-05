use std::collections::{HashMap, HashSet};
use std::hash::Hash;

struct Vertex<V> {
    id: usize,
    data: V,
}

struct Edge<E> {
    id: usize,
    data: E,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct VertexIden(usize);

impl VertexIden {
    fn from(i: usize) -> VertexIden {
        VertexIden { 0: i }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct EdgeIden(usize);

#[derive(Debug)]
pub struct Graph<V, E>
where
    V: Eq + Hash + Clone,
    E: Eq + Hash + Clone,
{
    vid_to_v: HashMap<VertexIden, V>,
    v_to_vid: HashMap<V, VertexIden>,
    eid_to_e: HashMap<EdgeIden, E>,
    e_to_eid: HashMap<E, EdgeIden>,
    next_vid: usize,
    next_eid: usize,

    core: HashMap<VertexIden, HashSet<EdgeIden>>,
}

impl<V, E> Graph<V, E>
where
    V: Eq + Hash + Clone,
    E: Eq + Hash + Clone,
{
    pub fn new() -> Graph<V, E> {
        Graph {
            vid_to_v: HashMap::new(),
            v_to_vid: HashMap::new(),
            eid_to_e: HashMap::new(),
            e_to_eid: HashMap::new(),
            next_vid: 0,
            next_eid: 0,
            core: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, v: V) {
        match self.v_to_vid.get(&v) {
            None => {
                let vid = VertexIden::from(self.next_vid);
                self.next_vid += 1;
                let v_cloned = v.clone();

                self.vid_to_v.insert(vid, v);
                self.v_to_vid.insert(v_cloned, vid);
            }
            Some(&v) => {
                // vertex already in graph: remove all edges
                self.core.insert(v, HashSet::new());
            }
        }
    }

    pub fn has_vertex(&self, v: &V) -> bool {
        self.v_to_vid.contains_key(v)
    }

    fn has_vertex_iden(&self, vid: usize) -> bool {
        let vid = VertexIden::from(vid);
        self.vid_to_v.contains_key(&vid)
    }
}

#[test]
fn test_new() {
    let g: Graph<i32, String> = Graph::new();
    assert!(g.core.is_empty());
}

#[test]
fn test_has_vertex() {
    #[derive(PartialEq, Eq, Hash, Clone)]
    struct Person {
        name: String,
        age: u8,
    }
    let g: Graph<Person, bool> = Graph::new();
    let p = Person {
        name: String::from("William"),
        age: 35,
    };
    assert_eq!(false, g.has_vertex(&p));
}

#[test]
fn test_hash_map() {
    let mut m1: HashMap<String, String> = HashMap::new();
    let mut m2: HashMap<&str, &str> = HashMap::new();
    m1.insert("banana".to_string(), "split".to_string());
    let k = "banana".to_string();
    let v = m1.get(&k).unwrap();
    m2.insert(&k, v);
}

#[test]
fn test_has_vertex_iden() {
    let g: Graph<String, usize> = Graph::new();
    assert_eq!(g.has_vertex_iden(2), false);
}

fn main() {
    println!("Hello, world!");
}
