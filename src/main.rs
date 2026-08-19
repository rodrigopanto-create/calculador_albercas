#[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))]
use calculador_albercas::gui::AlbercaApp;

#[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([500.0, 700.0])
            .with_title("Calculador de Albercas"),
        ..Default::default()
    };

    eframe::run_native(
        "Calculador de Albercas",
        options,
        Box::new(|_cc| Box::new(AlbercaApp::default())),
    )
}

// Cambiado 'or' por 'any'
#[cfg(any(target_os = "android", target_arch = "wasm32"))]
fn main() {}