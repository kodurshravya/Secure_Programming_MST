use crate::graphs;
use crate::graphs::Edge;
use crate::graphs::EdgeType;

use super::graphs::Graph;
use super::graphs::Vertex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;

use super::util::DisjointSet;
//use std::collections::BinaryHeap;

type VLT = String; //vertex label type

const INF: f64 = f64::INFINITY;

type TMPV = f64; //Should be V, but I'm being specific so I can debug.
pub fn Dijkstra<E>(mut g: Graph<TMPV, E>, start_vertex: VLT)
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

pub fn bellman_ford<V, E>(mut g: Graph<V, E>, start_vertex: VLT)
where
    E: Clone,
    V: Clone,
{
    println!("Beginning the Bellman-Ford algorithm.");
}

pub fn kruskals<V, E>(mut g: Graph<V, E>) -> Result<Graph<V, E>, String>
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
                graphs::EdgeType::Undirected,
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
