mod graphs;

fn main() {
    let mut g: graphs::Graph<i32, i32> = graphs::Graph::new();
    g.add_vertex(String::from("A"), 0);
    g.add_vertex(String::from("B"), 1);
    g.add_vertex(String::from("B"), 0);
    g.add_vertex(String::from("A"), 0);
    g.add_vertex(String::from("b"), 0);
    
    g.add_edge((String::from("b"), String::from('A')), 4);
    g.add_edge((String::from("b"), String::from('A')), 5);
    g.add_edge((String::from("b"), String::from('A')), 5);
}
