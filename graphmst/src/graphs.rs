// Contains definition of graph structures.
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader, Error};

/// vertex_label_type
type VLT = String;

/// Edge Type - Directed and Undirected Edge
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EdgeType {
    Directed,
    Undirected,
}

/// Edge Weight Type constraint enum
///
/// Weight can only be a numeric type
///
/// eg: GNumber::I32(10)
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum GNumber {
    U16(u16),
    U32(u32),
    U64(u64),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl fmt::Display for GNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GNumber::U16(x) => write!(f, "{}", x),
            GNumber::U32(x) => write!(f, "{}", x),
            GNumber::U64(x) => write!(f, "{}", x),
            GNumber::I16(x) => write!(f, "{}", x),
            GNumber::I32(x) => write!(f, "{}", x),
            GNumber::I64(x) => write!(f, "{}", x),
            GNumber::F32(x) => write!(f, "{}", x),
            GNumber::F64(x) => write!(f, "{}", x),
        }
    }
}

/// This is the basic graph structure. Graph consists of vertices, edges and edge_type
///
/// # Vertices:
///
/// Vertices - Hashmap of type String, Vertex. Vertex has a label and a value
///
/// Example: A vertex with label A and value 10 will look like this
/// ```
///  self.vertices.insert(
///     String::from("A");
///     Vertex {
///         label: String::from("A"),
///         value: 10,
///     },
/// );
/// ```
///
/// The structure of the vertex
///
/// ```
/// pub struct Vertex<T> {
///     pub label: VLT,
///     pub value: T,
/// }
/// ```
///
/// # Edges:
///
/// Edges - Hashmap of type (VLT, VLT), Edge.
///
/// (VLT, VLT) are two end points of the edge.
///
/// Edge has weight and edge type
///
/// Example:
///
/// ```
/// HashMap - Key: (String::from("A"), String::from("B")) | Value: Edge
/// ```
///
/// Edge Looks like this:
///
/// ```
/// pub struct Edge {
///     pub endpoints: (VLT, VLT),
///     pub weight: GNumber,
///     pub edge_type: EdgeType,
///}
/// ```
///
/// # Edge Type
///
/// edge_type: EdgeType - Directed or Undirected
#[derive(Clone)]
pub struct Graph {
    pub vertices: HashMap<VLT, Vertex>,
    pub edges: HashMap<(VLT, VLT), Edge>,
    pub edge_type: EdgeType,
}

impl Graph {
    /// Creates a new Graph
    ///
    /// # Parameters:
    ///
    /// directed - type boolean
    ///
    /// directed = true if we want a directed graph
    ///
    /// directed = false if we want an undirected graph
    ///
    /// # Return value
    ///
    /// This function returns Graph - directed or undirected based on the parameter passed (Graph)
    ///
    /// # Example
    ///
    /// Basic Usage:
    ///
    /// 1. Undirected graph:
    /// ```
    /// let mut g: graphs::Graph = graphs::Graph::new(false);
    /// ```
    /// 2. Directed graph:
    /// ```
    /// let mut g: graphs::Graph = graphs::Graph::new(true);
    /// ```
    pub fn new(directed: bool) -> Graph {
        //Create an empty graph.
        let v: HashMap<VLT, Vertex> = HashMap::new();
        let e: HashMap<(VLT, VLT), Edge> = HashMap::new();
        let edge_type = if directed {
            EdgeType::Directed
        } else {
            EdgeType::Undirected
        };
        Graph {
            vertices: v,
            edges: e,
            edge_type: edge_type,
        }
    }

    /// Prints the graph
    ///
    /// # usage:
    /// ```
    /// let mut g: graphs::Graph = graphs::Graph::new(false); // creates undirected graph
    /// g.print() // prints the graph
    /// ```
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

