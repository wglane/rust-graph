use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct VertexIden(usize);

impl VertexIden {
    fn from(i: usize) -> VertexIden {
        VertexIden { 0: i }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct EdgeIden(usize);

#[derive(Debug, Default)]
pub struct Graph<'a, V, E> {
    vid_to_v: HashMap<VertexIden, V>,
    v_to_vid: HashMap<&'a V, VertexIden>,
    eid_to_e: HashMap<EdgeIden, E>,
    e_to_eid: HashMap<&'a E, EdgeIden>,

    core: HashMap<V, E>,
}

impl<'a, V, E> Graph<'a, V, E> {
    pub fn new() -> Graph<'a, V, E> {
        Graph {
            vid_to_v: HashMap::new(),
            v_to_vid: HashMap::new(),
            eid_to_e: HashMap::new(),
            e_to_eid: HashMap::new(),
            core: HashMap::new(),
        }
    }

    pub fn has_vertex(&self, vertex_label: usize) -> bool {
        let id = VertexIden::from(vertex_label);
        self.vid_to_v.contains_key(&id)
    }
}

#[test]
fn test_new() {
    let g: Graph<i32, String> = Graph::new();
    assert!(g.vid_to_v.is_empty());
}

#[test]
fn test_has_vertex() {
    let g: Graph<String, usize> = Graph::new();
    assert_eq!(g.has_vertex(2), false);
}

fn main() {
    println!("Hello, world!");
}
