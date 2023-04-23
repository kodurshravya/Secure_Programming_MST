use crate::algos;
use crate::graphs;

/// Test cases
#[cfg(test)]
mod graph_tests {
    //extern crate graphs;
    //use graphs::Graph;
    use super::*;

    fn get_test_graph_1() -> Graph<f64> {
        let mut g: Graph<f64> = Graph::new(false);
        g.add_vertex(String::from("A"), 0.);
        g.add_vertex(String::from("B"), 1.);
        g.add_vertex(String::from("C"), 2.);
        g.add_vertex(String::from("D"), 3.);
        g.add_vertex(String::from("E"), 4.);
        g.add_vertex(String::from("F"), 5.);
        g.add_vertex(String::from("G"), 6.);
        g.add_vertex(String::from("H"), 7.);
        g.add_vertex(String::from("I"), 8.);
        g
    }

    #[test]
    fn add_one_vertex() {
        let mut g: Graph<f64> = Graph::new(false);
        g.add_vertex(String::from("A"), 0f64);
        assert_eq!(g.get_vertices().len(), 1);
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().label, "A");
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().get_value(), 0f64);
    }

    #[test]
    fn add_multiple_vertices() {
        let mut g = get_test_graph_1();
        assert_eq!(g.get_vertices().len(), 9);
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().label, "A");
        assert_eq!(g.get_vertex(&String::from("A")).unwrap().get_value(), 0.);
        assert_eq!(g.get_vertex(&String::from("C")).unwrap().label, "C");
        assert_eq!(g.get_vertex(&String::from("C")).unwrap().get_value(), 2.);
        assert_eq!(g.get_vertex(&String::from("H")).unwrap().label, "H");
        assert_eq!(g.get_vertex(&String::from("H")).unwrap().get_value(), 7.);
        assert_eq!(g.get_vertex(&String::from("H")).unwrap().label, "H");
        assert_eq!(g.get_vertex(&String::from("H")).unwrap().get_value(), 7.);
        assert_eq!(g.get_vertex(&String::from("I")).unwrap().label, "I");
        assert_eq!(g.get_vertex(&String::from("I")).unwrap().get_value(), 8.);
    }

    #[test]
    fn remove_one_vertex() {
        let mut g = get_test_graph_1();
        g.remove_vertex(String::from("F"));
        assert_eq!(g.get_vertices().len(), 8);
        assert_eq!(g.get_vertices().get("F").is_none(), true);
    }

    #[test]
    fn remove_multiple_vertices() {
        let mut G = get_test_graph_1();
        g.remove_vertex(String::from("I"));
        g.remove_vertex(String::from("H"));
        assert_eq!(G.get_vertices().len(), 7);
        g.remove_vertex(String::from("E"));
        assert_eq!(G.get_vertices().len(), 6);
        g.remove_vertex(String::from("A"));
        g.remove_vertex(String::from("B"));
        assert_eq!(G.get_vertices().len(), 4);
        g.remove_vertex(String::from("I"));
        assert_eq!(G.get_vertices().len(), 4);
        g.remove_vertex(String::from("G"));
        g.remove_vertex(String::from("F"));
        g.remove_vertex(String::from("D"));
        g.remove_vertex(String::from("C"));
        assert_eq!(G.get_vertices().len(), 0);
    }

    #[test]
    fn add_one_undirected_edge() {
        let mut G = get_test_graph_1();
        g.add_edge((String::from("A"), String::from('B')), GNumber::F64((4.)));
        assert_eq!(G.get_edges().len(), 1);
    }

    #[test]
    fn make_from_macro() {
        let mut G = gph!("A", "B");
        assert_eq!(G.get_vertices().len(), 2);
    }
}
