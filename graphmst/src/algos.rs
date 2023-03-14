use crate::graphs;
use crate::graphs::Edge;
use crate::graphs::EdgeType;

use super::graphs::Graph;
use super::graphs::Vertex;
use std::collections::HashMap;
//use std::collections::BinaryHeap;

type VLT = String; //vertex label type

const INF: f64 = f64::INFINITY;

type TMPV = f64; //Should be V, but I'm being specific so I can debug.
pub fn Dijkstra<E>(mut g: Graph<TMPV, E>, start_vertex: VLT)
where
    E: Clone,
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

pub fn BellmanFord<V, E>(mut g: Graph<V, E>, start_vertex: VLT)
where
    E: Clone,
    V: Clone,
{
    println!("Beginning the Bellman-Ford algorithm.");
}

pub fn Kruskals<V, E>(mut g: Graph<V, E>, num_edges: i32) -> Result<Graph<V, E>, String>
where
    E: Clone + std::cmp::Ord, // E will have int or float values so we need to mark the Ord to compare them
    V: Clone,
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

    // vector to collect all edge values
    let mut edges: Vec<Edge<E>> = Vec::new();

    // fill the vector with edges in graph
    for (_, edge) in g.edges {
        edges.push(edge.clone());
    }

    edges.sort_by(|a, b| a.weight.cmp(&b.weight));
    // edges.sort(); // Sort the edges - we need to pick the smallest edge first and so on

    let mut mst = graphs::Graph::new(true);

    Ok(mst)
}
