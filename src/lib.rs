#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeIndex(usize);

impl NodeIndex {
    fn from(i: usize) -> NodeIndex {
        NodeIndex { 0: i }
    }
}

#[derive(Debug)]
pub struct Node<N> {
    data: N,
    edges: Vec<EdgeIndex>,
}

impl<N> Node<N> {
    fn from(data: N) -> Node<N> {
        Node { data, edges: vec![] }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgeIndex(usize);

impl EdgeIndex {
    pub fn from(i: usize) -> EdgeIndex {
        EdgeIndex { 0: i }
    }
}

#[derive(Debug)]
pub struct Edge<E> {
    // source is implicit: stored with Node
    dest: NodeIndex,
    data: E,
}

impl<E> Edge<E> {
    fn from(dest: NodeIndex, data: E) -> Edge<E> {
        Edge { dest, data }
    }
}

#[derive(Debug)]
pub struct Graph<N, E> {
    nodes: Vec<Option<Node<N>>>,
    edges: Vec<Option<Edge<E>>>,
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Graph<N, E> {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, data: N) -> NodeIndex {
        let ind = NodeIndex::from(self.nodes.len());
        self.nodes.push(Some(Node::from(data)));
        ind
    }

    pub fn add_edge(&mut self, source: NodeIndex, dest: NodeIndex, data: E) -> Option<EdgeIndex> {
        let has_source_node = self.has_node(source);
        let has_dest_node = self.has_node(dest);

        match (has_source_node, has_dest_node) {
            (false, _) => None,
            (_, false) => None,
            (true, _) => {
                let edge = Edge::from(dest, data);
                let ind = self.push_edge(edge);
                self.get_node_from_index_mut(source).unwrap().edges.push(ind);
                Some(ind)
            }
        }
    }

    pub fn get_node_from_index(&self, ind: NodeIndex) -> Option<&Node<N>> {
        match self.nodes.get(ind.0) {
            None => None,
            Some(None) => None,
            Some(Some(n)) => Some(n),
        }
    }

    pub fn get_node_from_index_mut(&mut self, ind: NodeIndex) -> Option<&mut Node<N>> {
        match self.nodes.get_mut(ind.0) {
            None => None,
            Some(None) => None,
            Some(Some(n)) => Some(n),
        }
    }

    pub fn get_edge_from_index(&self, ind: EdgeIndex) -> Option<&Edge<E>> {
        match self.edges.get(ind.0) {
            None => None,
            Some(None) => None,
            Some(Some(e)) => Some(e),
        }
    }

    pub fn get_edge_from_index_mut(&mut self, ind:EdgeIndex) -> Option<&mut Edge<E>> {
        match self.edges.get_mut(ind.0) {
            None => None,
            Some(None) => None,
            Some(Some(e)) => Some(e)
        }
    }

    pub fn get_edge_index_between_nodes(&self, source: NodeIndex, dest: NodeIndex) -> Option<EdgeIndex> {
        let source_node = self.get_node_from_index(source);
        let dest_node = self.get_node_from_index(dest);
        match (source_node, dest_node) {
            (None, _) => None,
            (_, None) => None,
            (_, _) => {
                let source_node = source_node.unwrap();
                for edge_ind in source_node.edges.iter() {
                    let edge = self.edges[edge_ind.0].as_ref();
                    if let Some(e) = edge {
                        if e.dest == dest {
                            return Some(*edge_ind);
                        }
                    }

                }
                None
            },
        }
    }

    pub fn get_edge_between_nodes(&self, source: NodeIndex, dest: NodeIndex) -> Option<&Edge<E>> {
        if let Some(ind) = self.get_edge_index_between_nodes(source, dest) {
            self.get_edge_from_index(ind)
        } else {
            None
        }
    }

    pub fn get_edge_between_nodes_mut(&mut self, source: NodeIndex, dest: NodeIndex) -> Option<&mut Edge<E>> {
        if let Some(ind) = self.get_edge_index_between_nodes(source, dest) {
            self.get_edge_from_index_mut(ind)
        } else {
            None
        }
    }

    pub fn delete_node(&mut self, ind: NodeIndex) {
        if !self.has_node(ind) {
            return
        }
        // delete all edges that point to that node
        for edge in self.edges.iter_mut() {
            if let Some(e) = edge {
                if e.dest == ind {
                    *edge = None
                }
            }
        }
        // delete all edges from that node
        // avoid multiple borrows by cloning indices into vector
        let to_delete = self.get_node_from_index(ind).unwrap().edges.clone();
        for edge_ind in to_delete {
            self.delete_edge(edge_ind);
        }
        // delete node itself
        self.nodes[ind.0] = None;
    }

    pub fn delete_edge(&mut self, ind: EdgeIndex) {
        self.edges[ind.0] = None;
    }

    pub fn delete_edge_between_nodes(&mut self, source: NodeIndex, dest: NodeIndex) {
        if let Some(ind) = self.get_edge_index_between_nodes(source, dest) {
            self.delete_edge(ind);
        }
    }

    pub fn has_node(&self, ind: NodeIndex) -> bool {
        self.get_node_from_index(ind).is_some()
    }

    pub fn has_edge(&self, ind: EdgeIndex) -> bool {
        self.get_edge_from_index(ind).is_some()
    }

    pub fn size(&self) -> (usize, usize) {
        // counts entries that are Some(_)
        (self.nodes.iter().flatten().count(), self.edges.iter().flatten().count())
    }

    pub fn merge_nodes(&mut self, from: NodeIndex, into: NodeIndex) {
        unimplemented!()
    }

    fn push_edge(&mut self, e: Edge<E>) -> EdgeIndex {
        let ind = EdgeIndex { 0: self.edges.len() };
        self.edges.push(Some(e));
        ind
    }
    
}

#[test]
fn test_graph() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    type HasFriend = bool;
    type FriendGraph = Graph<Person, HasFriend>;

    // test empty graph
    let mut g: FriendGraph = Graph::new();
    assert_eq!((0, 0), g.size());
    assert!(!g.has_node(NodeIndex::from(0)));
    assert!(!g.has_edge(EdgeIndex::from(0)));

    // test add bogus edge to empty graph 
    assert!(g.add_edge(NodeIndex::from(2), NodeIndex::from(5), true).is_none());
    assert_eq!((0, 0), g.size());

    let bob = Person {
        name: "bob".to_string(),
        age: 37,
    };

    // test add node
    g.add_node(bob);
    assert_eq!((1, 0), g.size());

    let sally = Person {
        name: "sally".to_string(),
        age: 24,
    };

    // test add second node
    g.add_node(sally);
    assert_eq!((2, 0), g.size());

    // test add edges
    g.add_edge(NodeIndex::from(0), NodeIndex::from(1), true);
    g.add_edge(NodeIndex::from(1), NodeIndex::from(0), true);
    assert_eq!((2, 2), g.size());

    // test edges between nodes
    fn are_friends(g: &FriendGraph, a: NodeIndex, b: NodeIndex) -> bool {
        g.get_edge_between_nodes(a, b).is_some()
    }
    assert!(are_friends(&g, NodeIndex::from(0), NodeIndex::from(1)));
    assert!(are_friends(&g, NodeIndex::from(1), NodeIndex::from(0)));
    
    let zach = Person {
        name: "zach".to_string(),
        age: 33
    };

    // test add more nodes and edges
    g.add_node(zach);
    g.add_edge(NodeIndex::from(0), NodeIndex::from(2), true);
    g.add_edge(NodeIndex::from(2), NodeIndex::from(0), true);
    assert_eq!((3, 4), g.size());
    assert!(are_friends(&g, NodeIndex::from(0), NodeIndex::from(2)));
    assert!(are_friends(&g, NodeIndex::from(2), NodeIndex::from(0)));

    // test delete node
    g.delete_node(NodeIndex::from(2));
    assert_eq!((2, 2), g.size());
    assert!(!g.has_node(NodeIndex::from(2)));

    g.delete_edge_between_nodes(NodeIndex::from(0), NodeIndex::from(1));
    g.delete_edge_between_nodes(NodeIndex::from(1), NodeIndex::from(0));
    assert_eq!((2, 0), g.size());
    assert!(!g.has_edge(EdgeIndex::from(2)));
    assert!(!g.has_edge(EdgeIndex::from(3)));

    g.delete_node(NodeIndex::from(1));
    assert_eq!((1, 0), g.size());

    // test double delete -- shouldn't change anything
    g.delete_node(NodeIndex::from(1));
    assert_eq!((1, 0), g.size());


    // To print graph:
    // println!("{:?}", g);
    // assert!(false);
}
