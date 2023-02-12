//Contains definition of graph structures.

//Basic undirected graph.
pub struct Graph<V, E> {
    vertices: Vec<Vertex<V>>,
    edges: Vec<Edge<E>>,
}

impl<V, E> Graph<V, E> {
    pub fn new() -> Graph<V, E> {
        //Create an empty graph.
        let v: Vec<Vertex<V>> = Vec::new();
        let e: Vec<Edge<E>> = Vec::new();
        Graph::<V, E> {
            vertices: v,
            edges: e
        }
    }
    
    pub fn add_vertex(&mut self, label: String, value: V) {
        //Add vertex to graph.
        if self.vertices.iter().any(|g| g.label.eq(&label)){
            //TODO: Create more sophosticated handling.
            println!("Vertex '{}' already in graph", label);
        } else {
            self.vertices.push(
                Vertex {
                    label: label,
                    value: value
                }
            );            
        }
    }
    
    //pub fn add_edge(&mut self, e: (<label, label>)) { }
    
    //TODO: Add function to print graph.
}

struct Vertex<T> {
    label: String,
    value: T
}

struct Edge<T> {
    weight: T,
}