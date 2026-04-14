use sqlx::{mysql::MySqlPoolOptions, MySql, Pool, Row};

pub async fn get_db_pool() -> Pool<MySql> {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:senha123@127.0.0.1:3306/megastore")
        .await
        .expect("Erro ao conectar ao banco")
}

pub async fn insert_product(
    pool: &Pool<MySql>,
    name: &str,
    brand: &str,
    category: &str,
    product_type: &str,
    price: f64,
) {
    sqlx::query("INSERT INTO products (name, brand, category, product_type, price) VALUES (?, ?, ?, ?, ?)")
        .bind(name)
        .bind(brand)
        .bind(category)
        .bind(product_type)
        .bind(price)
        .execute(pool)
        .await
        .expect("Erro ao inserir produto");
}

pub async fn search_products(pool: &Pool<MySql>, query: &str) -> Vec<String> {
    let rows = sqlx::query("SELECT name FROM products WHERE name LIKE ?")
        .bind(format!("%{}%", query))
        .fetch_all(pool)
        .await
        .expect("Erro ao buscar produtos");

    rows.into_iter().map(|row| row.get("name")).collect()
}
