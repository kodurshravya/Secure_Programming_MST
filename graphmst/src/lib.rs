// use crate::algos;
// use crate::graphs::Graph;

pub mod algos;
pub mod graphs;
pub mod util;

// use crate::graphs::Graph;

/// Test cases
#[cfg(test)]
mod graph_tests {
    use crate::{
        algos::*,
        graphs::{self, *},
    };

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
        g.add_edge(
            (String::from("A"), String::from('B')),
            graphs::GNumber::I32(4),
        );
        g.add_edge(
            (String::from("B"), String::from('C')),
            graphs::GNumber::I32(8),
        );
        g.add_edge(
            (String::from("C"), String::from('D')),
            graphs::GNumber::I32(7),
        );
        g.add_edge(
            (String::from("D"), String::from('E')),
            graphs::GNumber::I32(10),
        );
        g.add_edge(
            (String::from("E"), String::from('F')),
            graphs::GNumber::I32(11),
        );
        g.add_edge(
            (String::from("F"), String::from('G')),
            graphs::GNumber::I32(2),
        );
        g.add_edge(
            (String::from("G"), String::from('H')),
            graphs::GNumber::I32(1),
        );
        g.add_edge(
            (String::from("H"), String::from('I')),
            graphs::GNumber::I32(7),
        );
        g.add_edge(
            (String::from("H"), String::from('A')),
            graphs::GNumber::I32(9),
        );
        g.add_edge(
            (String::from("B"), String::from('H')),
            graphs::GNumber::I32(12),
        );
        g.add_edge(
            (String::from("C"), String::from('I')),
            graphs::GNumber::I32(2),
        );
        g.add_edge(
            (String::from("C"), String::from('F')),
            graphs::GNumber::I32(4),
        );
        g.add_edge(
            (String::from("D"), String::from('F')),
            graphs::GNumber::I32(14),
        );
        g.add_edge(
            (String::from("G"), String::from('I')),
            graphs::GNumber::I32(6),
        );
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
        g.add_edge(
            (String::from("A"), String::from('B')),
            graphs::GNumber::I32(4),
        );
        g.add_edge(
            (String::from("B"), String::from('C')),
            graphs::GNumber::I32(8),
        );
        g.add_edge(
            (String::from("C"), String::from('D')),
            graphs::GNumber::I32(7),
        );
        g.add_edge(
            (String::from("D"), String::from('E')),
            graphs::GNumber::I32(10),
        );
        g.add_edge(
            (String::from("F"), String::from('G')),
            graphs::GNumber::I32(2),
        );
        g.add_edge(
            (String::from("G"), String::from('H')),
            graphs::GNumber::I32(1),
        );
        g.add_edge(
            (String::from("C"), String::from('I')),
            graphs::GNumber::I32(2),
        );
        g.add_edge(
            (String::from("C"), String::from('F')),
            graphs::GNumber::I32(4),
        );
        g
    }

    #[test]
    fn add_one_vertex() {
        let mut g: Graph = Graph::new(false);
        g.add_vertex(String::from("A"));
        assert_eq!(g.get_vertices().len(), 1);
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().label, "A");
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().get_value(), 0f64);
    }

    #[test]
    fn add_multiple_vertices() {
        let mut g = get_test_graph_1(false);
        assert_eq!(g.get_vertices().len(), 9);
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().label, "A");
        //assert_eq!(g.get_vertex(&String::from("A")).unwrap().get_value(), 0);
        assert_eq!(g.get_vertex(&String::from("C")).unwrap().label, "C");
        //assert_eq!(g.get_vertex(&String::from("C")).unwrap().get_value(), 2);
        assert_eq!(g.get_vertex(&String::from("H")).unwrap().label, "H");
        //assert_eq!(g.get_vertex(&String::from("H")).unwrap().get_value(), 7);
        assert_eq!(g.get_vertex(&String::from("H")).unwrap().label, "H");
        //assert_eq!(g.get_vertex(&String::from("H")).unwrap().get_value(), 7);
        assert_eq!(g.get_vertex(&String::from("I")).unwrap().label, "I");
        //assert_eq!(g.get_vertex(&String::from("I")).unwrap().get_value(), 8);
    }

    #[test]
    fn remove_one_vertex() {
        let mut g = get_test_graph_1(false);
        g.remove_vertex(String::from("F"));
        assert_eq!(g.get_vertices().len(), 8);
        assert_eq!(g.get_vertices().get("F").is_none(), true);
    }


    #[test]
    fn update_edge_test() {
        let mut g = get_mst_of_graph_1();
        g.update_edge((String::from("B"), String::from("C")),  graphs::GNumber::I32(-0),);
    }

    #[test]
    fn remove_multiple_vertices() {
        let mut g = get_test_graph_1(false);
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

    // #[test]
    // fn add_one_undirected_edge() {
    //     let mut g: Graph = Graph::new(false);
    //     g.add_vertex(String::from("A"));
    //     g.add_vertex(String::from("B"));
    //     g.add_edge((String::from("A"), String::from('B')), GNumber::I32(4));
    //     assert_eq!(g.get_edges().len(), 1);
    // }
    // #[test]
    // fn make_from_macro() {
    //     let mut G = gph!("A", "B");
    //     assert_eq!(G.get_vertices().len(), 2);
    // }
   
    #[test]
    fn add_topologicalorder_test() {
        let mut expected: Vec<String> = Vec::new();
        let mut G = get_test_graph_1(false);
        G.add_edge((String::from("A"), String::from('B')), GNumber::F64(4.));
        G.add_edge((String::from("B"), String::from('C')), GNumber::F64(1.));
        G.add_edge((String::from("A"), String::from('D')), GNumber::F64(1.));
        G.add_edge((String::from("S"), String::from('C')), GNumber::F64(1.));
        let answer = G.get_topological_order();
        assert_eq!(expected, answer);
    }

    #[test]
    fn add_vertex_test() {
        let mut g: Graph = Graph::new(false);
        g.add_vertex(String::from("越"));
        let vertex = g.get_vertices();
        println!("{:?}", vertex);
    }

    #[test]
    fn set_value_test() {
        let mut gh: Graph = Graph::new(true);
        gh.add_vertex(String::from("F"));
        gh.add_vertex(String::from("C"));
        gh.add_edge(
        (String::from("C"), String::from('F')),
        graphs::GNumber::I32(4), );

        for (lbl, vertex) in gh.get_vertices().iter_mut() {
            let xyz = (*vertex).set_value(1.2);
             let xyz = (*vertex).get_value();
             assert_eq!(xyz, 1.2);
        }
    }

    #[test]
    fn get_neighbours_test() {
        let mut test: Graph = Graph::new(false);
        test.add_vertex(String::from("F"));
        test.add_vertex(String::from("C"));
        test.add_edge(
        (String::from("C"), String::from('F')),
        graphs::GNumber::I32(4), );
        let no_vertex = test.get_in_neighbors(&String::from("A"));
        let no_neighbour = test.get_out_neighbors(&String::from("A"));
        let no_neighbour_variable = test.get_neighbors(&String::from("A"));
        let expected: Vec<String> = Vec::new();
        //expecting empty vec since graph don't have vertex "A"
        assert_eq!(expected , no_vertex);
        assert_eq!(expected , no_neighbour);
        assert_eq!(expected , no_neighbour_variable);
    }

    #[test]
    fn remove_multiple_edges() {
        let mut g = get_mst_of_graph_1();
        assert_eq!(g.get_edges().len(), 8);
        //removes two edges from vertex A
        g.remove_vertex(String::from("A"));
        //trying to remove non-existant edge A-B 
        g.remove_edge((String::from("A"), String::from("B")));
        assert_eq!(g.get_edges().len(), 7);
        //removing edge in wrong order C-B insted of B-C
        g.remove_edge((String::from("C"), String::from("B")));
        assert_eq!(g.get_edges().len(), 7);
        g.remove_edge((String::from("B"), String::from("C")));
        g.remove_edge((String::from("D"), String::from("E")));
        g.remove_edge((String::from("F"), String::from("G")));
        g.remove_edge((String::from("G"), String::from("H")));
        g.remove_edge((String::from("C"), String::from("I")));
        g.remove_edge((String::from("C"), String::from("F")));
        g.remove_edge((String::from("C"), String::from("D")));
        assert_eq!(g.get_edges().len(), 0);
    }

    //Test prim's algorithm.
    #[test]
    fn test_prims_on_directed() {
        let g = get_test_graph_1(true);
        //TODO: Figure out how to check assertion error.
        assert!(prims(g).is_err());
        //assert_eq!(reverse_delete(G).unwrap_err(), "Boruvka only work on undirected graphs!");
    }

    #[test]
    fn test_prims_on_trivial() {
        let mut g: Graph = Graph::new(false);
        g.add_vertex(String::from("Banana"));
        //TODO: Come up with a better check.
        assert_eq!(prims(g).unwrap().get_vertices().len(), 1);
    }

    #[test]
    fn test_prims_disconnected() {
        let g = get_test_graph_2(false);
        assert!(prims(g).is_err());
    }

    //Test boruvka's algorithm.
    #[test]
    fn test_boruvka_on_directed() {
        let g = get_test_graph_1(true);
        //TODO: Figure out how to check assertion error.
        assert!(boruvka(g).is_err());
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


}

