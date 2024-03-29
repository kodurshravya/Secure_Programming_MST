use crate::graphs::*;
use crate::util::DisjointSet;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;

use std::sync::{Arc, Mutex};
use std::thread;

type VLT = String; // vertex label type

const INF: f64 = f64::INFINITY;

//type TMPV = f64; // Should be V, but I'm being specific so I can debug.
/// Dijkstra Algorithm - Find the single source shortest path given a graph and a starting vertex
///
/// # Parameters:
///
/// 1. g - the graph that needs to be traversed. This will be of type `Graph`
/// 2. start_vertex - the source vertex from which you want to find the shortest distance of all other vertex
///
/// # Return Value:
///
/// Void
///
///
/// # Example Usage:
///
/// ```
///
/// use graphalgos::algos;
/// use graphalgos::graphs;
///
/// let mut g: graphs::Graph = graphs::Graph::new(false); // creates an undirected graph
///
/// // Add vertices
///
/// g.add_vertex(String::from("A")); // add vertex A
/// g.add_vertex(String::from("B")); // add vertex B
/// ...
/// ...
/// g.add_vertex(String::from("I")); // add vertex I
///
/// // Add edges
///
/// // Add multiple edges
/// g.add_edge(
///     (String::from("A"), String::from('B')),
///     graphs::GNumber::I32(4),
/// );
/// ...
/// ...
/// algos::dijkstra(g);
///
/// ```
///
fn _dijkstra<E>(mut g: Graph, start_vertex: VLT) {
    //FIXME: Finish implementation.
    println!("Beginning Dikstra's algorithm.");

    // let prev: HashMap<Vertex<TMPV>, Option<Vertex<TMPV>>> = HashMap::new();
    let mut prev: HashMap<VLT, Option<VLT>> = HashMap::new();

    for (lbl, vertex) in g.get_vertices().iter_mut() {
        //Initialize all vertex values to Inf.
        //We will interpret this value as the distance to the vertex.
        (*vertex).set_value(INF);

        //Initialize previous to none.
        prev.insert(lbl.clone(), None);
    }

    //Initialize distance to start as 0.
    g.get_vertex(&start_vertex).unwrap().set_value(0.0);

    //Can maybe convert to binary heap so we have ordering.
    //let heap: BinaryHeap<_> = g.get_vertices().values().collect();
}

fn dfs(g: &mut Graph, start_vertex: VLT) -> HashMap<VLT, bool> {
    //Stack will hold all vertices. Algorithm will end when all vertices have been popped.
    let stack: Arc<Mutex<VecDeque<Vertex>>> = Arc::new(Mutex::new(VecDeque::new()));
    //Hashmap letting us know which vertices have been visited by dfs.
    let visited: Arc<Mutex<HashMap<VLT, bool>>> = Arc::new(Mutex::new(HashMap::new()));
    //Initialize visited.
    for (lbl, _) in g.get_vertices().iter() {
        (*visited).lock().unwrap().insert((*lbl).clone(), false);
    }
    //Populate stack.
    (*stack)
        .lock()
        .unwrap()
        .push_front(g.get_vertex(&start_vertex).unwrap().clone());

    //Because of interactions between lifetimes and Arc's, we have to clone the graph.
    //It could greatly speed up our algorithm if we found a way to avoid this.
    let h = Arc::new(Mutex::new(g.clone()));

    let mut handles: Vec<thread::JoinHandle<_>> = vec![]; //Vector of thread handles.
    let max_num_threads = 10; //Maximum number of theads allowed at a time.
    let num_threads = Arc::new(Mutex::new(0)); //Counter to keep track of number of threads.
    while !(*(*stack).lock().unwrap()).is_empty() {
        //While stack is not empty
        let stack_clone = Arc::clone(&stack);
        let visited_clone = Arc::clone(&visited);
        let g_clone = Arc::clone(&h);

        let num_threads = Arc::clone(&num_threads);
        if *num_threads.lock().unwrap() < max_num_threads {
            //Limit the number of threads.
            {
                *num_threads.lock().unwrap() += 1;
            }
            let handler = thread::spawn(move || {
                let mut sc = stack_clone.lock().unwrap();
                if let Some(v) = sc.pop_front() {
                    //Pop vertex off of stack.
                    let mut vis = visited_clone.lock().unwrap();
                    if !vis.get(&v.label).unwrap() {
                        //If vertex has not already been visited:
                        vis.insert(v.label.clone(), true); //Label vertex as visited.
                        let mut int_g = g_clone.lock().unwrap();
                        for neighbor in int_g.get_neighbors(&v.label).iter() {
                            //Push all unvisited neighbors onto the stack.
                            if !vis.get(neighbor).unwrap() {
                                sc.push_front((int_g.get_vertex(neighbor).unwrap()).clone());
                            }
                        }
                    }
                } else {
                    //This means the algorithm has finished, and we can begin wrapping up threads.
                }
                *num_threads.lock().unwrap() -= 1;
            });
            handles.push(handler);
        }
    }

    //Make sure all threads have finished.
    for handle in handles {
        let _ = handle.join();
    }

    //Return the visited hashmap.
    let x = (*visited.lock().unwrap()).clone();
    x
}

