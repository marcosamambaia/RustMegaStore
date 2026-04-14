// Importa o módulo de busca (db.rs) e o pool de conexão MySQL
use megastore_search::db;
use sqlx::mysql::MySqlPoolOptions;

// Teste assíncrono usando tokio
// Esse teste valida se a busca por "Notebook" retorna algum resultado
#[tokio::test]
async fn test_search_by_name() {
    // Cria pool de conexão com o banco (ajuste a URL conforme seu ambiente local)
    let pool = MySqlPoolOptions::new()
        .connect("mysql://megastore:1234@localhost/megastore")
        .await
        .expect("Erro ao conectar ao banco");


    // Executa busca por "Notebook"
    let results: Vec<db::ProductResult> = db::search_products(&pool, "Notebook").await;

    // Verifica se retornou pelo menos um resultado
    assert!(
        !results.is_empty(),
        "Busca por Notebook não retornou resultados"
    );
}

// Teste de busca por categoria
// Aqui validamos se ao buscar "Monitores" realmente aparecem produtos dessa categoria
#[tokio::test]
async fn test_search_by_category() {
    let pool = MySqlPoolOptions::new()
        .connect("mysql://megastore:1234@localhost/megastore")
        .await
        .expect("Erro ao conectar ao banco");


    // Busca por "Monitores"
    let results: Vec<db::ProductResult> = db::search_products(&pool, "Monitores").await;

    // Verifica se pelo menos um dos resultados tem categoria "Monitores"
    assert!(
        results.iter().any(|p| p.category == "Monitores"),
        "Nenhum produto da categoria Monitores foi encontrado"
    );
}
