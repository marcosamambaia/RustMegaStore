use petgraph::graph::Graph;
use std::collections::HashMap;

fn main() {
    // Indexação simples com HashMap
    let mut catalog: HashMap<String, String> = HashMap::new();
    catalog.insert("Notebook".to_string(), "Eletrônicos".to_string());
    catalog.insert("Camisa".to_string(), "Vestuário".to_string());

    // Grafo para recomendações
    let mut g: Graph<&str, &str> = Graph::new();
    let n1 = g.add_node("Notebook");
    let n2 = g.add_node("Mouse");
    g.add_edge(n1, n2, "frequentemente comprados juntos");

    println!("Busca: {:?}", catalog.get("Notebook"));
    println!("Recomendações: Notebook -> Mouse");
}