// pub fn bellman_ford<E>(mut _g: Graph, _start_vertex: VLT)
// where
//     E: Clone,
// {
//     println!("Beginning the Bellman-Ford algorithm.");
// }

/// Kruskals Algorithm - Generate MST for any graph using the Kruskal's Algorithm
///
/// # Parameters:
///
/// 1. g - the graph that needs to be converted to MST. This will be of type `Graph`
///
/// # Return Value:
///
/// This function returns a result, which will be either a Graph - the MST that was generated using the algo or a `Error<String>` in case of any error.
///
/// The common errors would be - if graph is directed or if MST cannot be generated for the given graph
///
///
/// # Example Usage:
///
/// ```
///
/// use graphalgos::algos;
/// use graphalgos::graphs;
///
/// let mut g: graphs::Graph = graphs::Graph::new(false); // creates an undirected graph
///
/// // Add vertices
///
/// g.add_vertex(String::from("A")); // add vertex A
/// g.add_vertex(String::from("B")); // add vertex B
/// ...
/// ...
/// g.add_vertex(String::from("I")); // add vertex I
///
/// // Add edges
///
/// // Add multiple edges
/// g.add_edge(
///     (String::from("A"), String::from('B')),
///     graphs::GNumber::I32(4),
/// );
/// ...
/// ...
/// let mst = algos::kruskals(g); // get the mst using kruskals algorithm
///
/// // kruskals returns results, so use match statement to process it
/// match mst{
///     Ok(g) => g.print(), // print the MST if generated successfully
///     Err(e) => println!("{}", e), // print the error if any
/// }
///
/// ```
///
pub fn kruskals(mut g: Graph) -> Result<Graph, String> {
    // check if graph has directed edges - Kruskals work on undirected graph and not directed
    let is_directed = match g.edge_type {
        EdgeType::Directed => true,
        EdgeType::Undirected => false,
    };

    // return error if the graph has directed edges
    if is_directed {
        return Err(String::from(
            "Kruskals only work properly on undirected graphs!",
        ));
    }

    // Check for empty or trivial graph
    if g.get_vertices().len() <= 1 {
        return Ok(g);
    }

    // println!("{}", g.edges.len());

    // vector to collect all edge values
    let mut edges: Vec<Edge> = Vec::new();

    // fill the vector with edges in graph
    for (_, edge) in &g.edges {
        edges.push(edge.clone());
    }

    edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

    // The graph that we are going to return
    let mut mst = Graph::new(false);

    // Use Disjoint set for union find algorithm
    let mut set = DisjointSet::new();

    // Add all the vertices to the disjoint set
    for (node, _) in &g.vertices {
        set.set_insert(node.clone());
    }

    // iterate over edges - smallest weight to largest weight
    for edge in edges {
        let u = edge.endpoints.0.clone(); // get the first vertex of the edge
        let v = edge.endpoints.1.clone(); // get the second vertex of the edge
        set.find(&u); // Find parent of u

        // check if they are in different sets
        if set.find(&u) != set.find(&v) {
            // If they are in different sets then we join them using union and also use the edge in MST
            mst.add_vertex(u.clone()); // add vertex u to mst
            mst.add_vertex(v.clone()); // add vertex v to mst
            mst.add_edge((u.clone(), v.clone()), edge.weight.clone());
            set.union(&u, &v);
        }
    }

    // check if MST is successfull
    if mst.edges.len() != mst.vertices.len() - 1 {
        return Err(String::from(
            "MST doesn't exist for this graph since it is not connected",
        ));
    }

    println!("\nMST generated using Kruskal's algorithm: \n");

    for (_, edge) in &mst.edges {
        println!(
            "({}) -------[{}]------- ({})",
            edge.endpoints.0.clone(),
            edge.weight,
            edge.endpoints.1.clone()
        );
    }

    println!("");

    Ok(mst)
}

