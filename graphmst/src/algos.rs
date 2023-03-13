use super::graphs::Graph;
use super::graphs::Vertex;
use std::collections::HashMap;
//use std::collections::BinaryHeap;

type VLT = String; //vertex label type

const INF: f64 = f64::INFINITY;

type TMPV = f64; //Should be V, but I'm being specific so I can debug.
pub fn Dijkstra<E>(mut g: Graph<TMPV, E>, start_vertex: VLT) {
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


pub fn BellmanFord<V, E>(mut g: Graph<V, E>, start_vertex: VLT) {
    println!("Beginning the Bellman-Ford algorithm.");
}