//Contains definition of graph structures.
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader, Error};
use std::cmp::Ordering;

type VLT = String; //vertex_label_type

#[derive(Debug, Copy, Clone)]
pub enum EdgeType {
    Directed,
    Undirected,
}

//Basic undirected graph.
pub struct Graph<V, E>
where
    V: Clone,
    E: Clone,
{
    pub vertices: HashMap<VLT, Vertex<V>>,
    pub edges: HashMap<(VLT, VLT), Edge<E>>,
    pub edge_type: EdgeType,

}

impl<V, E: Clone> Graph<V, E>
where
    E: Clone + Debug,
    V: Clone + Debug,
{
    pub fn print(&self) {
        println!("Vertices:");
        for (id, vertex) in &self.vertices {
            println!("{:?}: {:?}", id, vertex);
        }

        println!("Edges:");
        for ((src, dst), edge) in &self.edges {
            println!("({:?}, {:?}) -> {:?}", src, dst, edge);
        }
    }

    pub fn new(directed: bool) -> Graph<V, E> {
        //Create an empty graph.
        let v: HashMap<VLT, Vertex<V>> = HashMap::new();
        let e: HashMap<(VLT, VLT), Edge<E>> = HashMap::new();
        let edge_type = if directed {
            EdgeType::Directed
        } else {
            EdgeType::Undirected
        };
        Graph::<V, E> {
            vertices: v,
            edges: e,
            edge_type: edge_type,
        }
    }

    pub fn get_topological_order(&mut self) -> Vec<VLT> {
        //FIXME: Function not finished.
        //TODO: Consider moving to utils.
        let mut g: Graph<f64, f64> = Graph::new(true);
        let nodes = g.get_vertices().keys();
        // let nodes =  g.edges;
        let mut order: Vec<VLT> = vec![];
        let mut visited_vertex: HashMap<VLT, bool> = HashMap::new();
        
        for node in nodes {
            if visited_vertex.get(node) == None {
                self.get_order(node, &mut order);
            }
        }
        order.reverse();
        println!("{:?}", order);
        return order;
    }

    pub fn get_order(&mut self, node: &VLT, order: &mut Vec<VLT>) {
        //TODO: Consider moving to utils.
        let mut g: Graph<f64, f64> = Graph::new(true);
        //let coming_nodes = self.get_vertices().get(node);
        let coming_nodes = g.get_vertices().keys();

        for value in coming_nodes {
            self.get_order(node, order)
        }
        // if new_graph.get(node) == None {
        // if coming_nodes != None {
        //     for value in coming_nodes. {
        //         self.get_order(value, order);
        //     }
        // }
        if !order.contains(node) {
            order.push(node.to_string()); //FIXME: Is .to_string needed here?
        }
    }

    pub fn get_vertices(&mut self) -> &mut HashMap<VLT, Vertex<V>> {
        &mut self.vertices
    }
    
    pub fn get_edges(&mut self) -> &mut HashMap<(VLT, VLT), Edge<E>> {
        &mut self.edges
    }
    
    //pub fn get_edge(&mut self, e: (VLT, VLT)) -> &mut Edge<E> {
    //    &mut self.edges.get(&e).unwrap()
    //}

    pub fn add_vertex(&mut self, label: VLT, value: V) {
        //Add vertex to graph.
        if self.contains_vertex(&label) {
            // self.vertices.iter().any(|vert| vert.label.eq(&label)){
            //TODO: Create more sophosticated handling.
            println!("Vertex '{}' already in graph", label);
        } else {
            self.vertices.insert(
                label.clone(),
                Vertex {
                    label: label,
                    value: value,
                },
            );
        }
    }

    pub fn remove_vertex(&mut self, label: VLT) {
        // Remove vertex and all of its adjacent edges.

        // Find all neighbors.
        let neighbors = self.get_neighbors(&label);

        // Remove all edges, regardless of direction.
        // TODO: Decide on handling of directed vs undirected graphs.
        for vert_label in neighbors.into_iter() { //FIXME: Keep an eye on these '.to_string' uses.
            self.remove_edge((label.clone(), vert_label.to_string()));
            self.remove_edge((vert_label.to_string(), label.clone()));
        }

        //Remove central vertex.
        self.vertices.remove(&label);
    }

    //FICME: VLT ~is~ a String. This function isn't needed.
    pub fn get_vertices_from_edge(e: (VLT, VLT)) -> (String, String) {
        (e.0, e.1)
    }

    pub fn add_edge(&mut self, e: (VLT, VLT), weight: E) {
        // Adds an edge to the graph.
        // Endpoint vertices must be present in graph.
        
        let edge_type = self.edge_type;

        let is_undirected = match edge_type {
            EdgeType::Directed => false,
            EdgeType::Undirected => true,
        };

        if self.contains_edge(&e) {
            println!("Edge '{}'-'{}' already in graph", e.0, e.1);
            return;
        }

        if is_undirected {
            let rev = (e.1.clone(), e.0.clone());
            if self.contains_edge(&rev) {
                println!("Edge '{}'-'{}' already in graph", e.1, e.0);
                return;
            }
        }

        if self.contains_vertex(&e.0) && self.contains_vertex(&e.1) {
            self.edges.entry(e.clone()).or_insert(Edge {
                endpoints: e,
                weight: weight,
                edge_type,
            });
        }
    }

    pub fn update_edge(&mut self, e: (VLT, VLT), weight: E) {
        // Update the weight of an edge to the graph.
        // Edge must be present in graph.
        if self.contains_edge(&e) {
            self.edges.insert(
                e.clone(),
                Edge {
                    endpoints: e,
                    weight: weight,
                    edge_type: EdgeType::Undirected,
                },
            );
        }
    }

    pub fn remove_edge(&mut self, e: (VLT, VLT)) {
        // Removes an edge from a graph.
        // Endpoint vertices are not affected.
        let target_edge = self.edges.get(&e);
        match target_edge {
            Some(te) => match te.edge_type {
                EdgeType::Directed => {
                    if self.edges.contains_key(&e) {
                        self.edges.remove(&e);
                    }
                }
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
        for (edge_labels, _edge) in self.edges.iter() {
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

    // Reads an adjacency matrix from a file and returns it as a Vec<Vec<u32>>.
    pub fn read_adjacency_matrix(filename: &str) -> Result<Vec<Vec<u32>>, Error> {
        // Open the file for reading.
        let file = File::open(filename)?;
        // Create a buffered reader to read the file line by line.
        let reader = BufReader::new(file);
        // Initialize an empty vector to hold the matrix.
        let mut matrix: Vec<Vec<u32>> = Vec::new();
        // Iterate over each line in the file.
        for line in reader.lines() {
            // Parse each line as a vector of u32 values.
            let row: Vec<u32> = line?
                .split_whitespace() // Split the line by space.
                .map(|s| s.parse().unwrap()) // Parse each value as u32
                .collect(); // Collect the values into a vector.
                            // Add the row to the matrix.
            matrix.push(row);
        }

        // Return the completed matrix.
        Ok(matrix)
    }

    // Writes an adjacency matrix to a file.
    pub fn write_adjacency_matrix(matrix: &[Vec<u32>], filename: &str) -> Result<(), Error> {
        // Open the file for writing.
        let mut file = File::create(filename)?;

        // Iterate over each row in the matrix.
        for row in matrix.iter() {
            // Convert the row to a string, separating each value with a space.
            let row_str = row
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ");

            // Write the row string to the file, followed by a newline character.
            writeln!(file, "{}", row_str)?;
        }

        // Return success.
        Ok(())
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

#[derive(Debug, Clone)]
pub struct Vertex<T> {
    pub label: VLT,
    pub value: T,
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

//impl<E: Eq> PartialEq for Edge<E> {
    //fn eq(&self, other: &Self) -> bool {
        //self.weight.eq(&other.weight)
    //}
//}

impl<E: Eq> Eq for Edge<E> {}


impl<E: Ord> Ord for Edge<E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<E: PartialOrd> PartialOrd for Edge<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}



#[derive(Debug, Clone)]
pub struct Edge<T> {
    pub endpoints: (VLT, VLT),
    pub weight: T,
    pub edge_type: EdgeType,
}

impl<T> PartialEq for Edge<T> {
    fn eq(&self, e: &Edge<T>) -> bool {
        let ends1 = &self.endpoints;
        let ends2 = &e.endpoints;
        match self.edge_type {
            EdgeType::Directed => (ends1.0).eq(&ends2.0) && (ends1.1).eq(&ends2.1),
            EdgeType::Undirected => {
                (ends1.0).eq(&ends2.0) && (ends1.1).eq(&ends2.1)
                    || (ends1.1).eq(&ends2.1) && (ends1.0).eq(&ends2.0)
            }
        }
    }
}


#[cfg(test)]
mod graph_tests {
    //extern crate graphs;
    //use graphs::Graph;
    use super::*;
    
    fn get_test_graph_1() -> Graph<f64, f64> {
        let mut g: Graph<f64, f64> = Graph::new(false);
        g.add_vertex(String::from("A"), 0.);        
        g.add_vertex(String::from("B"), 1.);
        g.add_vertex(String::from("C"), 2.);
        g.add_vertex(String::from("D"), 3.);
        g.add_vertex(String::from("E"), 4.);
        g.add_vertex(String::from("F"), 5.);
        g.add_vertex(String::from("G"), 6.);
        g.add_vertex(String::from("H"), 7.);
        g.add_vertex(String::from("I"), 8.);
        g
    }

    #[test]
    fn add_one_vertex() {
        let mut g: Graph<f64, f64> = Graph::new(false);
        g.add_vertex(String::from("A"), 0f64);
        assert_eq!(g.get_vertices().len(), 1);
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().label, "A");
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().get_value(), 0f64);
    }
    
    #[test]
    fn add_multiple_vertices() {
        let mut g = get_test_graph_1();        
        assert_eq!(g.get_vertices().len(), 9);
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().label, "A");
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().get_value(), 0.);
        assert_eq!(g.get_vertex(&String::from("C")).unwrap().label, "C");
        assert_eq!(g.get_vertex(&String::from("C")).unwrap().get_value(), 2.);
        assert_eq!(g.get_vertex(&String::from("H")).unwrap().label, "H");
        assert_eq!(g.get_vertex(&String::from("H")).unwrap().get_value(), 7.);
        assert_eq!(g.get_vertex(&String::from("H")).unwrap().label, "H");
        assert_eq!(g.get_vertex(&String::from("H")).unwrap().get_value(), 7.);
        assert_eq!(g.get_vertex(&String::from("I")).unwrap().label, "I");
        assert_eq!(g.get_vertex(&String::from("I")).unwrap().get_value(), 8.);
    }
    
    
    #[test]
    fn remove_one_vertex() {
        let mut g = get_test_graph_1();
        g.remove_vertex(String::from("F"));
        assert_eq!(g.get_vertices().len(), 8);
        assert_eq!(g.get_vertices().get("F").is_none(), true);
    }
    
    #[test]
    fn remove_multiple_vertices() {
        let mut G = get_test_graph_1();
        G.remove_vertex(String::from("I"));
        G.remove_vertex(String::from("H"));
        assert_eq!(G.get_vertices().len(), 7);
        G.remove_vertex(String::from("E"));
        assert_eq!(G.get_vertices().len(), 6);
        G.remove_vertex(String::from("A"));
        G.remove_vertex(String::from("B"));
        assert_eq!(G.get_vertices().len(), 4);
        G.remove_vertex(String::from("I"));
        assert_eq!(G.get_vertices().len(), 4);
        G.remove_vertex(String::from("G"));
        G.remove_vertex(String::from("F"));
        G.remove_vertex(String::from("D"));
        G.remove_vertex(String::from("C"));
        assert_eq!(G.get_vertices().len(), 0);
    }
    
    #[test]
    fn add_one_undirected_edge() {
        let mut G = get_test_graph_1();
        G.add_edge(
            (String::from("A"), String::from('B')),
            4.,
        );
        assert_eq!(G.get_edges().len(), 1);
    }
        
}