/// Boruvka's Algorithm - Generate MST for any graph using the Boruvka's Algorithm
///
/// # Parameters:
///
/// 1. g - the graph that needs to be converted to MST. This will be of type `Graph`
///
/// # Return Value:
///
/// This function returns a result, which will be either a Graph - the MST that was generated using the algo or a `Error<String>` in case of any error.
///
/// The common errors would be - if graph is directed or if MST cannot be generated for the given graph
///
///
/// # Example Usage:
///
/// ```
/// use graphalgos::algos;
/// use graphalgos::graphs;

/// let mut g: graphs::Graph = graphs::Graph::new(false); /// creates an undirected graph

/// // Add vertices

/// g.add_vertex(String::from("A")); // add vertex A
/// g.add_vertex(String::from("B")); // add vertex B

/// g.add_vertex(String::from("I")); // add vertex I

/// // Add edges

/// // Add multiple edges
/// g.add_edge(
///     (String::from("A"), String::from('B')),
///     graphs::GNumber::I32(4),
/// );

/// let mst = algos::boruvka(g); // get the mst using boruvkas algorithm

/// // boruvkas returns results, so use match statement to process it
/// match mst {
///     Ok(g) => g.print(),          // print the MST if generated successfully
///     Err(e) => println!("{}", e), // print the error if any
/// }
/// ```
///
pub fn boruvka(mut g: Graph) -> Result<Graph, String> {
    // check if graph has directed edges - boruvkas work on undirected graph and not directed
    let is_directed = match g.edge_type {
        EdgeType::Directed => true,
        EdgeType::Undirected => false,
    };

    // return error if the graph has directed edges

    if is_directed {
        return Err(String::from(
            "Boruvka's only work properly on undirected graphs!",
        ));
    }

    // Check for empty or trivial graph
    if g.get_vertices().len() <= 1 {
        return Ok(g);
    }

    println!("{}", g.edges.len());

    // vector to collect all edge values
    let mut edges: Vec<Edge> = Vec::new();

    //
    let mut added_edges: Vec<Edge> = Vec::new();

    // fill the vector with edges in graph
    for (_, edge) in &g.edges {
        edges.push(edge.clone());
    }

    edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

    // set to keep track of visited nodes
    let mut visited = HashSet::new();

    // Use Disjoint set for union find algorithm
    let mut set = DisjointSet::new();

    // Add all the vertices to the disjoint set
    for (node, _) in &g.vertices {
        set.set_insert(node.clone());
    }

    //Minimum spanning graph initialization
    let mut mst = Graph::new(true);

    // Add the first vertex to the visited set
    let first_vertex = g.vertices.keys().next().unwrap().clone();
    visited.insert(first_vertex.clone());

    let edges1 = edges.clone();
    for (vertex, _) in &g.vertices {
        //
        for (endpoint, edge) in &g.edges {
            if endpoint.0 == vertex.clone() {
                added_edges.push(edge.clone());
            }
        }
        for edge in &edges1 {
            let u = edge.endpoints.0.clone(); // get the first vertex of the edge
            let v = edge.endpoints.1.clone();

            // Skip this edge if both endpoints are already visited
            if visited.contains(&u) && visited.contains(&v) {
                continue;
            }

            // get the second vertex of the edge
            set.find(&u); // Find parent of u
                          // check if they are in different sets
            if set.find(&u) != set.find(&v) {
                // If they are in different sets then we join them using union and also use the edge in MST
                mst.add_vertex(u.clone()); // add vertex u to mst
                mst.add_vertex(v.clone()); // add vertex v to mst
                mst.add_edge((u.clone(), v.clone()), edge.weight.clone());
                added_edges.push(edge.clone());
                set.union(&u, &v);
            }
        }
    }

    let mut remaining_edges: Vec<Edge> = Vec::new();
    for iter in added_edges {
        if edges.contains(&iter) {
            continue;
        } else {
            remaining_edges.push(iter);
        }
    }

    remaining_edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

    for in_between in remaining_edges {
        let u = in_between.endpoints.0.clone(); // get the first vertex of the edge
        let v = in_between.endpoints.1.clone();
        if set.find(&u) != set.find(&v) {
            // If they are in different sets then we join them using union and also use the edge in MST
            mst.add_vertex(u.clone()); // add vertex u to mst
            mst.add_vertex(v.clone()); // add vertex v to mst
            mst.add_edge((u.clone(), v.clone()), in_between.weight.clone());
            set.union(&u, &v);
        }
    }

    // check if MST is successfull
    if mst.edges.len() != mst.vertices.len() - 1 {
        return Err(String::from(
            "MST doesn't exist for this graph since it is not connected",
        ));
    }

    println!("\nMST generated using Boruvka's algorithm: \n");

    for (_, edge) in &mst.edges {
        println!(
            "({}) -------[{}]------- ({})",
            edge.endpoints.0.clone(),
            edge.weight,
            edge.endpoints.1.clone()
        );
    }

    println!("");

    Ok(mst)
}

