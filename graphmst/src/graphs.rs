//Contains definition of graph structures.
use std::collections::HashMap;

type VLT = String; //vertex_label_type

enum EdgeType {
    Directed,
    Undirected
}

//Basic undirected graph.
pub struct Graph<V, E> {
    vertices: HashMap<VLT, Vertex<V>>,
    edges: HashMap<(VLT, VLT), Edge<E>>,
    edge_type: EdgeType
}

impl<V, E> Graph<V, E> {
    pub fn new(directed: bool) -> Graph<V, E> {
        //Create an empty graph.
        let v: HashMap<VLT, Vertex<V>> = HashMap::new();
        let e: HashMap<(VLT, VLT), Edge<E>> = HashMap::new();
        let edge_type = if directed {EdgeType::Directed} else {EdgeType::Undirected};
        Graph::<V, E> {
            vertices: v,
            edges: e,
            edge_type: edge_type
        }
    }
    
    pub fn get_vertices(&mut self) -> &mut HashMap<VLT, Vertex<V>> {
        &mut self.vertices
    }
    
    pub fn add_vertex(&mut self, label: VLT, value: V) {
        //Add vertex to graph.
        if self.contains_vertex(&label) {// self.vertices.iter().any(|vert| vert.label.eq(&label)){
            //TODO: Create more sophosticated handling.
            println!("Vertex '{}' already in graph", label);
        } else {
            self.vertices.insert(label.clone(),
                Vertex {
                    label: label,
                    value: value
                }
            );            
        }
    }
    
    pub fn remove_vertex(&mut self, label: VLT) {
        // Remove vertex and all of its adjacent edges.
            
        // Find all neighbors.
        let neighbors = self.get_neighbors(&label);
        
        // Remove all edges, regardless of direction.
        // TODO: Decide on handling of directed vs undirected graphs.
        for vert_label in neighbors.into_iter() {
            self.remove_edge((label.clone(), vert_label.to_string()));
            self.remove_edge((vert_label.to_string(), label.clone()));
        }
        
        //Remove central vertex.
        self.vertices.remove(&label);
    }
    
    pub fn add_edge(&mut self, e: (VLT, VLT), weight: E){
        // Adds an edge to the graph.
        // Endpoint vertices must be present in graph.
        if self.contains_edge(&e) {
            println!("Edge '{}'-'{}' already in graph", e.0, e.1);
        } else if
            self.contains_vertex(&e.0)
            && self.contains_vertex(&e.1) {
                self.edges.insert(e.clone(),
                    Edge {
                        endpoints: e,
                        weight: weight,
                        edge_type: EdgeType::Undirected
                    }
                );
        }
    }
    
    pub fn remove_edge(&mut self, e: (VLT, VLT)){
        // Removes an edge from a graph.
        // Endpoint vertices are not affected.
        let target_edge = self.edges.get(&e);
        match target_edge {
            Some(te) => match te.edge_type {
                EdgeType::Directed => if self.edges.contains_key(&e) {
                        self.edges.remove(&e);
                    },
                EdgeType::Undirected => {
                    let re = (e.1.clone(), e.0.clone()); //reverse_edge
                    if self.edges.contains_key(&e) || self.edges.contains_key(&re) {
                            self.edges.remove(&e);
                            self.edges.remove(&re);
                    }
                }
            },
            None => println!("Edge '{}'-'{}' not in graph", e.0, e.1),
        }
    }
    
    pub fn get_neighbors(&self, label: &VLT) -> Vec<VLT> {
        //Input a vertex label.
        //Returns a vector of vertex labels which correspond to the neighbors of the input vertex.
        let mut neighbors: Vec<VLT> = Vec::<VLT>::new();
        for (edge_labels, _edge) in self.edges.iter() {
            if (label).eq(&edge_labels.0) {
                neighbors.push(edge_labels.1.clone())
            } else if (label).eq(&edge_labels.1) {
                neighbors.push(edge_labels.0.clone())
            }
        }
        neighbors
    }
    
    pub fn get_out_neighbors(&self, label: &VLT) -> Vec<VLT> {
        //Input a vertex label.
        //Returns a vector of vertex labels which correspond
        //to the outgoing neighbors of the input vertex.
        let mut neighbors: Vec<VLT> = Vec::<VLT>::new();
        for (edge_labels,_edge) in self.edges.iter() {
            if (label).eq(&edge_labels.0) {
                neighbors.push(edge_labels.1.clone())
            }
        }
        neighbors
    }
    
    pub fn get_in_neighbors(&self, label: &VLT) -> Vec<VLT> {
        //Input a vertex label.
        //Returns a vector of vertex labels which correspond
        //to the incoming neighbors of the input vertex.
        let mut neighbors: Vec<VLT> = Vec::<VLT>::new();
        for (edge_labels, _edge) in self.edges.iter() {
            if (label).eq(&edge_labels.1) {
                neighbors.push(edge_labels.0.clone())
            }
        }
        neighbors
    }
    
    
    pub fn get_vertex(&mut self, label: &VLT) -> Option<&mut Vertex<V>> {
        self.vertices.get_mut(label)
    }
    /*
    fn get_vertex(&self, label: &VLT) -> Result<&Vertex<V>, String> {
        //Input vertex label and return reference to vertex.
        
        self.vertices.get(label);
        
        if self.contains_vertex(label) {
            for vert in &self.vertices {
                if vert.label.eq(label) {
                    return Ok(vert)
                }
            }
        }
        
        //Ok(&Vertex { label: String::from("TEST"), value: val })
        //todo: Return proper error.
        Err(String::from("Vertex not in graph."))
    }
    */
    
    fn contains_vertex(&self, label: &VLT) -> bool {
        //Check if graph contain vertex with label.
        self.vertices.contains_key(label)
    }
    
    fn contains_edge(&self, e: &(VLT, VLT)) -> bool {
        //Check if graph contain an edge.
        self.edges.contains_key(e)
    }
    
    //TODO: Add function to print graph.
}

pub struct Vertex<T> {
    label: VLT,
    value: T
}

// FIXME: This is here for debugging.
impl Vertex<f64> {
    pub fn get_value(&self) -> f64 {
        self.value
    }
}

impl<V> Vertex<V> {
    pub fn set_value(&mut self, new_value: V) {
        self.value = new_value;
    }
}

impl<V> PartialEq for Vertex<V> {
    //Two vertices are equal if they have the same label.
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

pub struct Edge<T> {
    endpoints: (VLT, VLT),
    weight: T,
    edge_type: EdgeType
}

impl<T> PartialEq for Edge<T> {
    fn eq(&self, e: &Edge<T>) -> bool {
        let ends1 = &self.endpoints;
        let ends2 = &e.endpoints;
        match self.edge_type {
            EdgeType::Directed => (ends1.0).eq(&ends2.0) && (ends1.1).eq(&ends2.1),
            EdgeType::Undirected => (ends1.0).eq(&ends2.0) && (ends1.1).eq(&ends2.1)
                || (ends1.1).eq(&ends2.1) && (ends1.0).eq(&ends2.0)
        }
        
    }
}