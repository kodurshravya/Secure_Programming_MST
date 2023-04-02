use crate::graphs;
use crate::graphs::Edge;
use crate::graphs::EdgeType;

use super::graphs::Graph;
use super::graphs::Vertex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Display;

use super::util::DisjointSet;
//use std::collections::BinaryHeap;

type VLT = String; //vertex label type

const INF: f64 = f64::INFINITY;

type TMPV = f64; //Should be V, but I'm being specific so I can debug.
pub fn dijkstra<E>(mut g: Graph<TMPV, E>, start_vertex: VLT)
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
pub fn dfs<V: Clone + Debug, E: Clone + Debug>(G: &mut Graph<V, E>, start_vertex: VLT) -> HashMap<VLT, bool> {
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

pub fn bellman_ford<V, E>(mut g: Graph<V, E>, start_vertex: VLT)
where
    E: Clone,
    V: Clone,
{
    println!("Beginning the Bellman-Ford algorithm.");
}

pub fn kruskals<V, E>(g: Graph<V, E>) -> Result<Graph<V, E>, String>
where
    E: Clone + std::cmp::PartialOrd + Display + Debug, // E will have int or float values so we need to mark the Ord to compare them
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
    let mut edges: Vec<Edge<E>> = Vec::new();

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
            mst.add_edge(
                (u.clone(), v.clone()),
                edge.weight.clone(),
            );
            set.union(&u, &v);
        }
    }

    // check if MST is successfull
    if mst.edges.len() != mst.vertices.len() - 1 {
        return Err(String::from(
            "MST doesn't exist for this graph since it is not connected",
        ));
    }

    println!("\nMST: \n");

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

pub fn boruvka<V, E>(mut g: Graph<V, E>, mut edge_weight: i32) -> Result<Graph<V, E>, String>
where
    E: Clone + std::cmp::PartialOrd + Debug, // E will have int or float values so we need to mark the Ord to compare them
    V: Clone + Debug,
{
    // vector to collect all edge values
    let mut edge_weight: Vec<Edge<E>> = Vec::new();

    // fill the vector with edges in graph
    for (_, edge) in &g.edges {
        edge_weight.push(edge.clone());
    }

    for (_, edge) in &g.edges {
        edge_weight.push(edge.clone());
    }

    let mut mst = graphs::Graph::new(true);

    loop {
        // let mut min_edge_Weight: Vec<K> = vec![K::infinite();
        // g.get_vertices_count()];
        // let mut min_edge: Vec<Option<(usize, usize)>> = vec![None; g.get_vertices_count()];
    }

    Ok(mst)
}

pub fn reverse_delete<V, E>(mut G: Graph<V, E>) -> Result<Graph<V, E>, String>
where
    E: Clone + std::cmp::PartialOrd + Display + Debug, // E will have int or float values so we need to mark the Ord to compare them
    V: Clone + Debug,
{
    // Reverse delete only works for undirected graphs.
    let is_directed = match G.edge_type {
        EdgeType::Directed => return Err(String::from(
            "Reverse delete only work on undirected graphs!",
        )),
        EdgeType::Undirected => {},
    };
    
    // Check for empty or trivial graph
    if G.get_vertices().len() <= 1 {
        return Ok(G);
    }

    // vector to collect all edge values
    let mut edges: Vec<Edge<E>> = Vec::new();

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
        let start_vertex_lbl = G.get_vertices().keys().next().unwrap().clone(); //Get an arbitrary start vertex.
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

#[cfg(test)]
mod algos_tests {
    use super::*;
    
    fn get_test_graph_1() -> Graph<i32, i32>  {
        let mut G: Graph<i32, i32> = Graph::new(false);
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
            (String::from("A"), String::from('B')), 4
        );
        G.add_edge(
            (String::from("B"), String::from('C')), 8
        );
        G.add_edge(
            (String::from("C"), String::from('D')), 7
        );
        G.add_edge(
            (String::from("D"), String::from('E')), 9
        );
        G.add_edge(
            (String::from("E"), String::from('F')), 10
        );
        G.add_edge(
            (String::from("F"), String::from('G')), 2
        );
        G.add_edge(
            (String::from("G"), String::from('H')), 1
        );
        G.add_edge(
            (String::from("H"), String::from('I')), 7
        );
        G.add_edge(
            (String::from("H"), String::from('A')), 8
        );
        G.add_edge(
            (String::from("B"), String::from('H')), 11
        );
        G.add_edge(
            (String::from("C"), String::from('I')), 2 
        );
        G.add_edge(
            (String::from("C"), String::from('F')), 4
        );
        G.add_edge(
            (String::from("D"), String::from('F')), 14
        );
        G.add_edge(
            (String::from("G"), String::from('I')), 6  
        );
        G
    }
    
    fn get_test_graph_2() -> Graph<i32, i32>  {
        //Generates a graph with 2 connected components.
        let mut G: Graph<i32, i32> = Graph::new(false);
        G.add_vertex(String::from("A"), 0);
        G.add_vertex(String::from("B"), 1);
        G.add_vertex(String::from("C"), 2);
        G.add_vertex(String::from("D"), 3);
        G.add_vertex(String::from("E"), 4);
        G.add_vertex(String::from("F"), 5);
        G.add_vertex(String::from("G"), 6);
        G.add_vertex(String::from("H"), 7);

        // Integers - i32
        G.add_edge((String::from("A"), String::from('B')), 4);
        G.add_edge((String::from("C"), String::from('D')), 7);
        G.add_edge((String::from("D"), String::from('E')), 9);
        G.add_edge((String::from("E"), String::from('F')), 10);
        G.add_edge((String::from("G"), String::from('H')), 1);
        G.add_edge((String::from("H"), String::from('I')), 7);
        G.add_edge((String::from("H"), String::from('A')), 8);
        G.add_edge((String::from("B"), String::from('H')), 11);
        G.add_edge((String::from("C"), String::from('I')), 2 );
        G.add_edge((String::from("C"), String::from('F')), 4);
        G.add_edge((String::from("D"), String::from('F')), 14);
        G.add_edge((String::from("G"), String::from('I')), 6);
        G
    }
    
    #[test]
    fn run_dfs_on_connected() {
        let mut G = get_test_graph_1();
        let res = dfs(&mut G, String::from("A"));
        assert!(res.values().all(|&x| x));
        println!("dfs result: {:?}", res);
    }
    
    #[test]
    fn run_dfs() {
        let mut G = get_test_graph_2();
        let res = dfs(&mut G, String::from("A"));
        assert!(res.get(&String::from("G")).unwrap());
        assert!(!res.get(&String::from("E")).unwrap());
    }
}