/// Reverse Delete Algorithm - Generate MST for any graph using the Reverse Delete Algorithm
///
/// # Parameters:
///
/// 1. g - the graph that needs to be converted to MST. This will be of type `Graph`
///
/// # Return Value:
///
/// This function returns a result, which will be either a Graph - the MST that was generated using the algo or a `Error<String>` in case of any error.
///
/// The common errors would be - if graph is directed or if MST cannot be generated for the given graph
///
/// # Example Usage:
///
/// ```
/// use graphalgos::algos;
/// use graphalgos::graphs;

/// let mut g: graphs::Graph = graphs::Graph::new(false); /// creates an undirected graph

/// // Add vertices

/// g.add_vertex(String::from("A")); // add vertex A
/// g.add_vertex(String::from("B")); // add vertex B

/// g.add_vertex(String::from("I")); // add vertex I

/// // Add edges

/// // Add multiple edges
/// g.add_edge(
///     (String::from("A"), String::from('B')),
///     graphs::GNumber::I32(4),
/// );

/// let mst = algos::reverse_delete(g); // get the mst using reverse delete algorithm

/// // reverse delete returns results, so use match statement to process it
/// match mst {
///     Ok(g) => g.print(),          // print the MST if generated successfully
///     Err(e) => println!("{}", e), // print the error if any
/// }
/// ```
///
pub fn reverse_delete(mut g: Graph) -> Result<Graph, String> {
    // Reverse delete only works for undirected graphs.
    let _is_directed = match g.edge_type {
        EdgeType::Directed => {
            return Err(String::from(
                "Reverse delete only work on undirected graphs!",
            ))
        }
        EdgeType::Undirected => {}
    };

    // Check for empty or trivial graph
    if g.get_vertices().len() <= 1 {
        return Ok(g);
    }

    // Check for connected graph
    //TODO: Consider removing this check for speed and instead check that resulting MST is connected.
    let start_vertex_lbl = g.get_vertices().keys().next().unwrap().clone(); //Get an arbitrary start vertex.
    if !dfs(&mut g, start_vertex_lbl).values().all(|&x| x) {
        return Err(String::from("Graph is not connected."));
    }

    // vector to collect all edge values
    let mut edges: Vec<Edge> = Vec::new();

    // fill the vector with edges in graph
    for (_, edge) in g.get_edges().iter() {
        edges.push(edge.clone());
    }

    edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
    edges.reverse(); //Instead of reversing here, could use a reverse iterator. Not sure which is faster.

    // iterate over edges - largest to smallest weight
    for edge in edges.iter() {
        let w = g.get_edges().get(&edge.endpoints).unwrap().weight.clone(); //TODO: This isn't pretty. Better is to have remove_edge return the edge that was removed.
        g.remove_edge(edge.endpoints.clone());
        let start_vertex_lbl = g.get_vertices().keys().next().unwrap().clone();
        if !dfs(&mut g, start_vertex_lbl).values().all(|&x| x) {
            g.add_edge(edge.endpoints.clone(), w);
        }
    }

    println!("\nMST: \n");
    for (_, edge) in &g.edges {
        println!(
            "({}) -------[{}]------- ({})",
            edge.endpoints.0.clone(),
            edge.weight,
            edge.endpoints.1.clone()
        );
    }

    Ok(g)
}

/// Prim's Algorithm - Generate MST for any graph using the Prim's Algorithm
///
/// # Parameters:
///
/// 1. g - the graph that needs to be converted to MST. This will be of type `Graph`
///
/// # Return Value:
///
/// This function returns a result, which will be either a Graph - the MST that was generated using the algo or a `Error<String>` in case of any error.
///
/// The common errors would be - if graph is directed or if MST cannot be generated for the given graph
///
/// # Example Usage:
///
/// ```
/// use graphalgos::algos;
/// use graphalgos::graphs;

