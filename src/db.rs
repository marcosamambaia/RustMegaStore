use sqlx::{mysql::MySqlPoolOptions, MySql, Pool, Row};
use sqlx::MySqlPool; // importa o alias correto

/// Cria e retorna um pool de conexões com o banco MySQL.
/// Esse pool é usado em toda a aplicação para executar queries.
pub async fn get_db_pool() -> Pool<MySql> {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:senha123@127.0.0.1:3306/megastore")
        .await
        .expect("Erro ao conectar ao banco")
}

/// Insere um novo produto na tabela `products`.
pub async fn insert_product(
    pool: &Pool<MySql>,
    name: &str,
    brand: &str,
    category: &str,
    product_type: &str,
    price: f64,
) {
    sqlx::query(
        "INSERT INTO products (name, brand, category, product_type, price) 
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(name)
    .bind(brand)
    .bind(category)
    .bind(product_type)
    .bind(price)
    .execute(pool)
    .await
    .expect("Erro ao inserir produto");
}

/// Estrutura que representa um produto retornado pela busca.
#[derive(Debug, Clone)]
pub struct ProductResult {
    pub id: i32,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub product_type: String,
    pub price: f64,
}

/// Busca produtos no banco de dados pelo nome, marca ou categoria.
/// Retorna uma lista de `ProductResult` com todos os atributos.
pub async fn search_products(pool: &MySqlPool, query: &str) -> Vec<ProductResult> {
    let rows = sqlx::query(
        "SELECT id, name, brand, category, product_type, price 
         FROM products 
         WHERE name LIKE ? OR brand LIKE ? OR category LIKE ?"
    )
    .bind(format!("%{}%", query))
    .bind(format!("%{}%", query))
    .bind(format!("%{}%", query))
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter().map(|row| ProductResult {
        id: row.get("id"),
        name: row.get("name"),
        brand: row.get("brand"),
        category: row.get("category"),
        product_type: row.get("product_type"),
        price: row.get("price"),
    }).collect()
}
