//! Controles de parámetros químicos y selección del producto de cloro.

use eframe::egui;
use crate::quimica::cloro::TipoCloro;
use super::AlbercaApp;

/// Renderiza los valores de pH, cloro, temperatura, dureza y alcalinidad.
pub fn ui_parametros(ui: &mut egui::Ui, app: &mut AlbercaApp) {
    ui.group(|ui| {
        ui.label("2. Parámetros del Agua");
        ui.add(egui::Slider::new(&mut app.ph_actual, 6.0..=8.5).text("pH Actual"));
        ui.add(egui::Slider::new(&mut app.cloro_actual, 0.0..=10.0).text("Cloro Actual (ppm)"));
        ui.add(egui::Slider::new(&mut app.cloro_objetivo, 0.0..=10.0).text("Cloro Objetivo (ppm)"));
        ui.add(egui::Slider::new(&mut app.temperatura_c, 5.0..=40.0).text("Temperatura (°C)"));
        ui.add(egui::Slider::new(&mut app.dureza_calcio_ppm, 0.0..=1000.0).text("Dureza Calcio (ppm)"));
        ui.add(egui::Slider::new(&mut app.alcalinidad_ppm, 0.0..=500.0).text("Alcalinidad Total (ppm)"));

        ui.horizontal(|ui| {
            ui.label("Producto de Cloro:");
            ui.selectable_value(&mut app.tipo_cloro, TipoCloro::Tricloro, "Tricloro");
            ui.selectable_value(&mut app.tipo_cloro, TipoCloro::Dicloro, "Dicloro");
        });
    });
}