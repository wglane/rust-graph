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
        let mut source_node = self.get_node(source);
        let dest_node = self.get_node(dest);

        match (source_node, dest_node) {
            (None, _) => None,
            (_, None) => None,
            (Some(n), Some(m)) => {
                let edge = Edge::from(dest, data);
                let ind = self.push_edge(edge);
                // TODO: add to target node's Edges
                Some(self.push_edge(edge))
            }
        }
    }

    pub fn get_node(&self, ind: NodeIndex) -> Option<&Node<N>> {
        match self.nodes.get(ind.0) {
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

    pub fn has_node(&self, ind: NodeIndex) -> bool {
        self.get_node(ind).is_some()
    }

    pub fn has_edge_from_ind(&self, ind: EdgeIndex) -> bool {
        self.get_edge_from_index(ind).is_some()
    }

    // pub fn get_edge(&self, source: NodeIndex, dest: NodeIndex) -> Option<&Edge<E>> {
    //     if !self.has_node(source) || !self.has_node(dest) {
    //         None
    //     } else {
    //         self.node
    //         None
    //     }
    // }

    // pub fn get_edge(&self) -> &Option<Edge<E>> {

    // }
    // pub fn get_node(&self) -> &Option<Node<V>> {}

    pub fn size(&self) -> (usize, usize) {
        (self.nodes.len(), self.edges.len())
    }

    fn push_edge(&mut self, e: Edge<E>) -> EdgeIndex {
        let ind = EdgeIndex { 0: self.edges.len() };
        self.edges.push(Some(e));
        ind
    }

    fn is_valid_node_index(&self, ind: NodeIndex) -> bool {
        ind.0 < self.size().0
    }

    fn is_valid_edge_index(&self, ind: EdgeIndex) -> bool {
        ind.0 < self.size().1
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

    let mut g: Graph<Person, HasFriend> = Graph::new();
    assert_eq!((0, 0), g.size());

    let bob = Person {
        name: "bob".to_string(),
        age: 37,
    };
    let mut nodes = vec![g.add_node(bob)];
    assert_eq!((1, 0), g.size());

    let sally = Person {
        name: "sally".to_string(),
        age: 24,
    };
    nodes.push(g.add_node(sally));
    assert_eq!((2, 0), g.size());

    let edges = vec![g.add_edge(nodes[0], nodes[1], true)];
    assert_eq!((2, 1), g.size());

    println!("{:?}", g);
    assert!(false);
}
