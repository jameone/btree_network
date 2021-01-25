use alloc::collections::BTreeSet;

/// `Vertices` returns the set of the vertices which comprise the network.
///
/// # Example
///
/// ```
/// use btree_network::{BTreeNetwork, AddVertex, Vertices};
/// let mut network: BTreeNetwork<String> = BTreeNetwork::new();
///
/// assert_eq!(network.vertices().len(), 0);
/// ```
pub trait Vertices<T>
where
    T: Ord,
{
    fn vertices(&self) -> BTreeSet<&T>;
}

/// `AddVertex` adds the vertex x, if it is not there.
///
/// # Example
///
/// ```
/// use btree_network::{BTreeNetwork, AddVertex, Vertices};
/// let mut network: BTreeNetwork<String> = BTreeNetwork::new();
/// network.add_vertex(String::from("origin"));
///
/// assert_eq!(network.vertices().len(), 1);
/// ```
pub trait AddVertex<T>
where
    T: Ord,
{
    fn add_vertex(&mut self, x: T) -> Option<BTreeSet<T>>;
}

/// `AddEdge` add an edge from the vertex x to the vertex y, if it is not there.
///
/// # Example
///
/// ```
/// extern crate alloc;
/// use alloc::collections::btree_set::BTreeSet;
/// use btree_network::{BTreeNetwork, AddVertex, AddEdge, GetVertexValue};
/// let mut network: BTreeNetwork<String> = BTreeNetwork::new();
/// network.add_vertex(String::from("origin"));
/// network.add_vertex(String::from("destination"));
/// network.add_edge(String::from("origin"), String::from("destination"));
///
/// let x_value: &BTreeSet<String> = network.get_vertex_value(String::from("origin")).unwrap();
/// assert!(x_value.contains(&String::from("destination")));
///
/// let y_value: &BTreeSet<String> = network.get_vertex_value(String::from("destination")).unwrap();
/// assert!(y_value.contains(&String::from("origin")));
/// ```
pub trait AddEdge<T> {
    type Error;
    fn add_edge(&mut self, x: T, y: T) -> Result<(), Self::Error>;
}

/// `GetVertexValue` returns the value associated with the vertex x.
///
/// # Example
///
/// ```
/// extern crate alloc;
/// use alloc::collections::btree_set::BTreeSet;
/// use btree_network::{BTreeNetwork, AddVertex, AddEdge, GetVertexValue};
/// let mut network: BTreeNetwork<String> = BTreeNetwork::new();
/// network.add_vertex(String::from("origin"));
/// network.add_vertex(String::from("destination"));
/// network.add_edge(String::from("origin"), String::from("destination"));
///
/// let vertex_value: &BTreeSet<String> = network.get_vertex_value(String::from("origin")).unwrap();
/// assert!(vertex_value.contains(&String::from("destination")));
/// ```
pub trait GetVertexValue<T>
where
    T: Ord,
{
    fn get_vertex_value(&self, x: T) -> Option<&BTreeSet<T>>;
}

/// `RemoveEdge` removes the edge from the vertex x to the vertex y, if it is there.
///
/// # Example
///
/// ```
/// use btree_network::{BTreeNetwork, AddVertex, AddEdge, RemoveEdge, GetVertexValue};
/// let mut network: BTreeNetwork<String> = BTreeNetwork::new();
/// network.add_vertex(String::from("origin"));
/// network.add_vertex(String::from("destination"));
/// network.add_edge(String::from("origin"), String::from("destination"));
///
///
/// network.remove_edge(String::from("origin"), String::from("destination"));
///
/// // Note: deletion of edges cascade i.e. the edge is also deleted from any incident
/// // vertices' adjacency lists.
/// assert_eq!(network.get_vertex_value(String::from("origin")).unwrap().len(), 0);
/// assert_eq!(network.get_vertex_value(String::from("destination")).unwrap().len(), 0);
/// ```
pub trait RemoveEdge<T> {
    type Error;
    fn remove_edge(&mut self, x: T, y: T) -> Result<(), Self::Error>;
}

/// `RemoveVertex` removes the vertex x, if it is there.
///
/// # Example
///
/// ```
/// extern crate alloc;
/// use alloc::collections::btree_set::BTreeSet;
/// use btree_network::{BTreeNetwork, AddVertex, AddEdge, RemoveVertex, GetVertexValue, Vertices};
/// let mut network: BTreeNetwork<String> = BTreeNetwork::new();
/// network.add_vertex(String::from("origin"));
/// network.add_vertex(String::from("destination"));
/// network.add_edge(String::from("origin"), String::from("destination"));
///
///
/// network.remove_vertex(String::from("destination"));
/// assert_eq!(network.vertices().len(), 1);
///
/// // Note: removing a vertex will also cascade delete any incident edges, which will then
/// // cascade delete any edges from the origin existing vertices' adjacency list.
/// assert_eq!(network.get_vertex_value(String::from("origin")).unwrap().len(), 0);
/// ```
pub trait RemoveVertex<T>
where
    T: Ord,
{
    type Error;
    fn remove_vertex(&mut self, x: T) -> Result<(), Self::Error>;
}

/// `Adjacent` tests whether there is an edge from the vertex x to the vertex y.
/// An error is thrown if either x, or y do not exist. By definition of adjacent there
/// must exist an edge e, with value (x, y) in order for vertices x, and y to be
/// considered adjacent.
///
/// # Example
///
/// ```
/// use btree_network::{BTreeNetwork, AddVertex, AddEdge, Adjacent};
/// let mut network: BTreeNetwork<String> = BTreeNetwork::new();
/// network.add_vertex(String::from("origin"));
/// network.add_vertex(String::from("destination"));
/// network.add_edge(String::from("origin"), String::from("destination"));
///
/// assert!(network.adjacent(String::from("origin"), String::from("destination")).unwrap());
/// // Note: the network is undirected, and the definition of adjacent
/// // can be phrased, if there exists a relationship from x to y. Therefore
/// // A and B adjacent implies B and A are adjacent.
/// assert!(network.adjacent(String::from("destination"), String::from("origin")).unwrap());
/// ```
pub trait Adjacent<T> {
    type Error;
    fn adjacent(&self, x: T, y: T) -> Result<bool, Self::Error>;
}

/// `Connections` lists all vertices y such that there is an edge from the vertex x to
/// the vertex y. An error is thrown if x does not exist.
///
/// # Example
///
/// ```
/// use btree_network::{BTreeNetwork, AddVertex, AddEdge, Connections};
/// let mut network: BTreeNetwork<String> = BTreeNetwork::new();
/// network.add_vertex(String::from("origin"));
/// network.add_vertex(String::from("destination"));
/// network.add_edge(String::from("origin"), String::from("destination"));
///
/// assert!(network.connections(String::from("origin")).unwrap().contains(&String::from("destination")));
/// ```
pub trait Connections<T> {
    fn connections(&self, x: T) -> Option<&BTreeSet<T>>;
}
