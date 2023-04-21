use crate::graphs;
use crate::graphs::Edge;
use crate::graphs::EdgeType;

use super::graphs::Graph;
use super::graphs::Vertex;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::f32::INFINITY;
use std::fmt::Debug;
use std::fmt::Display;

use super::util::DisjointSet;
use std::collections::BinaryHeap;

type VLT = String; //vertex label type

const INF: f64 = f64::INFINITY;

type TMPV = f64; //Should be V, but I'm being specific so I can debug.
pub fn dijkstra<E>(mut g: Graph<TMPV>, start_vertex: VLT)
where
    E: Clone + Debug,
{
    println!("Beginning Dikstra's algorithm.");

    //let prev: HashMap<Vertex<TMPV>, Option<Vertex<TMPV>>> = HashMap::new();
    let mut prev: HashMap<VLT, Option<VLT>> = HashMap::new();

    for (lbl, vertex) in g.get_vertices().iter_mut() {
        //Initialize all vertex values to Inf.
        //We will interpret this value as the distance to the vertex.
        (*vertex).set_value(INF);

        //Initialize previous to none.
        prev.insert(lbl.clone(), None);
    }

    //Initialize distance to start as 0.
    //(*g.get_vertices().get_mut(&start_vertex).unwrap()).set_value(0.0);
    g.get_vertex(&start_vertex).unwrap().set_value(0.0);

    //Can maybe convert to binary heap so we have ordering.
    //let heap: BinaryHeap<_> = g.get_vertices().values().collect();

    //g.get_vertices().iter();
    //let num = (*vertex).get_value();
    //println!("{}", (*vertex).get_value());
    //(*vertex).set_value(44);
    //println!("{}", (*vertex).get_value());
}

//TODO: Test this function
// pub fn dfs<V: Clone + Debug, E: Clone + Debug>(
pub fn dfs<V: Clone + Debug>(G: &mut Graph<V>, start_vertex: VLT) -> HashMap<VLT, bool> {
    let mut stack: VecDeque<Vertex<V>> = VecDeque::new();
    let mut visited: HashMap<VLT, bool> = HashMap::new();
    for (lbl, _) in G.get_vertices().iter() {
        visited.insert((*lbl).clone(), false);
    }
    stack.push_front(G.get_vertex(&start_vertex).unwrap().clone());
    while !stack.is_empty() {
        let V = stack.pop_front().unwrap();
        if !visited.get(&V.label).unwrap() {
            visited.insert(V.label.clone(), true);
            for neighbor in G.get_neighbors(&V.label).iter() {
                stack.push_front((*G.get_vertex(neighbor).unwrap()).clone());
            }
        }
    }
    visited
}

pub fn bellman_ford<V, E>(mut g: Graph<V>, start_vertex: VLT)
where
    E: Clone,
    V: Clone,
{
    println!("Beginning the Bellman-Ford algorithm.");
}

