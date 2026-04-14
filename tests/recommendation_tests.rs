// Importa o motor de recomendação (recommendation.rs)
use megastore_search::recommendation::RecommendationEngine;
use sqlx::mysql::MySqlPoolOptions;

#[tokio::test]
async fn test_recommendations_exist() {
    // Cria pool de conexão com o banco
    let pool = MySqlPoolOptions::new()
        .connect("mysql://megastore:1234@localhost/megastore")
        .await
        .expect("Erro ao conectar ao banco");


    // Inicializa motor de recomendação
    let mut engine = RecommendationEngine::new();

    // Carrega produtos do banco para dentro do grafo
    engine
        .load_products(&pool)
        .await
        .expect("Erro ao carregar produtos");

    // Escolhe um produto específico (Notebook Gamer, id=1)
    let product_id = 1;

    // Pede recomendações para esse produto
    let recs = engine.recommend(product_id, 5);

    // Verifica se retornou pelo menos uma recomendação
    assert!(
        !recs.is_empty(),
        "Nenhuma recomendação encontrada para produto_id=1"
    );
}
