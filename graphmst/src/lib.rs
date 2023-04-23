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
        let mut G: Graph = Graph::new(directed);
        G.add_vertex(String::from("A"));
        G.add_vertex(String::from("B"));
        G.add_vertex(String::from("C"));
        G.add_vertex(String::from("D"));
        G.add_vertex(String::from("E"));
        G.add_vertex(String::from("F"));
        G.add_vertex(String::from("G"));
        G.add_vertex(String::from("H"));
        G.add_vertex(String::from("I"));

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
            graphs::GNumber::I32(10),
        );
        G.add_edge(
            (String::from("E"), String::from('F')),
            graphs::GNumber::I32(11),
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
            graphs::GNumber::I32(9),
        );
        G.add_edge(
            (String::from("B"), String::from('H')),
            graphs::GNumber::I32(12),
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

    fn get_test_graph_2(directed: bool) -> Graph {
        //Generates a graph with 2 connected components.
        let mut G = get_test_graph_1(directed);
        G.remove_vertex(String::from("I"));
        G.remove_edge((String::from("B"), String::from('C')));
        G.remove_edge((String::from("F"), String::from('G')));
        G
    }

    fn get_mst_of_graph_1() -> Graph {
        //Generate solution to test graph 1.
        let mut G: Graph = Graph::new(false);
        G.add_vertex(String::from("A"));
        G.add_vertex(String::from("B"));
        G.add_vertex(String::from("C"));
        G.add_vertex(String::from("D"));
        G.add_vertex(String::from("E"));
        G.add_vertex(String::from("F"));
        G.add_vertex(String::from("G"));
        G.add_vertex(String::from("H"));
        G.add_vertex(String::from("I"));
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
            (String::from("C"), String::from('I')),
            graphs::GNumber::I32(2),
        );
        G.add_edge(
            (String::from("C"), String::from('F')),
            graphs::GNumber::I32(4),
        );
        G
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
    fn remove_multiple_vertices() {
        let mut G = get_test_graph_1(false);
        G.remove_vertex(String::from("I"));
        G.remove_vertex(String::from("H"));
        assert_eq!(G.get_vertices().len(), 7);
        G.remove_vertex(String::from("E"));
        assert_eq!(G.get_vertices().len(), 6);
        G.remove_vertex(String::from("A"));
        G.remove_vertex(String::from("B"));
        assert_eq!(G.get_vertices().len(), 4);
        G.remove_vertex(String::from("I"));
        assert_eq!(G.get_vertices().len(), 4);
        G.remove_vertex(String::from("G"));
        G.remove_vertex(String::from("F"));
        G.remove_vertex(String::from("D"));
        G.remove_vertex(String::from("C"));
        assert_eq!(G.get_vertices().len(), 0);
    }

    #[test]
    // fn add_one_undirected_edge() {
    //     let mut g: Graph = Graph::new(false);
    //     g.add_vertex(String::from("A"));
    //     g.add_vertex(String::from("B"));
    //     g.add_edge((String::from("A"), String::from('B')), GNumber::I32(4));
    //     assert_eq!(g.get_edges().len(), 1);
    // }
    #[test]
    // fn make_from_macro() {
    //     let mut G = gph!("A", "B");
    //     assert_eq!(G.get_vertices().len(), 2);
    // }
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
            .all(|x| solution.get_edges().contains_key(x)));
    }

    //Test Kruskal's algorithm.
    #[test]
    fn test_kruskals_on_directed() {
        let mut G = get_test_graph_1(true);
        //TODO: Figure out how to check assertion error.
        assert!(kruskals(G).is_err());
        //assert_eq!(reverse_delete(G).unwrap_err(), "Boruvka only work on undirected graphs!");
    }

    #[test]
    fn test_kruskals_on_empty() {
        let mut G: Graph= Graph::new(false);
        //TODO: Come up with a better check.
        assert_eq!(kruskals(G).unwrap().get_vertices().len(), 0);
    }

    #[test]
    fn test_kruskals_on_trivial() {
        let mut G: Graph = Graph::new(false);
        G.add_vertex(String::from("Banana"));
        //TODO: Come up with a better check.
        assert_eq!(kruskals(G).unwrap().get_vertices().len(), 1);
    }

    #[test]
    fn test_kruskals_disconnected() {
        let mut G = get_test_graph_2(false);
        assert!(kruskals(G).is_err());
    }

    #[test]
    fn test_kruskals_on_non_trivial() {
        let mut G = get_test_graph_1(false);
        let mut mst = kruskals(G).unwrap();
        let mut solution = get_mst_of_graph_1();
        println!("{:?}", mst.get_edges().keys());
        println!("{:?}", solution.get_edges().keys());
        assert!(mst
            .get_edges()
            .keys()
            .all(|y| solution.get_edges().contains_key(y)));
    }
}
