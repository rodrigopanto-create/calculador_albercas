pub mod gui;
pub mod quimica;

#[cfg(target_os = "android")]
use android_activity::AndroidApp;

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: AndroidApp) {
    use eframe::NativeOptions;
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let mut options = NativeOptions::default();

    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Calculadora Albercas",
        options,
        Box::new(|_cc| Ok(Box::new(gui::AlbercaApp::default()))),
    );
}

// --- CONFIGURACIÓN WEB (WASM) ---
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|_cc| Box::new(gui::AlbercaApp::default())),
            )
            .await
            .expect("Error al iniciar eframe en WASM");
    });

    Ok(())
}