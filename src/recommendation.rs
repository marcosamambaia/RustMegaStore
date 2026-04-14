use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::dijkstra;
use sqlx::MySqlPool;
use crate::db::ProductResult;
use sqlx::Row;


pub struct RecommendationEngine {
    graph: Graph<ProductResult, f64>, // grafo com produtos e pesos
    index_map: Vec<NodeIndex>,        // mapeia cada produto para um nó
}

impl RecommendationEngine {
    pub fn new() -> Self {
        Self {
            graph: Graph::<ProductResult, f64>::new(),
            index_map: Vec::new(),
        }
    }

    /// Carrega produtos do banco e cria nós e arestas
    pub async fn load_products(&mut self, pool: &MySqlPool) -> sqlx::Result<()> {
        let rows = sqlx::query(
            "SELECT id, name, brand, category, product_type, price FROM products"
        )
        .fetch_all(pool)
        .await?;

        for row in rows {
            let product = ProductResult {
                id: row.get("id"),
                name: row.get("name"),
                brand: row.get("brand"),
                category: row.get("category"),
                product_type: row.get("product_type"),
                price: row.get("price"),
            };
            let node = self.graph.add_node(product);
            self.index_map.push(node);
        }

        // Exemplo: criar arestas entre produtos da mesma categoria
        for i in 0..self.index_map.len() {
            for j in (i+1)..self.index_map.len() {
                let pi = &self.graph[self.index_map[i]];
                let pj = &self.graph[self.index_map[j]];
                if pi.category == pj.category {
                    self.graph.add_edge(self.index_map[i], self.index_map[j], 1.0);
                }
            }
        }

        Ok(())
    }

    /// Recomenda produtos relacionados a um produto específico
    pub fn recommend(&self, product_id: i32, max_results: usize) -> Vec<&ProductResult> {
        let node = self.index_map
            .iter()
            .find(|&&n| self.graph[n].id == product_id);

        if let Some(&start) = node {
            let scores = dijkstra(&self.graph, start, None, |e| *e.weight());
            let mut related: Vec<_> = scores
                .iter()
                .filter(|&(idx, _)| *idx != start)
                .map(|(idx, &score)| (score, &self.graph[*idx]))
                .collect();

            related.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            related.into_iter().take(max_results).map(|(_, p)| p).collect()
        } else {
            Vec::new()
        }
    }
}
