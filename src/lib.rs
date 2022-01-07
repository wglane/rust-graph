#[derive(Clone, Copy, Debug)]
struct NodeIndex(usize);

impl NodeIndex {
    fn from(i: usize) -> NodeIndex {
        NodeIndex { 0: i }
    }
}

#[derive(Debug)]
struct Node<N> {
    data: N,
    edges: Vec<EdgeIndex>,
}

impl<N> Node<N> {
    fn from(data: N) -> Node<N> {
        Node { data, edges: vec![] }
    }
}

#[derive(Clone, Copy, Debug)]
struct EdgeIndex(usize);

#[derive(Debug)]
struct Edge<E> {
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
struct Graph<N, E> {
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
        if !self.is_valid_node_index(source) || !self.is_valid_node_index(dest) {
            return None;
        }

        let source_node = self.nodes.get(source.0).unwrap();
        let dest_node = self.nodes.get(dest.0).unwrap();
        if source_node.is_none() || dest_node.is_none() {
            return None;
        }

        let edge = Edge::from(dest, data);
        Some(self.push_edge(edge))
    }

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
fn test_add_nodes() {
    struct Person {
        name: String,
        age: u8,
    }
    type are_friends = bool;

    let g: Graph<Person, are_friends> = Graph::new();
    assert_eq!((0, 0), g.size());

    let bob = Person {
        name: "bob".to_string(),
        age: 37,
    };
}
