#![cfg(test)]

mod unit_tests {
    use crate::Error;
    use crate::network::*;
    use alloc::collections::{BTreeMap, BTreeSet};

    #[test]
    fn test_network() {
        let network: BTreeNetwork<usize> = BTreeNetwork::new();
        assert_eq!(network, BTreeNetwork::new())
    }

    #[test]
    fn definition() {
        // Instantiate a network using the new associated function.
        let network: BTreeNetwork<usize> = BTreeNetwork::new();
        let vertices: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

        // Check network struct is generated.
        assert_eq!(network, BTreeNetwork { vertices })

        // Test passed
    }

    #[test]
    fn new_and_default() {
        // Instantiate a network using the implementation of default.
        let network: BTreeNetwork<usize> = BTreeNetwork::new();

        // Check network struct is equivalent to the struct generated
        // with the new associated function.
        assert_eq!(network, BTreeNetwork::default())

        // Test passed
    }

    #[test]
    fn vertices() {
        // Ensure there is a getter method for the vertices.
        let network: BTreeNetwork<usize> = BTreeNetwork::new();
        let vertices: BTreeSet<&usize> = BTreeSet::new();
        assert_eq!(network.vertices(), vertices)

        // Test passed.
    }

    #[test]
    fn add_vertex() {
        // Add three nodes.
        let mut network: BTreeNetwork<usize> = BTreeNetwork::new();
        network.add_vertex(0);
        network.add_vertex(1);
        network.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(network.vertices().len(), 3);

        // Verify nodes retain order on read.
        let mut exp_vertices: BTreeSet<&usize> = BTreeSet::new();
        exp_vertices.insert(&0);
        exp_vertices.insert(&1);
        exp_vertices.insert(&2);
        assert_eq!(network.vertices(), exp_vertices)

        // Test passed.
    }

    #[test]
    fn add_edge() -> Result<(), Error> {
        // Add three nodes.
        let mut network: BTreeNetwork<usize> = BTreeNetwork::new();
        network.add_vertex(0);
        network.add_vertex(1);
        network.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(network.vertices().len(), 3);

        // Add an edge (0, 1) and (1, 2).
        network.add_edge(0, 1)?;
        network.add_edge(1, 2)?;

        // The vertex 0 is adjacent to vertex 1.
        let mut exp_edges_0: BTreeSet<usize> = BTreeSet::new();
        exp_edges_0.insert(1);
        assert_eq!(network.get_vertex_value(0).unwrap(), &exp_edges_0);

        // The vertex 1 is adjacent to vertex 2 and 0.
        let mut exp_edges_1: BTreeSet<usize> = BTreeSet::new();
        exp_edges_1.insert(0);
        exp_edges_1.insert(2);
        assert_eq!(network.get_vertex_value(1).unwrap(), &exp_edges_1);

        // If you attempt to add an edge to a vertex that does not
        // exist, then an error is raised.
        assert_eq!(
            network.add_edge(0, 3).unwrap_err(),
            Error::VertexDoesNotExist
        );
        assert_eq!(
            network.add_edge(3, 0).unwrap_err(),
            Error::VertexDoesNotExist
        );
        assert_eq!(
            network.add_edge(1, 3).unwrap_err(),
            Error::VertexDoesNotExist
        );
        assert_eq!(
            network.add_edge(3, 1).unwrap_err(),
            Error::VertexDoesNotExist
        );

        // Tests passed.
        Ok(())
    }

    #[test]
    fn remove_vertex() -> Result<(), Error> {
        // Add three nodes.
        let mut network: BTreeNetwork<usize> = BTreeNetwork::new();
        network.add_vertex(0);
        network.add_vertex(1);
        network.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(network.vertices().len(), 3);

        // Add an edge (0, 1) = 2 and (1, 2) = 3.
        network.add_edge(0, 1)?;
        network.add_edge(1, 2)?;

        // Remove the first node.
        network.remove_vertex(0)?;

        // Check there remain only two nodes.
        let mut exp_vertices: BTreeSet<&usize> = BTreeSet::new();
        exp_vertices.insert(&1);
        exp_vertices.insert(&2);
        assert_eq!(network.vertices(), exp_vertices);

        // Add three nodes.
        let mut network: BTreeNetwork<usize> = BTreeNetwork::new();
        network.add_vertex(0);
        network.add_vertex(1);
        network.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(network.vertices().len(), 3);

        // Add an edge (0, 1) = 2 and (1, 2) = 3.
        network.add_edge(0, 1)?;
        network.add_edge(1, 2)?;

        // Remove the first node.
        network.remove_vertex(1)?;

        // Check there remain only two nodes.
        let mut exp_vertices: BTreeSet<&usize> = BTreeSet::new();
        exp_vertices.insert(&0);
        exp_vertices.insert(&2);
        assert_eq!(network.vertices(), exp_vertices);

        let exp_edges_0: BTreeSet<usize> = BTreeSet::new();
        assert_eq!(network.get_vertex_value(0).unwrap(), &exp_edges_0);

        Ok(())

        // Test passed.
    }