    /// Returns topological sorted order of the vertice of the graph
    ///
    pub fn get_topological_order(&mut self) -> Vec<VLT> {
        //FIXME: Function not finished.
        //TODO: Consider moving to utils.
        let mut g: Graph = Graph::new(true);
        let nodes = g.get_vertices().keys();
        // let nodes =  g.edges;
        let mut order: Vec<VLT> = vec![];
        let visited_vertex: HashMap<VLT, bool> = HashMap::new();

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
        let mut g: Graph = Graph::new(true);
        //let coming_nodes = self.get_vertices().get(node);
        let coming_nodes = g.get_vertices().keys();

        for _value in coming_nodes {
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

    pub fn get_vertices(&mut self) -> &mut HashMap<VLT, Vertex> {
        &mut self.vertices
    }

    pub fn get_edges(&mut self) -> &mut HashMap<(VLT, VLT), Edge> {
        &mut self.edges
    }

    //pub fn get_edge(&mut self, e: (VLT, VLT)) -> &mut Edge<E> {
    //    &mut self.edges.get(&e).unwrap()
    //}

    /// Add vertex to the graph
    ///
    /// # Parameters:
    ///
    /// 1. label - the label of the vertex which should be of type String
    ///
    /// 2. value - value of the vertex, any generic
    ///
    /// # Example
    ///
    /// ```
    /// let mut G: graphs::Graph = graphs::Graph::new(false); // create undirected graph
    /// g.add_vertex(String::from("A")); // add vertex to the graph with label A and value 0
    /// ```
    pub fn add_vertex(&mut self, label: VLT) {
        //Add vertex to graph.
        if self.contains_vertex(&label) {
            //TODO: Create more sophosticated handling.
            //println!("Vertex '{}' already in graph", label);
        } else {
            self.vertices.insert(
                label.clone(),
                Vertex {
                    label: label,
                    value: 0f64,
                },
            );
        }
    }

    /// Remove vertex and all of its adjacent edges.
    ///
    /// # Parameters
    ///
    /// 1. label: The label of the vertex
    ///
    /// # Example
    ///
    /// ```
    /// g.remove_vertex(String::from("A")); // Remove vertex A from the graph G
    /// ```
    ///  
    pub fn remove_vertex(&mut self, label: VLT) {
        // Find all neighbors.
        let neighbors = self.get_neighbors(&label);

        // Remove all edges, regardless of direction.
        // TODO: Decide on handling of directed vs undirected graphs.
        for vert_label in neighbors.into_iter() {
            //FIXME: Keep an eye on these '.to_string' uses.
            self.remove_edge((label.clone(), vert_label.to_string()));
            self.remove_edge((vert_label.to_string(), label.clone()));
        }

        //Remove central vertex.
        self.vertices.remove(&label);
    }

    /// Adds an edge to the graph (Endpoint vertices must be present in graph)
    ///
    /// # Parameters
    ///
    /// 1. (endpoint1, endpoint2) - the two endpoints of the edge each will be of type String
    ///
    /// 2. weight - The weight of the edge
    ///
    /// # Example
    ///
    /// ```
    /// // Edge with I32 weights having endpoints "A" and "B"
    ///  g.add_edge(
    ///     (String::from("A"), String::from('B')),
    ///     graphs::GNumber::I32(4),
    /// );
    ///
    /// // Edge with F32 weights having endpoints "A" and "B"
    /// g2.add_edge(
    ///     (String::from("A"), String::from('B')),
    ///     graphs::GNumber::F32(4.),
    /// );
    ///
    /// // Edge with U32 weights having endpoints "A" and "B"
    /// g3.add_edge(
    ///     (String::from("A"), String::from('B')),
    ///     graphs::GNumber::U32(2),
    /// );
    /// ```
    pub fn add_edge(&mut self, e: (VLT, VLT), weight: GNumber) {
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

    /// Update the weight of an edge to the graph (Edge must be present in graph)
    ///
    /// # Parameters
    ///
    /// 1. (endpoint1, endpoint2) - the two endpoints of the edge each will be of type String
    ///
    /// 2. weight - The weight of the edge
    ///
    /// # Example
    ///
    /// ```
    /// // This will update the value of the edge with endpoint (A, B) to 10 (I32 value)
    ///  g.update_edge(
    ///     (String::from("A"), String::from('B')),
    ///     graphs::GNumber::I32(10),
    /// );
    /// ```
    pub fn update_edge(&mut self, e: (VLT, VLT), weight: GNumber) {
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

    /// Removes an edge from a graph (Endpoint vertices are not affected)
    ///
    /// # Parameters
    ///
    /// 1. (endpoint1, endpoint2) - the two endpoints of the edge (type String)
    ///
    /// # Example
    ///
    /// ```
    /// // This will remove edge with endpoints A and B
    ///  g.remove_edge(
    ///     (String::from("A"), String::from('B')),
    /// );
    /// ```
    pub fn remove_edge(&mut self, e: (VLT, VLT)) {
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

    /// Input a vertex label (Returns a vector of vertex labels which correspond to the neighbors of the input vertex)
    ///
    /// # Parameter:
    ///
    /// 1. label - Label of type String
    ///
    /// # Return Value:
    ///
    /// Returns a vector of labels of all the vertices that are neighbors of this vertex
    ///
    /// # Example
    ///
    /// ```
    /// G.get_neighbors(String::from("A")) // returns all the neighbors of A
    ///
    /// // example return: ["B", "C", "D"]. If B, C and D are neighbors of A
    /// ```
    pub fn get_neighbors(&self, label: &VLT) -> Vec<VLT> {
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

    /// Input a vertex label. Returns a vector of vertex labels which correspond to the outgoing neighbors of the input vertex.
    ///
    /// # Parameter:
    ///
    /// 1. label - Label of type String
    ///
    /// # Return Value:
    ///
    /// Returns a vector of labels of all the vertices that are outgoing neighbors of this vertex.
    /// This is for a directed graph
    ///
    /// # Example
    ///
    /// ```
    /// g.get_out_neighbors(String::from("A")) // returns all the  outgoing neighbors of A
    ///
    /// // example return: ["B", "C", "D"].
    /// // A -> B, A -> C, A -> D
    pub fn get_out_neighbors(&self, label: &VLT) -> Vec<VLT> {
        let mut neighbors: Vec<VLT> = Vec::<VLT>::new();
        for (edge_labels, _edge) in self.edges.iter() {
            if (label).eq(&edge_labels.0) {
                neighbors.push(edge_labels.1.clone())
            }
        }
        neighbors
    }

    /// Input a vertex label. Returns a vector of vertex labels which correspond to the incoming neighbors of the input vertex.
    ///
    /// # Parameter:
    ///
    /// 1. label - Label of type String
    ///
    /// # Return Value:
    ///
    /// Returns a vector of labels of all the vertices that are incoming neighbors of this vertex.
    /// This is for a directed graph
    ///
    /// # Example
    ///
    /// ```
    /// G.get_in_neighbors(String::from("A")) // returns all the incoming neighbors of A
    ///
    /// // example return: ["B", "C", "D"].
    /// // B -> A, C -> A, D -> A
    pub fn get_in_neighbors(&self, label: &VLT) -> Vec<VLT> {
        let mut neighbors: Vec<VLT> = Vec::<VLT>::new();
        for (edge_labels, _edge) in self.edges.iter() {
            if (label).eq(&edge_labels.1) {
                neighbors.push(edge_labels.0.clone())
            }
        }
        neighbors
    }

    // TODO: Documentation
    /// Reads an adjacency matrix from a file and returns it as a `Vec<Vec<u32>>`
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

    // TODO: Documentation
    /// Writes an adjacency matrix to a file.
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

    /// Function to get the vertex given the label
    ///
    /// # Parameters:
    ///
    /// 1. label - Label of the vertex - type String
    ///
    /// # Return Type:
    ///
    /// Returns an Option of type mutable `Vertex`. If there are no vertex with the provided label - None will be returned
    ///
    /// # Example
    ///
    /// ```
    /// let vertex_A = g.get_vertex(String::from("A")); // this wil return the vertex A which is mutable (We can change the value of the vertex)
    /// ```
    pub fn get_vertex(&mut self, label: &VLT) -> Option<&mut Vertex> {
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

    /// Function to check if the given vertex is present in the graph
    ///
    /// # Parameters
    ///
    /// 1. label - Label of the vertex - type String
    ///
    /// # Return Type
    ///
    /// Returns a boolean value.
    ///
    /// true - if the vertex is present in the graph
    ///
    /// false - if the vertex is not present in the graph
    ///
    /// # Example
    ///
    /// ```
    /// if g.contains_vertex(String::from("A")){
    ///     // Do something
    /// }
    /// ```
    fn contains_vertex(&self, label: &VLT) -> bool {
        //Check if graph contain vertex with label.
        self.vertices.contains_key(label)
    }

    /// Function to check if the given edge is present in the graph
    ///
    /// # Parameters
    ///
    /// 1. (endpoint1, endpoint2) - endpoints of the edge (String, String)
    ///
    /// # Return Type
    ///
    /// Returns a boolean value.
    ///
    /// true - if the edge is present in the graph
    ///
    /// false - if the edge is not present in the graph
    ///
    /// # Example
    ///
    /// ```
    /// // Check if the edge A-B is present in the graph
    /// // Note if the graph is directed, it will return true only if the edge A -> B is present. B -> A will not be counted
    /// if g.contains_edge((String::from("A"), String::from("B"))){
    ///     // Do something
    /// }
    /// ```
    fn contains_edge(&self, e: &(VLT, VLT)) -> bool {
        //Check if graph contain an edge.
        self.edges.contains_key(e)
    }

    //TODO: Add function to print graph.
}

//Internal macro that matches the pattern of a single expession (indicating the user would like to add a vertex,
//or a tuple-like pattern (str, i32, str), indicating the user would like an edge.
#[allow(unused_macros)]
macro_rules! edg_or_vert {
    ( $G:expr, ($a:literal, $b:literal, $c:literal) ) => {
        {
            $G.add_vertex(String::from($a));
            $G.add_vertex(String::from($c));
            $G.add_edge((String::from($a), String::from($c)), GNumber::I32($b));
            println!( "{}, {}, {}", $a, $b, $c );
        }
    };

    ( $G:expr, $($x:expr ),* ) => {
        {
            {
                $(
                    $G.add_vertex(String::from($x));
                    println!("{}", String::from($x));
                )*
            }
        }
    };

}

/// Function to check if the given vertex is present in the graph
///
/// # Parameters
///
/// 1. label - Label of the vertex - type String
///
/// # Return Type
///
/// Returns a boolean value.
///
/// true - if the vertex is present in the graph
///
/// false - if the vertex is not present in the graph
///
/// # Example
///
/// ```
/// if g.contains_vertex(String::from("A")){
///     // Do something
/// }
/// ```

///Build an undirected graph
///
///This macro can make both vertices and edges.
///For a vertex, simple pass a string literal to be that vertex's label.
///For an edge, write a pattern of the form (str, i32, str) where the first and last element represent the label of a vertex, and the middle value is the edges weight.
///
///# Example
/// ```
/// let G = gph!("A", "B", "C", ("A", 3, "C"), ("B", 7, "D"))
/// ```
///Notice that we do not need to list all vertices before adding edges for them, as shown in the last edge pattern.
#[macro_export]
macro_rules! gph {
    ( $($sub:tt),* ) => { //iterate over every token. Could be a single string or an edge tuple.
        {
            let mut g: Graph = Graph::new(false);
            $(
                edg_or_vert!(&mut g, $sub);
            )*
            g
        }
    };
    /*
    ( $($x:expr ),* ) => {
        {
            let mut g: Graph = Graph::new(false);
            {
                $(
                    g.add_vertex(String::from($x));
                )*
                G
            }
        }
    };
    */
}

/// Vertex Structure
///
/// The structure of the vertex
///
/// A vertex has a label and a value
///
/// Label is a string and value is f64
#[derive(Debug, Clone)]
pub struct Vertex {
    pub label: VLT,
    pub value: f64,
}

impl Vertex {
    pub fn get_value(&self) -> f64 {
        self.value.clone()
    }
}

impl Vertex {
    pub fn set_value(&mut self, new_value: f64) {
        self.value = new_value;
    }
}

impl PartialEq for Vertex {
    //Two vertices are equal if they have the same label.
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.partial_cmp(&other.weight).unwrap()
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

/// Edge Structure
///
/// Edges have three fields
///
/// 1. endpoints (a,b) - this contains the info of the two vertices of the edge (A -- B)
/// 2. weight - the weight of the edge. It's of type GNumber
/// 3. edge_type - the type of the edge (Directed / Undirected)
#[derive(Debug, Clone)]
pub struct Edge {
    pub endpoints: (VLT, VLT),
    pub weight: GNumber,
    pub edge_type: EdgeType,
}

impl PartialEq for Edge {
    fn eq(&self, e: &Edge) -> bool {
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

/// Test cases
#[cfg(test)]
mod graph_tests {
    //extern crate graphs;
    //use graphs::Graph;
    use super::*;

    fn get_test_graph_1() -> Graph {
        let mut g: Graph = Graph::new(false);
        g.add_vertex(String::from("A"));
        g.add_vertex(String::from("B"));
        g.add_vertex(String::from("C"));
        g.add_vertex(String::from("D"));
        g.add_vertex(String::from("E"));
        g.add_vertex(String::from("F"));
        g.add_vertex(String::from("G"));
        g.add_vertex(String::from("H"));
        g.add_vertex(String::from("I"));
        g
    }

    #[test]
    fn add_one_vertex() {
        let mut g: Graph = Graph::new(false);
        g.add_vertex(String::from("A"));
        assert_eq!(g.get_vertices().len(), 1);
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().label, "A");
    }

    #[test]
    fn add_multiple_vertices() {
        let mut g = get_test_graph_1();
        assert_eq!(g.get_vertices().len(), 9);
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().label, "A");
        assert_eq!(g.get_vertex(&String::from("C")).unwrap().label, "C");
        assert_eq!(g.get_vertex(&String::from("H")).unwrap().label, "H");
        assert_eq!(g.get_vertex(&String::from("H")).unwrap().label, "H");
        assert_eq!(g.get_vertex(&String::from("I")).unwrap().label, "I");
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
        let mut g = get_test_graph_1();
        g.remove_vertex(String::from("I"));
        g.remove_vertex(String::from("H"));
        assert_eq!(g.get_vertices().len(), 7);
        g.remove_vertex(String::from("E"));
        assert_eq!(g.get_vertices().len(), 6);
        g.remove_vertex(String::from("A"));
        g.remove_vertex(String::from("B"));
        assert_eq!(g.get_vertices().len(), 4);
        g.remove_vertex(String::from("I"));
        assert_eq!(g.get_vertices().len(), 4);
        g.remove_vertex(String::from("G"));
        g.remove_vertex(String::from("F"));
        g.remove_vertex(String::from("D"));
        g.remove_vertex(String::from("C"));
        assert_eq!(g.get_vertices().len(), 0);
    }

    #[test]
    fn add_one_undirected_edge() {
        let mut g = get_test_graph_1();
        g.add_edge((String::from("A"), String::from('B')), GNumber::F64(4.));
        assert_eq!(g.get_edges().len(), 1);
    }

    #[test]
    fn make_from_macro() {
        let mut g = gph!("A", "B");
        assert_eq!(g.get_vertices().len(), 2);
        let mut g = gph!("C", ("A", 5, "B"));
        assert_eq!(g.get_vertices().len(), 3);
        assert_eq!(g.get_edges().len(), 1);
    }
}
