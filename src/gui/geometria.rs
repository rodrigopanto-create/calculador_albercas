//! Controles de forma y dimensiones de la alberca.

use eframe::egui;
use super::AlbercaApp;

/// Renderiza la selección de alberca rectangular o circular y sus dimensiones.
pub fn ui_geometria(ui: &mut egui::Ui, app: &mut AlbercaApp) {
    ui.group(|ui| {
        ui.label("1. Geometría de la Alberca");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut app.es_rectangular, true, "Rectangular");
            ui.selectable_value(&mut app.es_rectangular, false, "Circular");
        });

        if app.es_rectangular {
            ui.add(egui::Slider::new(&mut app.largo, 1.0..=30.0).text("Largo (m)"));
            ui.add(egui::Slider::new(&mut app.ancho, 1.0..=20.0).text("Ancho (m)"));
        } else {
            ui.add(egui::Slider::new(&mut app.diametro, 1.0..=20.0).text("Diámetro (m)"));
        }
        ui.add(egui::Slider::new(&mut app.profundidad, 0.5..=5.0).text("Profundidad (m)"));
    });
}