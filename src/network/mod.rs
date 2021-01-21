mod api;
mod test;

use alloc::collections::{BTreeMap, BTreeSet};
use core::default::Default;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use api::*;
use crate::error::Error;

/// `BTreeNetwork` is an implementation of a network (abstract data structure)
/// which utilizes `BTreeMap` for the edge and vertex adjacency lists.
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BTreeNetwork<T>
where
    T: Ord,
{
    vertices: BTreeMap<T, BTreeSet<T>>,
}

impl<T> BTreeNetwork<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        let vertices: BTreeMap<T, BTreeSet<T>> = BTreeMap::new();
        BTreeNetwork { vertices }
    }
}

impl<T> Default for BTreeNetwork<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Vertices<T> for BTreeNetwork<T>
where
    T: Ord,
{
    fn vertices(&self) -> BTreeSet<&T> {
        self.vertices.keys().collect()
    }
}

impl<T> AddVertex<T> for BTreeNetwork<T>
where
    T: Ord,
{
    fn add_vertex(&mut self, x: T) -> Option<BTreeSet<T>> {
        self.vertices.insert(x, BTreeSet::new())
    }
}

/// When you add an edge, you should make sure that the x, and y vertices exist.
impl<T> AddEdge<T> for BTreeNetwork<T>
where
    T: Ord + Clone,
{
    type Error = Error;
    fn add_edge(&mut self, x: T, y: T) -> Result<(), Self::Error> {
        if let Some(adj_y) = self.vertices.get(&y) {
            if let Some(adj_x) = self.vertices.get(&x) {
                // Add x to y's adjacency list.
                let mut adj_y: BTreeSet<T> = adj_y.clone();
                adj_y.insert(x.clone());

                // Add y to x's adjacency list.
                let mut adj_x: BTreeSet<T> = adj_x.clone();
                adj_x.insert(y.clone());

                self.vertices.insert(x, adj_x);
                self.vertices.insert(y, adj_y);
                return Ok(());
            }
        }
        Err(Error::VertexDoesNotExist)
    }
}

impl<T> GetVertexValue<T> for BTreeNetwork<T>
where
    T: Ord,
{
    fn get_vertex_value(&self, v: T) -> Option<&BTreeSet<T>> {
        self.vertices.get(&v)
    }
}

/// When an edge is removed, you should find the incident vertex and ensure the edge
/// is removed from the vertex's adjacency list.
impl<T> RemoveEdge<T> for BTreeNetwork<T>
where
    T: Ord + Clone,
{
    type Error = Error;
    fn remove_edge(&mut self, x: T, y: T) -> Result<(), Self::Error> {
        if let Some(adj_x) = self.vertices.get(&x) {
            if let Some(adj_y) = self.vertices.get(&y) {
                // Remove y from x's adjacency list.
                let mut adj_x = adj_x.clone();
                adj_x.remove(&y);

                // Remove x from y's adjacency list.
                let mut adj_y = adj_y.clone();
                adj_y.remove(&x);

                // Update vertices.
                self.vertices.insert(x, adj_x);
                self.vertices.insert(y, adj_y);
                return Ok(());
            }
        }
        Err(Error::VertexDoesNotExist)
    }
}

/// When you remove a vertex, you should ensure there are no dangling edges.
impl<T> RemoveVertex<T> for BTreeNetwork<T>
where
    T: Ord + Clone,
{
    type Error = Error;
    fn remove_vertex(&mut self, x: T) -> Result<(), Self::Error> {
        // When removing a vertex, of course, we should remove
        // all adjacent edges;
        if let Some(adj_x) = self.vertices.get(&x) {
            adj_x
                .clone()
                .into_iter()
                .try_for_each(|y| self.remove_edge(x.clone(), y))?;
            // At this point, the adjacency list should be empty,
            // and can be removed.
            self.vertices.remove(&x);
            return Ok(());
        }
        Err(Error::VertexDoesNotExist)
    }
}

impl<T> Adjacent<T> for BTreeNetwork<T>
where
    T: Ord,
{
    type Error = Error;
    fn adjacent(&self, x: T, y: T) -> Result<bool, Self::Error> {
        if let Some(adj_y) = self.vertices.get(&y) {
            if let Some(adj_x) = self.vertices.get(&x) {
                if adj_y.contains(&x) && adj_x.contains(&y) {
                    return Ok(true);
                }
                return Ok(false);
            }
        }
        Err(Error::VertexDoesNotExist)
    }
}

impl<T> Connections<T> for BTreeNetwork<T>
where
    T: Ord,
{
    fn connections(&self, x: T) -> Option<&BTreeSet<T>> {
        self.vertices.get(&x)
    }
}
