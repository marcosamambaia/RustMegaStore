use eframe::egui;
use sqlx::MySqlPool;

/// Estrutura principal da aplicação.
/// Mantém estado da busca, resultados, recomendações e campos do formulário.
pub struct MyApp {
    search_query: String,
    results: Vec<crate::db::ProductResult>,
    recommendations: Vec<crate::db::ProductResult>, // <-- novo campo para guardar recomendações
    pool: MySqlPool,
    // campos do formulário de cadastro
    name: String,
    brand: String,
    category: String,
    product_type: String,
    price: String,
    // flag para controlar se a janela de cadastro está aberta
    show_cadastro: bool,
}

impl MyApp {
    /// Construtor da aplicação
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            search_query: String::new(),
            results: Vec::new(),
            recommendations: Vec::new(), // inicia vazio
            pool,
            name: String::new(),
            brand: String::new(),
            category: String::new(),
            product_type: String::new(),
            price: String::new(),
            show_cadastro: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Painel central da aplicação
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MegaStore Search");

            // Área de busca
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.search_query);
                if ui.button("Buscar").clicked() {
                    let pool = self.pool.clone();
                    let query = self.search_query.clone();
                    self.results = futures::executor::block_on(
                        crate::db::search_products(&pool, &query)
                    );
                    // limpa recomendações ao fazer nova busca
                    self.recommendations.clear();
                }
            });

            ui.separator();

            // Botão que abre a janela de cadastro
            if ui.button("Cadastrar Produto").clicked() {
                self.show_cadastro = true;
            }

            // Botão que chama o sistema de recomendação
            if ui.button("Recomendar").clicked() {
                let mut engine = crate::recommendation::RecommendationEngine::new();
                if let Err(e) = futures::executor::block_on(engine.load_products(&self.pool)) {
                    eprintln!("Erro ao carregar produtos: {}", e);
                } else if let Some(first) = self.results.first() {
                    let recs = engine.recommend(first.id, 5);
                    // guarda recomendações no estado da aplicação
                    self.recommendations = recs.into_iter().cloned().collect();
                }
            }

            ui.separator();
            ui.heading("Resultados da Busca:");
            for r in &self.results {
                ui.label(format!(
                    "{} | Marca: {} | Categoria: {} | Tipo: {} | Preço: R${:.2}",
                    r.name, r.brand, r.category, r.product_type, r.price
                ));
            }

            ui.separator();
            ui.heading("Recomendações:");
            if self.recommendations.is_empty() {
                ui.label("Nenhuma recomendação disponível.");
            } else {
                for r in &self.recommendations {
                    ui.label(format!("Recomendado: {} ({})", r.name, r.category));
                }
            }
        });

        // Janela modal de cadastro de produto
        if self.show_cadastro {
            egui::Window::new("Cadastro de Produto")
                .show(ctx, |ui| {
                    ui.label("Nome:");
                    ui.text_edit_singleline(&mut self.name);

                    ui.label("Marca:");
                    ui.text_edit_singleline(&mut self.brand);

                    ui.label("Categoria:");
                    ui.text_edit_singleline(&mut self.category);

                    ui.label("Tipo:");
                    ui.text_edit_singleline(&mut self.product_type);

                    ui.label("Preço:");
                    ui.text_edit_singleline(&mut self.price);

                    // Botão para salvar no banco
                    if ui.button("Salvar").clicked() {
                        if let Ok(price_val) = self.price.parse::<f64>() {
                            let pool = self.pool.clone();
                            futures::executor::block_on(
                                crate::db::insert_product(
                                    &pool,
                                    &self.name,
                                    &self.brand,
                                    &self.category,
                                    &self.product_type,
                                    price_val,
                                )
                            );
                            // Limpa os campos após salvar
                            self.name.clear();
                            self.brand.clear();
                            self.category.clear();
                            self.product_type.clear();
                            self.price.clear();
                            // Fecha a janela
                            self.show_cadastro = false;
                        } else {
                            ui.label("Preço inválido!");
                        }
                    }

                    // Botão para cancelar/fechar manualmente
                    if ui.button("Cancelar").clicked() {
                        self.show_cadastro = false;
                    }
                });
        }
    }
}
