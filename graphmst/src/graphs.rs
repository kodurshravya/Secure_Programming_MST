//Contains definition of graph structures.


type VLT = String; //vertex_label_type

//Basic undirected graph.
pub struct Graph<V, E> {
    vertices: Vec<Vertex<V>>,
    edges: Vec<Edge<E>>
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
    
    pub fn add_vertex(&mut self, label: VLT, value: V) {
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
    
    pub fn add_edge(&mut self, e: (VLT, VLT), weight: E) {
        if self.contains_edge(&e) {
            println!("Edge '{}'-'{}' already in graph", e.0, e.1);
        } else if
            self.contains_vertex(&e.0)
            && self.contains_vertex(&e.1) {
                self.edges.push(
                    Edge {
                        endpoints: e,
                        weight: weight
                    }
                )
        }
        /*
        if self.contains_vertex(&e.0) && self.contains_vertex(&e.1) {
            println!("{}, {}", e.0, e.1);

        } else {
            println!("Edge '{}'-'{}' already in graph", e.0, e.1);
        }
        */
    }
    
    fn contains_vertex(&self, label: &VLT) -> bool {
        //Check if graph contain vertex with label.
        self.vertices.iter().any(|vert| vert.label.eq(label))
    }
    
    fn contains_edge(&self, e: &(VLT, VLT)) -> bool {
        //Check if graph contain an edge.
        self.edges.iter().any(|edg| (edg.endpoints.0).eq(&e.0)
        && (edg.endpoints.1).eq(&e.1))
    }
    
    //TODO: Add function to print graph.
}

struct Vertex<T> {
    label: VLT,
    value: T
}

/*
impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}
*/

struct Edge<T> {
    //endpoints: &'a (Vertex<V>, Vertex<V>),
    //TODO: Change endpoints from vertex labels to vertex references.
        // This requires adding lifetimes.
    endpoints: (VLT, VLT),
    weight: T
}