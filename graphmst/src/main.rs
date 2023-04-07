mod algos;
mod graphs;
mod util;

fn main() {
    //let mut g = graphs::Graph::new(false);
    let mut g = graphs::Graph::new(false);
    
    


    g.add_vertex(String::from("A"), 0);
    g.add_vertex(String::from("B"), 1);
    g.add_vertex(String::from("C"), 2);
    g.add_vertex(String::from("D"), 3);
    g.add_vertex(String::from("E"), 4);
    g.add_vertex(String::from("F"), 5);
    g.add_vertex(String::from("G"), 6);
    g.add_vertex(String::from("H"), 7);
    g.add_vertex(String::from("I"), 8);

    g.add_edge(
        (String::from("A"), String::from('B')),
        4,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("B"), String::from('C')),
        8,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("C"), String::from('D')),
        7,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("D"), String::from('E')),
        9,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("E"), String::from('F')),
        10,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("F"), String::from('G')),
        2,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("G"), String::from('H')),
        1,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("H"), String::from('I')),
        7,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("H"), String::from('A')),
        8,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("B"), String::from('H')),
        11,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("C"), String::from('I')),
        2,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("C"), String::from('F')),
        4,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("D"), String::from('F')),
        14,
        graphs::EdgeType::Undirected,
    );
    g.add_edge(
        (String::from("G"), String::from('I')),
        6,
        graphs::EdgeType::Undirected,
    );

    // println!("Testing remove");
    // g.remove_edge((String::from("b"), String::from('A')));
    //g.remove_edge((String::from("x"), String::from('y')));
    //g.remove_edge((String::from("b"), String::from('A')));
    //g.remove_edge((String::from("b"), String::from('A')));

    //g.remove_vertex(String::from("A"));

    // algos::Dijkstra(g, String::from("A"));
    println!("\n\n----KRUSKALS START -----\n\n");
    //let mst_kruskals = algos::Kruskals(g);
    let mst_kruskals = algos::Kruskals(&mut g);
    match mst_kruskals {
        // Ok(g) => println!("MST generated successfully!"),
        Ok(g) => g.print(),
        Err(e) => println!("{}", e),
    }
    println!("\n\n----KRUSKALS END -----\n\n");

    // Kruskals(g, 5);
    //algos::BellmanFord(g, String::from("A"));
    println!("\n\n----PRIMS START -----\n\n");
    let mst_prims = algos::Prims(g);
    //let mst_prims = algos::Prims(&mut g);
    match mst_prims {
        Ok(g) => g.print(),
        Err(e) => println!("{}", e),
    }
    println!("\n\n----PRIMS END -----\n\n");

}
