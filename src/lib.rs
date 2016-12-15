/// The Michael Jackson Graph API
///
/// ## Examples
/// ```rust
/// use michael_jackson::Graph;
/// let x = Graph::new();
/// ```

use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::linked_list;
use std::marker::PhantomData;
use std::ops::Deref;
use std::hash::Hash;
use std::cmp::Eq;

/// A data structure which represents a mathematical graph.
/// It is implemented as an adjacency list (a vector of Linked Lists) together
/// with a Vector of vertices.
pub struct Graph<V, E> {
    adj_list: Vec<LinkedList<Edge<E>>>,
    vertices: Vec<Vertex<V>>,
}

/// A vertex, can be inserted into a Graph and holds data of arbitrary type.
pub struct Vertex<V> {
    contents: V,
}

/// A private struct in the Graph's adjacency list which keeps indices to
/// both endpoints and the data associated with the edge.
struct Edge<E> {
    parent: usize,
    child: usize,
    weight: E,
}

/// Iterator struct which keeps track of the location of a vertex within the Graph
/// struct. Can be used to iterate over vertices in an arbitrary order.
pub struct VRef<'a, V: 'a, E: 'a> {
    index: usize,
    graph: &'a Graph<V, E>,
}
impl<'a, V, E> Clone for VRef<'a, V, E> {
    fn clone(&self) -> Self {
        VRef{ index: self.index, graph: self.graph }
    }
}
impl<'a, V, E> Copy for VRef<'a, V, E> {
}

impl<'a, V, E> Deref for VRef<'a, V, E> {
    type Target = V;
    fn deref(&self) -> &V {
        unimplemented!()
    }
}

pub struct VIter<'a, V: 'a, E: 'a> {
    r: VRef<'a, V, E>,
}

impl<'a, V, E> Iterator for VIter<'a, V, E> {
    type Item = VRef<'a, V, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.r.index < self.r.graph.num_vertices() {
            let old_ref = VRef{ index: self.r.index, graph: self.r.graph };
            self.r.index = self.r.index + 1;
            Some(old_ref)
        }
        else {
            None
        }
    }
}

pub struct eIter<'a, E: 'a> {
    iter: linked_list::Iter<'a, Edge<E>>
}

impl<'a, E> Deref for eIter<'a, E> {
    type Target = E;
    fn deref(&self) -> &E {
        unimplemented!()
    }
}

// Some functions require types V and E to have default values
impl<'a, V: Hash + Eq + Copy + Clone, E: Default> Graph<V, E> {
    /// Returns an iterator that ranges over all vertices in arbitrary order.
    pub fn vertices(&'a self) -> VIter<'a, V, E> {
        VIter{ r: VRef{ index: 0, graph: &self } }
    }

    /// Construct a graph without data, with default values for V and E
    /// and populates the given iters vector with iterators to the added vectors
    /// in the order they were encountered. Clears the given vector before
    /// populating.
    #[allow(unused_variables)]
    pub fn extend_with_edges(&'a self, edges: &Vec<(V, V)>) -> Vec<VRef<'a, V, E>> {
        let mut vrefs : Vec<VRef<'a, V, E>> = Vec::new();
        let mut ref_map = HashMap::new();

        for vref in self.vertices() {
            ref_map.insert(*vref, vref);
        }

        for &(u, v) in edges {
                if !ref_map.contains_key(&u) {
                    let vref : VRef<'a, V, E> = self.add_vertex(u);
                    vrefs.push(vref);
                    ref_map.insert(u, vref);
                }
                if !ref_map.contains_key(&v) {
                    let vref = self.add_vertex(v);
                    vrefs.push(vref);
                    ref_map.insert(v, vref);
                }
            self.add_edge(ref_map.get(&u).unwrap(), ref_map.get(&v).unwrap(), E::default());
        };
        vrefs
    }
    /// Construct a graph without data, with default values for V and E
    #[allow(unused_variables)]
    pub fn new_from_edges(edges: &Vec<(V, V)>) -> Self {
        //let mut v = Vec::new();
        //let g: Graph<V, E> = Graph::new_from_edges_populate_iters(edges, &mut v);
        //return g;
        unimplemented!()
    }
}

impl<V, E> Graph<V, E> {
    /// Create a new, empty graph
    pub fn new() -> Self {
        Graph{ adj_list: Vec::new(), vertices: Vec::new() }
    }

    /// Add a vertex to a graph, returning an VRef to the inserted vertex.
    /// The lifetime of the VRef is limited to the lifetime of the inserted
    /// vertex.
    #[allow(unused_variables)]
    //pub fn add_vertex(&'a mut self, v: Vertex<V>) -> 'a VRef<V, E> {
    //TODO Alex, is it even possible to put a lifetime to a nonreference opject
    //as we want to do here? We want to ensure that VRef will not outlive the
    //graph for saftey reasons.
    //One of our ideas for making this work would be to have an VRef contain a
    //reference to an index and insist the the VRef not outlive that reference.
    //We could then return an VRef out of references that do not outlive
    //their graph.
    pub fn add_vertex(&self, v: V) -> VRef<V, E> {
        unimplemented!()
        //VRef<V, E> { index: }
    }

    /// Add an edge to a graph if there is not currently an edge between those
    /// vertices.  Returns true if successful, and false otherwise.
    #[allow(unused_variables)]
    pub fn add_edge(&self, v1: &VRef<V, E>, v2: &VRef<V, E>, value: E) ->
        Option<eIter<E>> {
        //TODO Ask Alex if this return type is weird (gets back to the "should
        //we have edge VRefs?" question).
        unimplemented!()
    }

    /// Returns the old value associated with vertex v and replaces it with the
    /// given value.
    #[allow(unused_variables)]
    pub fn replace_vertex(&self, v: &VRef<V, E>, value: V) -> PhantomData<V> {
        Default::default()
    }

    /// Returns the E which was stored between vertices v1 and v2, leaving the
    /// value in its place, unless there was no such edge, in which case it
    /// lets the value die and returns None.
    #[allow(unused_variables)]
    pub fn replace_edge(&self, v1: &VRef<V, E>, v2: &VRef<V, E>, value: E) ->
        Option<E> {
        None
    }

    /// Returns a vector of terators neighboring the given vertex.
    #[allow(unused_variables)]
    pub fn get_neighbors(&self, v: &VRef<V, E>) -> Vec<VRef<V, E>> {
        Vec::new()
    }

    /// Returns whether or not the given vertices are adjacent.
    #[allow(unused_variables)]
    pub fn adjacent(&self, v1: &VRef<V, E>, v2: &VRef<V, E>) -> bool {
        true
    }

    /// Returns the number of vertices in the graph.
    #[allow(unused_variables)]
    pub fn num_vertices(&self) -> usize {
        0
    }

    /// Returns the number of edges in the graph.
    #[allow(unused_variables)]
    pub fn num_edges(&self) -> usize {
        0
    }

    /// Returns the adjacency matrix for the given graph.
    #[allow(unused_variables)]
    pub fn get_adjacency_matrix(&self) -> Vec<usize> {
        //TODO Ask Alex if Vec<Vec>> is a reasonable matrix representation.
        Vec::new()
    }

    /// Returns the Laplacian matrix for the given graph.
    #[allow(unused_variables)]
    pub fn get_laplacian(&self) -> Vec<Vec<isize> > {
        Vec::new()
    }

    /// Returns the number of connected components in the graph.
    #[allow(unused_variables)]
    pub fn num_components(&self) -> usize {
        1
    }
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


*/
