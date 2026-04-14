use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;

/// Estrutura que representa o sistema de recomendações
pub struct RecommendationSystem {
    graph: Graph<String, String>,
    index_map: HashMap<String, NodeIndex>,
}

impl RecommendationSystem {
    /// Cria um novo sistema de recomendações vazio
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            index_map: HashMap::new(),
        }
    }

    /// Adiciona um produto ao grafo
    pub fn add_product(&mut self, product: &str) {
        if !self.index_map.contains_key(product) {
            let idx = self.graph.add_node(product.to_string());
            self.index_map.insert(product.to_string(), idx);
        }
    }

    /// Cria uma relação de recomendação entre dois produtos
    pub fn add_relation(&mut self, product_a: &str, product_b: &str, relation: &str) {
        self.add_product(product_a);
        self.add_product(product_b);

        let idx_a = self.index_map[product_a];
        let idx_b = self.index_map[product_b];

        self.graph.add_edge(idx_a, idx_b, relation.to_string());
    }

    /// Retorna recomendações para um produto
    pub fn get_recommendations(&self, product: &str) -> Vec<String> {
        if let Some(&idx) = self.index_map.get(product) {
            self.graph
                .neighbors(idx)
                .map(|n| self.graph[n].clone())
                .collect()
        } else {
            vec![]
        }
    }
}