pub fn kruskals<V>(g: Graph<V>) -> Result<Graph<V>, String>
where
    // E: Clone + std::cmp::PartialOrd + Display + Debug, // E will have int or float values so we need to mark the Ord to compare them
    V: Clone + Debug,
{
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

    println!("{}", g.edges.len());

    // vector to collect all edge values
    let mut edges: Vec<Edge> = Vec::new();

    // fill the vector with edges in graph
    for (_, edge) in &g.edges {
        edges.push(edge.clone());
    }

    edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

    // println!("Edges in Sorted Order: \n");
    // let mut count = 0;
    // for i in &edges {
    //     println!("{count}.: {}", i.weight);
    //     count += 1;
    // }

    // The graph that we are going to return
    let mut mst = graphs::Graph::new(false);

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
            mst.add_vertex(u.clone(), g.vertices.get(&u).unwrap().value.clone()); // add vertex u to mst
            mst.add_vertex(v.clone(), g.vertices.get(&v).unwrap().value.clone()); // add vertex v to mst
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

pub fn boruvka<V>(mut g: Graph<V>) -> Result<Graph<V>, String>
where
    // E: Clone + std::cmp::PartialOrd + Display + Debug, // E will have int or float values so we need to mark the Ord to compare them
    V: Clone + Debug,
{
    // check if graph has directed edges - Kruskals work on undirected graph and not directed
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
    let mut mst = graphs::Graph::new(true);

    // Add the first vertex to the visited set
    let first_vertex = g.vertices.keys().next().unwrap().clone();
    visited.insert(first_vertex.clone());

    let edges1 = edges.clone();
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
               mst.add_vertex(u.clone(), g.vertices.get(&u).unwrap().value.clone()); // add vertex u to mst
               mst.add_vertex(v.clone(), g.vertices.get(&v).unwrap().value.clone()); // add vertex v to mst
               mst.add_edge((u.clone(), v.clone()), edge.weight.clone());
               added_edges.push(edge.clone());
               set.union(&u, &v);
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
            mst.add_vertex(u.clone(), g.vertices.get(&u).unwrap().value.clone()); // add vertex u to mst
            mst.add_vertex(v.clone(), g.vertices.get(&v).unwrap().value.clone()); // add vertex v to mst
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

pub fn reverse_delete<V>(mut G: Graph<V>) -> Result<Graph<V>, String>
where
    // E: Clone + std::cmp::PartialOrd + Display + Debug, // E will have int or float values so we need to mark the Ord to compare them
    V: Clone + Debug,
{
    // Reverse delete only works for undirected graphs.
    let is_directed = match G.edge_type {
        EdgeType::Directed => {
            return Err(String::from(
                "Reverse delete only work on undirected graphs!",
            ))
        }
        EdgeType::Undirected => {}
    };

    // Check for empty or trivial graph
    if G.get_vertices().len() <= 1 {
        return Ok(G);
    }

    // Check for connected graph
    //TODO: Consider removing this check for speed and instead check that resulting MST is connected.
    let start_vertex_lbl = G.get_vertices().keys().next().unwrap().clone(); //Get an arbitrary start vertex.
    if !dfs(&mut G, start_vertex_lbl).values().all(|&x| x) {
        return Err(String::from("Graph is not connected."));
    }

    // vector to collect all edge values
    let mut edges: Vec<Edge> = Vec::new();

    // fill the vector with edges in graph
    for (_, edge) in G.get_edges().iter() {
        edges.push(edge.clone());
    }

    edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
    edges.reverse(); //Instead of reversing here, could use a reverse iterator. Not sure which is faster.

    // iterate over edges - largest to smallest weight
    for edge in edges.iter() {
        let w = G.get_edges().get(&edge.endpoints).unwrap().weight.clone(); //TODO: This isn't pretty. Better is to have remove_edge return the edge that was removed.
        G.remove_edge(edge.endpoints.clone());
        let start_vertex_lbl = G.get_vertices().keys().next().unwrap().clone();
        if !dfs(&mut G, start_vertex_lbl).values().all(|&x| x) {
            G.add_edge(edge.endpoints.clone(), w);
        }
    }

    println!("\nMST: \n");
    for (_, edge) in &G.edges {
        println!(
            "({}) -------[{}]------- ({})",
            edge.endpoints.0.clone(),
            edge.weight,
            edge.endpoints.1.clone()
        );
    }

    Ok(G)
}

// -- Uncomment later --

pub fn prims<V>(mut g: Graph<V>) -> Result<Graph<V>, String>
where
    // E: Clone + std::cmp::Ord + Display + Debug, // E will have int or float values so we need to mark the Ord to compare them
    V: Clone + Debug,
{
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

    // vector to collect all edge values
    let mut edges: Vec<Edge> = Vec::new();

    // fill the vector with edges in graph
    for (_, edge) in &g.edges {
        edges.push(edge.clone());
    }

    // The graph that we are going to return
    let mut mst = graphs::Graph::new(false);

    // set to keep track of visited nodes
    let mut visited = HashSet::new();

    // Use a priority queue to keep track of the minimum edge at each step
    let mut pq = BinaryHeap::new();

    // Add the first vertex to the visited set
    let first_vertex = g.vertices.keys().next().unwrap().clone();
    visited.insert(first_vertex.clone());

    // Add all edges from the first vertex to the priority queue
    for (endpoint, edge) in &g.edges {
        if endpoint.0 == first_vertex {
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
        mst.add_vertex(u.clone(), g.vertices.get(&u).unwrap().value.clone());
        mst.add_vertex(v.clone(), g.vertices.get(&v).unwrap().value.clone());
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
            if visited.contains(&endpoint.0) && !visited.contains(&endpoint.1) {
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

// -- Uncomment later --

#[cfg(test)]
mod algos_tests {
    use super::*;
    fn get_test_graph_1(directed: bool) -> Graph<i32> {
        // Generate a connected undirected graph.
        let mut G: Graph<i32> = Graph::new(directed);
        G.add_vertex(String::from("A"), 0);
        G.add_vertex(String::from("B"), 1);
        G.add_vertex(String::from("C"), 2);
        G.add_vertex(String::from("D"), 3);
        G.add_vertex(String::from("E"), 4);
        G.add_vertex(String::from("F"), 5);
        G.add_vertex(String::from("G"), 6);
        G.add_vertex(String::from("H"), 7);
        G.add_vertex(String::from("I"), 8);

        // Integers - i32
        G.add_edge(
            (String::from("A"), String::from('B')),
            graphs::Number::I32(4),
        );
        G.add_edge(
            (String::from("B"), String::from('C')),
            graphs::Number::I32(8),
        );
        G.add_edge(
            (String::from("C"), String::from('D')),
            graphs::Number::I32(7),
        );
        G.add_edge(
            (String::from("D"), String::from('E')),
            graphs::Number::I32(10),
        );
        G.add_edge(
            (String::from("E"), String::from('F')),
            graphs::Number::I32(11),
        );
        G.add_edge(
            (String::from("F"), String::from('G')),
            graphs::Number::I32(2),
        );
        G.add_edge(
            (String::from("G"), String::from('H')),
            graphs::Number::I32(1),
        );
        G.add_edge(
            (String::from("H"), String::from('I')),
            graphs::Number::I32(7),
        );
        G.add_edge(
            (String::from("H"), String::from('A')),
            graphs::Number::I32(9),
        );
        G.add_edge(
            (String::from("B"), String::from('H')),
            graphs::Number::I32(12),
        );
        G.add_edge(
            (String::from("C"), String::from('I')),
            graphs::Number::I32(2),
        );
        G.add_edge(
            (String::from("C"), String::from('F')),
            graphs::Number::I32(4),
        );
        G.add_edge(
            (String::from("D"), String::from('F')),
            graphs::Number::I32(14),
        );
        G.add_edge(
            (String::from("G"), String::from('I')),
            graphs::Number::I32(6),
        );
        G
    }

    fn get_test_graph_2(directed: bool) -> Graph<i32> {
        //Generates a graph with 2 connected components.
        let mut G = get_test_graph_1(directed);
        G.remove_vertex(String::from("I"));
        G.remove_edge((String::from("B"), String::from('C')));
        G.remove_edge((String::from("F"), String::from('G')));
        G
    }

    fn get_mst_of_graph_1() -> Graph<i32> {
        //Generate solution to test graph 1.
        let mut G: Graph<i32> = Graph::new(false);
        G.add_vertex(String::from("A"), 0);
        G.add_vertex(String::from("B"), 1);
        G.add_vertex(String::from("C"), 2);
        G.add_vertex(String::from("D"), 3);
        G.add_vertex(String::from("E"), 4);
        G.add_vertex(String::from("F"), 5);
        G.add_vertex(String::from("G"), 6);
        G.add_vertex(String::from("H"), 7);
        G.add_vertex(String::from("I"), 8);
        G.add_edge(
            (String::from("A"), String::from('B')),
            graphs::Number::I32(4),
        );
        G.add_edge(
            (String::from("B"), String::from('C')),
            graphs::Number::I32(8),
        );
        G.add_edge(
            (String::from("C"), String::from('D')),
            graphs::Number::I32(7),
        );
        G.add_edge(
            (String::from("D"), String::from('E')),
            graphs::Number::I32(10),
        );
        G.add_edge(
            (String::from("F"), String::from('G')),
            graphs::Number::I32(2),
        );
        G.add_edge(
            (String::from("G"), String::from('H')),
            graphs::Number::I32(1),
        );
        G.add_edge(
            (String::from("C"), String::from('I')),
            graphs::Number::I32(2),
        );
        G.add_edge(
            (String::from("C"), String::from('F')),
            graphs::Number::I32(4),
        );
        G
    }

    //Test depth-first search.
    #[test]
    fn test_dfs_on_connected() {
        let mut G = get_test_graph_1(false);
        let res = dfs(&mut G, String::from("A"));
        assert!(res.values().all(|&x| x));
        println!("dfs result: {:?}", res);
    }

    #[test]
    fn test_dfs_on_disconnected() {
        let mut G = get_test_graph_2(false);
        let res = dfs(&mut G, String::from("A"));
        assert!(res.get(&String::from("G")).unwrap());
        assert!(!res.get(&String::from("E")).unwrap());
    }

    //Test reverse delete algorithm.
    #[test]
    fn test_reverse_delete_on_directed() {
        let mut G = get_test_graph_1(true);
        //TODO: Figure out how to check assertion error.
        assert!(reverse_delete(G).is_err());
        //assert_eq!(reverse_delete(G).unwrap_err(), "Reverse delete only work on undirected graphs!");
    }

    #[test]
    fn test_reverse_delete_on_empty() {
        let mut G: Graph<i32> = Graph::new(false);
        //TODO: Come up with a better check.
        assert_eq!(reverse_delete(G).unwrap().get_vertices().len(), 0);
    }

    #[test]
    fn test_reverse_delete_on_trivial() {
        let mut G: Graph<i32> = Graph::new(false);
        G.add_vertex(String::from("Banana"), 0);
        //TODO: Come up with a better check.
        assert_eq!(reverse_delete(G).unwrap().get_vertices().len(), 1);
    }

    #[test]
    fn test_reverse_delete_disconnected() {
        let mut G = get_test_graph_2(false);
        assert!(reverse_delete(G).is_err());
    }

    #[test]
    fn test_reverse_delete_on_non_trivial() {
        let mut G = get_test_graph_1(false);
        let mut mst = reverse_delete(G).unwrap();
        let mut solution = get_mst_of_graph_1();
        println!("{:?}", mst.get_edges().keys());
        println!("{:?}", solution.get_edges().keys());
        assert!(mst
            .get_edges()
            .keys()
            .all(|x| solution.get_edges().contains_key(x)));
    }

    #[test]
    fn test_boruvka_on_directed() {
        let mut G = get_test_graph_1(true);
        //TODO: Figure out how to check assertion error.
        assert!(boruvka(G).is_err());
        //assert_eq!(reverse_delete(G).unwrap_err(), "Boruvka only work on undirected graphs!");
    }

    #[test]
    fn test_boruvka_on_empty() {
        let mut G: Graph<i32> = Graph::new(false);
        //TODO: Come up with a better check.
        assert_eq!(boruvka(G).unwrap().get_vertices().len(), 0);
    }

    #[test]
    fn test_boruvka_on_trivial() {
        let mut G: Graph<i32> = Graph::new(false);
        G.add_vertex(String::from("Banana"), 0);
        //TODO: Come up with a better check.
        assert_eq!(boruvka(G).unwrap().get_vertices().len(), 1);
    }

    #[test]
    fn test_boruvka_disconnected() {
        let mut G = get_test_graph_2(false);
        assert!(boruvka(G).is_err());
    }

    #[test]
    fn test_boruvka_on_non_trivial() {
        let mut G = get_test_graph_1(false);
        let mut mst = boruvka(G).unwrap();
        let mut solution = get_mst_of_graph_1();
        println!("{:?}", mst.get_edges().keys());
        println!("{:?}", solution.get_edges().keys());
        assert!(mst
            .get_edges()
            .keys()
            .all(|y| solution.get_edges().contains_key(y)));
    }
}
