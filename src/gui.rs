use eframe::egui;
use sqlx::MySqlPool;

pub struct MyApp {
    search_query: String,
    results: Vec<String>,
    pool: MySqlPool,
}

impl MyApp {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            search_query: String::new(),
            results: Vec::new(),
            pool,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Busca de Produtos");
            ui.text_edit_singleline(&mut self.search_query);

            if ui.button("Buscar").clicked() {
                let pool = self.pool.clone();
                let query = self.search_query.clone();
                self.results = futures::executor::block_on(
                    crate::db::search_products(&pool, &query)
                );
            }

            if ui.button("Cadastrar Produto").clicked() {
                let pool = self.pool.clone();
                futures::executor::block_on(
                    crate::db::insert_product(&pool, "Mouse Gamer", "Logitech", "Periféricos", "Mouse", 199.90)
                );
                ui.label("Produto cadastrado com sucesso!");
            }

            ui.separator();
            ui.heading("Resultados da Busca:");
            for r in &self.results {
                ui.label(r);
            }
        });
    }
}
