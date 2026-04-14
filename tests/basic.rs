#[test]
fn test_search_by_category() {
    use std::collections::HashMap;

    let mut catalog: HashMap<String, String> = HashMap::new();
    catalog.insert("Notebook".to_string(), "Eletrônicos".to_string());
    catalog.insert("Mouse".to_string(), "Eletrônicos".to_string());
    catalog.insert("Camisa".to_string(), "Vestuário".to_string());

    let electronics: Vec<_> = catalog
        .iter()
        .filter(|(_, category)| *category == "Eletrônicos")
        .map(|(product, _)| product.clone())
        .collect();

    assert_eq!(electronics.len(), 2);
    assert!(electronics.contains(&"Notebook".to_string()));
    assert!(electronics.contains(&"Mouse".to_string()));
}

#[test]
fn test_graph_recommendation() {
    use petgraph::graph::Graph;

    let mut g: Graph<&str, &str> = Graph::new();
    let n1 = g.add_node("Notebook");
    let n2 = g.add_node("Mouse");
    g.add_edge(n1, n2, "comprados juntos");

    // Verifica se existe uma aresta entre Notebook e Mouse
    assert_eq!(g.edge_count(), 1);
}
