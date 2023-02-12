//Contains definition of graph structures.

//Basic undirected graph.
pub struct Graph<V, E> {
    vertices: Vec<Vertex<V>>,
    edges: Vec<Edge<E>>,
}

impl<V, E> Graph<V, E> {
    pub fn new() -> Graph<V, E> {
        let v: Vec<Vertex<V>> = Vec::new();
        let e: Vec<Edge<E>> = Vec::new();
        Graph::<V, E> {
            vertices: v,
            edges: e,
        }
    }
}

struct Vertex<T> {
    value: T,
}

struct Edge<T> {
    weight: T,
}