/// let mut g: graphs::Graph = graphs::Graph::new(false); /// creates an undirected graph

/// // Add vertices

/// g.add_vertex(String::from("A")); // add vertex A
/// g.add_vertex(String::from("B")); // add vertex B

/// g.add_vertex(String::from("I")); // add vertex I

/// // Add edges

/// // Add multiple edges
/// g.add_edge(
///     (String::from("A"), String::from('B')),
///     graphs::GNumber::I32(4),
/// );

/// let mst = algos::boruvka(g); // get the mst using prims algorithm

/// // prims returns results, so use match statement to process it
/// match mst {
///     Ok(g) => g.print(),          // print the MST if generated successfully
///     Err(e) => println!("{}", e), // print the error if any
/// }
/// ```
///
pub fn prims(mut g: Graph) -> Result<Graph, String> {
    // check if graph has directed edges - Prims works on undirected graph and not directed
    let is_directed = match g.edge_type {
        EdgeType::Directed => true,
        EdgeType::Undirected => false,
    };

    // return error if the graph has directed edges
    if is_directed {
        return Err(String::from(
            "Prims only works properly on undirected graphs!",
        ));
    }
    // Check for empty or trivial graph
    if g.get_vertices().len() <= 1 {
        return Ok(g);
    }

    // Check for connected graph
    //TODO: Consider removing this check for speed and instead check that resulting MST is connected.
    let start_vertex_lbl = g.get_vertices().keys().next().unwrap().clone(); //Get an arbitrary start vertex.
    if !dfs(&mut g, start_vertex_lbl).values().all(|&x| x) {
        return Err(String::from("Graph is not connected."));
    }

    // vector to collect all edge values
    let mut edges: Vec<Edge> = Vec::new();

    // fill the vector with edges in graph
    for (_, edge) in &g.edges {
        edges.push(edge.clone());
    }

    // The graph that we are going to return
    let mut mst = Graph::new(false);

    // set to keep track of visited nodes
    let mut visited = HashSet::new();

    // Use a priority queue to keep track of the minimum edge at each step
    let mut pq = BinaryHeap::new();

    // Add the first vertex to the visited set
    let first_vertex = g.vertices.keys().next().unwrap().clone();
    visited.insert(first_vertex.clone());

    // Add all edges from the first vertex to the priority queue
    for (endpoint, edge) in &g.edges {
        if endpoint.0 == first_vertex || endpoint.1 == first_vertex {
            pq.push(Reverse(edge.clone()));
        }
    }

    // Iterate until we have visited all vertices
    while visited.len() != g.vertices.len() {
        // Get the minimum edge from the priority queue
        let Reverse(edge) = pq.pop().unwrap();

        // Get the two endpoints of the edge
        let u = edge.endpoints.0.clone();
        let v = edge.endpoints.1.clone();

        // Skip this edge if both endpoints are already visited
        if visited.contains(&u) && visited.contains(&v) {
            continue;
        }

        // Add the vertices and edge to the MST
        mst.add_vertex(u.clone());
        mst.add_vertex(v.clone());
        mst.add_edge(
            (u.clone(), v.clone()),
            edge.weight.clone(),
            //graphs::EdgeType::Undirected,
        );

        // Add the endpoint that is not visited to the visited set
        if visited.contains(&u) {
            visited.insert(v.clone());
        } else {
            visited.insert(u.clone());
        }

        // Add all edges from the new visited vertex to the priority queue
        for (endpoint, edge) in &g.edges {
            if visited.contains(&endpoint.0) && !visited.contains(&endpoint.1)
                || visited.contains(&endpoint.1) && !visited.contains(&endpoint.0)
            {
                pq.push(Reverse(edge.clone()));
            }
        }
    }

    println!("\nMST: \n");

    for (_, edge) in &mst.edges {
        println!(
            "({}) -------{}------- ({})",
            edge.endpoints.0.clone(),
            edge.weight,
            edge.endpoints.1.clone()
        );
    }

    println!("");

    Ok(mst)
}

