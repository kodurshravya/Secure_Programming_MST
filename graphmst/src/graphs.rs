//Contains definition of graph structures.

use std::collections::HashMap;

//Basic undirected graph.
struct Graph {
    vertices: vec<Vertex>
    edges: vec<Edges>;
}

struct Vertex<T> {
    value: <T>,
}

struct Edge<T> {
    value: <T>,
}