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
        if self.contains_vertex(&label) {// self.vertices.iter().any(|vert| vert.label.eq(&label)){
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
    
    pub fn add_edge(&mut self, e: (VLT, VLT), weight: E){
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
    }
    
    
    pub fn get_neighbors(&self, label: &VLT) -> Vec<VLT> {
        //Input a vertex label.
        //Returns a vector of vertex labels which correspond to the neighbors of the input vertex.
        let mut neighbors: Vec<VLT> = Vec::<VLT>::new();
        for edge in &self.edges {
            if (edge.endpoints.0).eq(label) && !neighbors.contains(&edge.endpoints.1) {
                neighbors.push(edge.endpoints.1.clone())
            } else if (edge.endpoints.1).eq(label) && !neighbors.contains(&edge.endpoints.0) {
                neighbors.push(edge.endpoints.0.clone())
            }
        }
        neighbors
    }
    
    
    //todo: Make this return result.
    fn get_vertex(&self, label: &VLT) -> Result<&Vertex<V>, String> {
        //Input vertex label and return reference to vertex.
        
        if self.contains_vertex(label) {
            for vert in &self.vertices {
                if vert.label.eq(label) {
                    return Ok(vert)
                }
            }
        }
        
        //Ok(&Vertex { label: String::from("TEST"), value: val })
        Err(String::from("Vertex not in graph."))
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

impl<V> PartialEq for Vertex<V> {
    //Two vertices are equal if they have the same label.
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

struct Edge<T> {
    //endpoints: &'a (Vertex<V>, Vertex<V>),
    //TODO: Change endpoints from vertex labels to vertex references.
        // This requires adding lifetimes.
    endpoints: (VLT, VLT),
    weight: T
}