/// Tests
#[cfg(test)]
mod algos_tests {
    use super::*;
    fn get_test_graph_1(directed: bool) -> Graph {
        // Generate a connected undirected graph.
        let mut g: Graph = Graph::new(directed);
        g.add_vertex(String::from("A"));
        g.add_vertex(String::from("B"));
        g.add_vertex(String::from("C"));
        g.add_vertex(String::from("D"));
        g.add_vertex(String::from("E"));
        g.add_vertex(String::from("F"));
        g.add_vertex(String::from("G"));
        g.add_vertex(String::from("H"));
        g.add_vertex(String::from("I"));

        // Integers - i32
        g.add_edge((String::from("A"), String::from('B')), GNumber::I32(4));
        g.add_edge((String::from("B"), String::from('C')), GNumber::I32(8));
        g.add_edge((String::from("C"), String::from('D')), GNumber::I32(7));
        g.add_edge((String::from("D"), String::from('E')), GNumber::I32(10));
        g.add_edge((String::from("E"), String::from('F')), GNumber::I32(11));
        g.add_edge((String::from("F"), String::from('G')), GNumber::I32(2));
        g.add_edge((String::from("G"), String::from('H')), GNumber::I32(1));
        g.add_edge((String::from("H"), String::from('I')), GNumber::I32(7));
        g.add_edge((String::from("H"), String::from('A')), GNumber::I32(9));
        g.add_edge((String::from("B"), String::from('H')), GNumber::I32(12));
        g.add_edge((String::from("C"), String::from('I')), GNumber::I32(2));
        g.add_edge((String::from("C"), String::from('F')), GNumber::I32(4));
        g.add_edge((String::from("D"), String::from('F')), GNumber::I32(14));
        g.add_edge((String::from("G"), String::from('I')), GNumber::I32(6));
        g
    }

    fn get_test_graph_2(directed: bool) -> Graph {
        //Generates a graph with 2 connected components.
        let mut g = get_test_graph_1(directed);
        g.remove_vertex(String::from("I"));
        g.remove_edge((String::from("B"), String::from('C')));
        g.remove_edge((String::from("F"), String::from('G')));
        g
    }

    fn get_mst_of_graph_1() -> Graph {
        //Generate solution to test graph 1.
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
        g.add_edge((String::from("A"), String::from('B')), GNumber::I32(4));
        g.add_edge((String::from("B"), String::from('C')), GNumber::I32(8));
        g.add_edge((String::from("C"), String::from('D')), GNumber::I32(7));
        g.add_edge((String::from("D"), String::from('E')), GNumber::I32(10));
        g.add_edge((String::from("F"), String::from('G')), GNumber::I32(2));
        g.add_edge((String::from("G"), String::from('H')), GNumber::I32(1));
        g.add_edge((String::from("C"), String::from('I')), GNumber::I32(2));
        g.add_edge((String::from("C"), String::from('F')), GNumber::I32(4));
        g
    }

    //Test depth-first search.
    #[test]
    fn test_dfs_on_connected() {
        let mut g = get_test_graph_1(false);
        let res = dfs(&mut g, String::from("A"));
        assert!(res.values().all(|&x| x));
        println!("dfs result: {:?}", res);
    }

    #[test]
    fn test_dfs_on_disconnected() {
        let mut g = get_test_graph_2(false);
        let res = dfs(&mut g, String::from("A"));
        assert!(res.get(&String::from("G")).unwrap());
        assert!(!res.get(&String::from("E")).unwrap());
    }

    //Test reverse delete algorithm.
    #[test]
    fn test_reverse_delete_on_directed() {
        let g = get_test_graph_1(true);
        //TODO: Figure out how to check assertion error.
        assert!(reverse_delete(g).is_err());
        //assert_eq!(reverse_delete(G).unwrap_err(), "Reverse delete only work on undirected graphs!");
    }

    #[test]
    fn test_reverse_delete_on_empty() {
        let g: Graph = Graph::new(false);
        //TODO: Come up with a better check.
        assert_eq!(reverse_delete(g).unwrap().get_vertices().len(), 0);
    }

    #[test]
    fn test_reverse_delete_on_trivial() {
        let mut g: Graph = Graph::new(false);
        g.add_vertex(String::from("Banana"));
        //TODO: Come up with a better check.
        assert_eq!(reverse_delete(g).unwrap().get_vertices().len(), 1);
    }

    #[test]
    fn test_reverse_delete_disconnected() {
        let g = get_test_graph_2(false);
        assert!(reverse_delete(g).is_err());
    }

