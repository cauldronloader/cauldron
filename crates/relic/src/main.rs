fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([1024.0, 600.0])
            .with_max_inner_size([1024.0, 600.0])
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "unused",
        options,
        Box::new(|_cc| Ok(Box::new(RelicApp::new()))),
    )
}
