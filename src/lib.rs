pub mod gui;
pub mod quimica;

#[cfg(target_os = "android")]
use android_activity::AndroidApp;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    use eframe::NativeOptions;

    let mut options = NativeOptions::default();
    options.android_app = Some(app);

    eframe::run_native(
        "Calculador de Albercas",
        options,
        Box::new(|_cc| Box::new(gui::AlbercaApp::default())),
    )
    .expect("Error al iniciar la app en Android");
}

// --- CONFIGURACIÓN ANDROID ---
#[cfg(target_os = "android")]
use android_activity::AndroidApp;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(_app: AndroidApp) {
    use eframe::NativeOptions;
    let options = NativeOptions::default();
    eframe::run_native(
        "Calculador de Albercas",
        options,
        Box::new(|_cc| Box::new(gui::AlbercaApp::default())),
    )
    .expect("Error al iniciar en Android");
}

// --- CONFIGURACIÓN WEB (WASM) ---
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Redirigir logs de Rust a la consola del navegador (console.log)
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // El ID del <canvas> en el HTML
                web_options,
                Box::new(|_cc| Box::new(gui::AlbercaApp::default())),
            )
            .await
            .expect("Error al iniciar eframe en WASM");
    });

    Ok(())
}