    #[test]
    fn remove_edge() -> Result<(), Error> {
        // Add three nodes.
        let mut network: BTreeNetwork<usize> = BTreeNetwork::new();
        network.add_vertex(0);
        network.add_vertex(1);
        network.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(network.vertices().len(), 3);

        // Add an edge (0, 1) and (1, 2).
        network.add_edge(0, 1)?;
        network.add_edge(1, 2)?;

        // and that edge has a value (1, 2).
        let mut exp_edges: BTreeSet<usize> = BTreeSet::new();
        exp_edges.insert(1);
        assert_eq!(network.get_vertex_value(0).unwrap(), &exp_edges);
        assert_eq!(network.get_vertex_value(2).unwrap(), &exp_edges);

        let mut exp_edges: BTreeSet<usize> = BTreeSet::new();
        exp_edges.insert(0);
        exp_edges.insert(2);
        assert_eq!(network.get_vertex_value(1).unwrap(), &exp_edges);

        // Remove the first edge.
        network.remove_edge(0, 1)?;

        // Verify there are still three nodes.
        assert_eq!(network.vertices().len(), 3);

        // and that edge has a value (1, 2).
        let exp_edges: BTreeSet<usize> = BTreeSet::new();
        assert_eq!(network.get_vertex_value(0).unwrap(), &exp_edges);

        let mut exp_edges: BTreeSet<usize> = BTreeSet::new();
        exp_edges.insert(2);
        assert_eq!(network.get_vertex_value(1).unwrap(), &exp_edges);

        let mut exp_edges: BTreeSet<usize> = BTreeSet::new();
        exp_edges.insert(1);
        assert_eq!(network.get_vertex_value(2).unwrap(), &exp_edges);

        // Test passed.
        Ok(())
    }

    #[test]
    fn get_vertex_value() -> Result<(), Error> {
        // Add three nodes.
        let mut network: BTreeNetwork<usize> = BTreeNetwork::new();
        network.add_vertex(0);
        network.add_vertex(1);
        network.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(network.vertices().len(), 3);

        // Add an edge (0, 1) and (1, 2).
        network.add_edge(0, 1)?;
        network.add_edge(1, 2)?;

        let mut exp_edges_0: BTreeSet<usize> = BTreeSet::new();
        exp_edges_0.insert(1);
        assert_eq!(network.get_vertex_value(0).unwrap(), &exp_edges_0);

        let mut exp_edges_1: BTreeSet<usize> = BTreeSet::new();
        exp_edges_1.insert(0);
        exp_edges_1.insert(2);
        assert_eq!(network.get_vertex_value(1).unwrap(), &exp_edges_1);

        let mut exp_edges_2: BTreeSet<usize> = BTreeSet::new();
        exp_edges_2.insert(1);
        assert_eq!(network.get_vertex_value(2).unwrap(), &exp_edges_2);

        // Test passed.
        Ok(())
    }

    #[test]
    fn adjacent() -> Result<(), Error> {
        // Add three nodes.
        let mut network: BTreeNetwork<usize> = BTreeNetwork::new();
        network.add_vertex(0);
        network.add_vertex(1);
        network.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(network.vertices().len(), 3);

        // Add an edge (0, 1) and (1, 2).
        network.add_edge(0, 1)?;
        network.add_edge(1, 2)?;

        // By definition vertices 0, and 1 are adjacent.
        assert!(network.adjacent(0, 1)?);
        // By definition vertices 1, and 0 are adjacent.
        assert!(network.adjacent(1, 0)?);
        // By definition vertices 1, and 2 are adjacent.
        assert!(network.adjacent(1, 2)?);
        // By definition vertices 2, and 1 are adjacent.
        assert!(network.adjacent(2, 1)?);

        // If we attempt to check adjacency on a node that does not exist,
        // an error will be raised.
        assert_eq!(
            network.adjacent(0, 3).unwrap_err(),
            Error::VertexDoesNotExist
        );
        assert_eq!(
            network.adjacent(3, 0).unwrap_err(),
            Error::VertexDoesNotExist
        );

        // Test passed.
        Ok(())
    }

    #[test]
    fn connections() -> Result<(), Error> {
        // Add three nodes.
        let mut network: BTreeNetwork<usize> = BTreeNetwork::new();
        network.add_vertex(0);
        network.add_vertex(1);
        network.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(network.vertices().len(), 3);

        // Add an edge (0, 1), (1, 2), and (0, 2).
        network.add_edge(0, 1)?;
        network.add_edge(1, 2)?;
        network.add_edge(0, 2)?;

        // There should be, by definition, two nodes (1, and 2)
        // 'connected' to node 0 through edges 2, and 4;
        let mut exp_connections_0: BTreeSet<usize> = BTreeSet::new();
        exp_connections_0.insert(1);
        exp_connections_0.insert(2);
        assert_eq!(network.connections(0).unwrap(), &exp_connections_0);

        // similarly node 1 is 'connected' to only node 2.
        let mut exp_connections_1: BTreeSet<usize> = BTreeSet::new();
        exp_connections_1.insert(0);
        exp_connections_1.insert(2);
        assert_eq!(network.connections(1).unwrap(), &exp_connections_1);

        // similarly node 2 is 'connected' to only node 1.
        let mut exp_connections_1: BTreeSet<usize> = BTreeSet::new();
        exp_connections_1.insert(0);
        exp_connections_1.insert(2);
        assert_eq!(network.connections(1).unwrap(), &exp_connections_1);

        // If we try to check connections on a node that does not exist,
        // an error will be raised.
        assert!(network.connections(3).is_none());

        // Test passed.
        Ok(())
    }
}
