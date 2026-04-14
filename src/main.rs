mod db;
mod gui;

#[tokio::main]
async fn main() {
    let pool = db::get_db_pool().await;

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
    "MegaStore Search",
    native_options,
    Box::new(|_cc| Box::new(gui::MyApp::new(pool))),
);



