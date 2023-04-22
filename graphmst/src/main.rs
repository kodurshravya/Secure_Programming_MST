// use ::graphs::Graph;
// use ::

// use crate::algos::boruvka;

// mod algos;
// mod graphs;
// mod util;

use graphalgos::{algos, graphs};

fn main() {
    //let mut g = graphs::Graph::new(false);
    //
    //g.add_vertex(String::from("A"), 0);
    //g.add_vertex(String::from("B"), 1);
    //g.add_vertex(String::from("C"), 2);
    //g.add_vertex(String::from("D"), 3);
    //g.add_vertex(String::from("E"), 4);
    //g.add_vertex(String::from("F"), 5);
    //g.add_vertex(String::from("G"), 6);
    //g.add_vertex(String::from("H"), 7);
    //g.add_vertex(String::from("I"), 8);
    //
    //// Integers - i32
    //g.add_edge(
    //    (String::from("A"), String::from('B')),
    //    4,
    //
    //);
    //g.add_edge(
    //    (String::from("B"), String::from('C')),
    //    8,
    //
    //);
    //g.add_edge(
    //    (String::from("C"), String::from('D')),
    //    7,
    //
    //);
    //g.add_edge(
    //    (String::from("D"), String::from('E')),
    //    9,
    //
    //);
    //g.add_edge(
    //    (String::from("E"), String::from('F')),
    //    10,
    //
    //);
    //g.add_edge(
    //    (String::from("F"), String::from('G')),
    //    2,
    //
    //);
    //g.add_edge(
    //    (String::from("G"), String::from('H')),
    //    1,
    //
    //);
    //g.add_edge(
    //    (String::from("H"), String::from('I')),
    //    7,
    //
    //);
    //g.add_edge(
    //    (String::from("H"), String::from('A')),
    //    8,
    //
    //);
    //g.add_edge(
    //    (String::from("B"), String::from('H')),
    //    11,
    //
    //);
    //g.add_edge(
    //    (String::from("C"), String::from('I')),
    //    2,
    //
    //);
    //g.add_edge(
    //    (String::from("C"), String::from('F')),
    //    4,
    //
    //);
    //g.add_edge(
    //    (String::from("D"), String::from('F')),
    //    14,
    //
    //);
    //g.add_edge(
    //    (String::from("G"), String::from('I')),
    //    6,
    //
    //);

    // Float values - f64

    // g.add_edge(
    //     (String::from("A"), String::from('B')),
    //     4.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("B"), String::from('C')),
    //     8.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("C"), String::from('D')),
    //     7.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("D"), String::from('E')),
    //     9.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("E"), String::from('F')),
    //     10.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("F"), String::from('G')),
    //     2.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("G"), String::from('H')),
    //     1.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("H"), String::from('I')),
    //     7.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("H"), String::from('A')),
    //     8.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("B"), String::from('H')),
    //     11.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("C"), String::from('I')),
    //     2.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("C"), String::from('F')),
    //     4.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("D"), String::from('F')),
    //     14.,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("G"), String::from('I')),
    //     6.,
    //     graphs::EdgeType::Undirected,
    // );

    // Negetive edge
    // g.add_edge(
    //     (String::from("A"), String::from('B')),
    //     -4,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("B"), String::from('C')),
    //     -8,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("C"), String::from('D')),
    //     -7,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("D"), String::from('E')),
    //     -9,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("E"), String::from('F')),
    //     1 - 0,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("F"), String::from('G')),
    //     -2,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("G"), String::from('H')),
    //     -1,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("H"), String::from('I')),
    //     -7,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("H"), String::from('A')),
    //     -8,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("B"), String::from('H')),
    //     -11,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("C"), String::from('I')),
    //     -2,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("C"), String::from('F')),
    //     -4,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("D"), String::from('F')),
    //     -14,
    //     graphs::EdgeType::Undirected,
    // );
    // g.add_edge(
    //     (String::from("G"), String::from('I')),
    //     -6,
    //     graphs::EdgeType::Undirected,
    // );

    // algos::Dijkstra(g, String::from("A"));

    //    }

    fn get_graph() -> graphs::Graph<i32> {
        //Generates a graph with 2 connected components.
        let mut G: graphs::Graph<i32> = graphs::Graph::new(false);
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
            graphs::GNumber::I32(4),
        );
        G.add_edge(
            (String::from("B"), String::from('C')),
            graphs::GNumber::I32(8),
        );
        G.add_edge(
            (String::from("C"), String::from('D')),
            graphs::GNumber::I32(7),
        );
        G.add_edge(
            (String::from("D"), String::from('E')),
            graphs::GNumber::I32(9),
        );
        G.add_edge(
            (String::from("E"), String::from('F')),
            graphs::GNumber::I32(10),
        );
        G.add_edge(
            (String::from("F"), String::from('G')),
            graphs::GNumber::I32(2),
        );
        G.add_edge(
            (String::from("G"), String::from('H')),
            graphs::GNumber::I32(1),
        );
        G.add_edge(
            (String::from("H"), String::from('I')),
            graphs::GNumber::I32(7),
        );
        G.add_edge(
            (String::from("H"), String::from('A')),
            graphs::GNumber::I32(8),
        );
        G.add_edge(
            (String::from("B"), String::from('H')),
            graphs::GNumber::I32(11),
        );
        G.add_edge(
            (String::from("C"), String::from('I')),
            graphs::GNumber::I32(2),
        );
        G.add_edge(
            (String::from("C"), String::from('F')),
            graphs::GNumber::I32(4),
        );
        G.add_edge(
            (String::from("D"), String::from('F')),
            graphs::GNumber::I32(14),
        );
        G.add_edge(
            (String::from("G"), String::from('I')),
            graphs::GNumber::I32(6),
        );
        G
    }

    let mut G = get_graph();
    println!("\n\n----boruvka START -----\n\n");
    let mst_boruvka = algos::boruvka(G);
    match mst_boruvka {
        // Ok(g) => println!("MST generated successfully!"),
        Ok(g) => g.print(),
        Err(e) => println!("{}", e),
    }
    println!("\n\n----boruvka END -----\n\n");

    let mut G = get_graph();
    println!("\n\n----KRUSKALS START -----\n\n");
    let mst_kruskals = algos::kruskals(G);
    match mst_kruskals {
        // Ok(g) => println!("MST generated successfully!"),
        Ok(g) => g.print(),
        Err(e) => println!("{}", e),
    }
    println!("\n\n----KRUSKALS END -----\n\n");

    let mut G = get_graph();
    println!("\n\n----REVERSE DELETE START -----\n\n");
    let mst_rd = algos::kruskals(G);
    match mst_rd {
        // Ok(g) => println!("MST generated successfully!"),
        Ok(g) => g.print(),
        Err(e) => println!("{}", e),
    }
    println!("\n\n----REVERSE DELETE END -----\n\n");

    let mut G = get_graph();
    println!("\n\n----PRIMS START -----\n\n");
    let mst_prims = algos::prims(G);
    match mst_prims {
        Ok(g) => g.print(),
        Err(e) => println!("{}", e),
    }
    println!("\n\n----PRIMS END -----\n\n");

    //gph!("A", "B");

    // Kruskals(g, 5);
    //algos::BellmanFord(g, String::from("A"));
}