    #[test]
    fn test_reverse_delete_on_non_trivial() {
        let g = get_test_graph_1(false);
        let mut mst = reverse_delete(g).unwrap();
        let mut solution = get_mst_of_graph_1();
        println!("{:?}", mst.get_edges().keys());
        println!("{:?}", solution.get_edges().keys());
        assert!(mst
            .get_edges()
            .keys()
            .all(|x| solution.get_edges().contains_key(x)));
    }

    //Test boruvka's algorithm.
    #[test]
    fn test_boruvka_on_directed() {
        let g = get_test_graph_1(true);
        //TODO: Figure out how to check assertion error.
        assert!(boruvka(g).is_err());
        //assert_eq!(reverse_delete(G).unwrap_err(), "Boruvka only work on undirected graphs!");
    }

    #[test]
    fn test_boruvka_on_empty() {
        let g: Graph = Graph::new(false);
        //TODO: Come up with a better check.
        assert_eq!(boruvka(g).unwrap().get_vertices().len(), 0);
    }

    #[test]
    fn test_boruvka_on_trivial() {
        let mut g: Graph = Graph::new(false);
        g.add_vertex(String::from("Banana"));
        //TODO: Come up with a better check.
        assert_eq!(boruvka(g).unwrap().get_vertices().len(), 1);
    }

    #[test]
    fn test_boruvka_disconnected() {
        let g = get_test_graph_2(false);
        assert!(boruvka(g).is_err());
    }

    #[test]
    fn test_boruvka_on_non_trivial() {
        let g = get_test_graph_1(false);
        let mut mst = boruvka(g).unwrap();
        let mut solution = get_mst_of_graph_1();
        println!("{:?}", mst.get_edges().keys());
        println!("{:?}", solution.get_edges().keys());
        assert!(mst
            .get_edges()
            .keys()
            .all(|y| solution.get_edges().contains_key(y)));
    }

    //Test Kruskal's algorithm.
    #[test]
    fn test_kruskals_on_directed() {
        let g = get_test_graph_1(true);
        //TODO: Figure out how to check assertion error.
        assert!(kruskals(g).is_err());
        //assert_eq!(reverse_delete(G).unwrap_err(), "Boruvka only work on undirected graphs!");
    }

    #[test]
    fn test_kruskals_on_empty() {
        let g: Graph = Graph::new(false);
        //TODO: Come up with a better check.
        assert_eq!(kruskals(g).unwrap().get_vertices().len(), 0);
    }

    #[test]
    fn test_kruskals_on_trivial() {
        let mut g: Graph = Graph::new(false);
        g.add_vertex(String::from("Banana"));
        //TODO: Come up with a better check.
        assert_eq!(kruskals(g).unwrap().get_vertices().len(), 1);
    }

    #[test]
    fn test_kruskals_disconnected() {
        let g = get_test_graph_2(false);
        assert!(kruskals(g).is_err());
    }

    #[test]
    fn test_kruskals_on_non_trivial() {
        let g = get_test_graph_1(false);
        let mut mst = kruskals(g).unwrap();
        let mut solution = get_mst_of_graph_1();
        println!("{:?}", mst.get_edges().keys());
        println!("{:?}", solution.get_edges().keys());
        assert!(mst
            .get_edges()
            .keys()
            .all(|y| solution.get_edges().contains_key(y)));
    }
    // Test Prim's algorithm on an empty graph
    #[test]
    fn test_prims_on_empty() {
        let g: Graph = Graph::new(false);
        assert_eq!(prims(g).unwrap().get_vertices().len(), 0);
    }

    // Test Prim's algorithm on a trivial graph
    #[test]
    fn test_prims_on_trivial() {
        let mut g: Graph = Graph::new(false);
        g.add_vertex(String::from("Apple"));
        assert_eq!(prims(g).unwrap().get_vertices().len(), 1);
    }

    // Test Prim's algorithm on a disconnected graph
    #[test]
    fn test_prims_disconnected() {
        let g = get_test_graph_2(false);
        assert!(prims(g).is_err());
    }

    // Test Prim's algorithm on a non-trivial graph
    #[test]
    fn test_prims_on_non_trivial() {
        let g = get_test_graph_1(false);
        let mut mst = prims(g).unwrap();
        let mut solution = get_mst_of_graph_1();
        assert!(mst
            .get_edges()
            .keys()
            .all(|y| solution.get_edges().contains_key(y)));
    }
}
