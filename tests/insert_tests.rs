// Teste para validar inserção de produtos no banco
use megastore_search::db;
use sqlx::mysql::MySqlPoolOptions;

#[tokio::test]
async fn test_insert_product() {
    let pool = MySqlPoolOptions::new()
        .connect("mysql://megastore:1234@localhost/megastore")
        .await
        .expect("Erro ao conectar ao banco");

    // Insere um produto de teste
    let name = "Produto Teste";
    let brand = "Marca Teste";
    let category = "Categoria Teste";
    let product_type = "Tipo Teste";
    let price = 123.45;

    db::insert_product(&pool, name, brand, category, product_type, price).await;


    // Busca pelo produto recém inserido
    let results: Vec<db::ProductResult> = db::search_products(&pool, "Produto Teste").await;

    // Verifica se o produto aparece nos resultados
    assert!(
        results.iter().any(|p| p.name == "Produto Teste"),
        "Produto Teste não foi encontrado após inserção"
    );
}
