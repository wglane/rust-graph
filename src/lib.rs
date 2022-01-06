use std::collections::{HashMap, HashSet};

type NodeLabel = String;
struct Node<N> {
    data: N,
}

type EdgeId = (NodeLabel, NodeLabel);
struct Edge<'a, N, E> {
    source: &'a Node<N>,
    dest: &'a Node<N>,
    data: E,
}

struct Graph<'a, N, E> {
    nid_to_n: HashMap<NodeLabel, Node<N>>,
    eid_to_e: HashMap<EdgeId, Edge<'a, N, E>>,
    neighbors: HashMap<NodeLabel, HashSet<NodeLabel>>,
}

impl<'a, N, E> Graph<'a, N, E> {
    pub fn new() -> Self {
        Graph {
            nid_to_n: HashMap::new(),
            eid_to_e: HashMap::new(),
            neighbors: HashMap::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.neighbors.len()
    }

    pub fn add_node(&mut self, label: &str, data: N) {
        let node = Node { data };
        self.nid_to_n.insert(label.to_string(), node);
        self.neighbors.insert(label.to_string(), HashSet::new());
    }

    pub fn has_node(&self, label: &NodeLabel) -> bool {
        self.neighbors.contains_key(label)
    }

    pub fn add_edge(&self, source: &str, dest: &str, data: E) -> Option<EdgeId> {
        if !self.has_node(source) || !self.has_node(dest) {
            return None;
        }
        Some((source.to_string(), dest.to_string()))
    }
}

#[test]
fn test_add_node() {
    struct Person {
        name: String,
        age: u8,
    }
    let mut g: Graph<Person, ()> = Graph::new();
    assert_eq!(0, g.size());

    let bob = Person {
        name: "Bob".to_string(),
        age: 37,
    };

    g.add_node("Bob", bob);
    assert_eq!(1, g.size());

    let sally = Person {
        name: "Sally".to_string(),
        age: 23,
    };
    g.add_node("Sally", sally);
    assert_eq!(2, g.size());
}
