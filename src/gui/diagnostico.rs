//! Generación, visualización y guardado del diagnóstico de la alberca.

use eframe::egui;
use crate::quimica::{
    reporte::{generar_reporte, guardar_reporte_en_archivo, DatosReporte},
    volumen::Alberca,
};
use super::AlbercaApp;

/// Calcula el diagnóstico con el estado actual y permite guardar el reporte.
pub fn ui_diagnostico(ui: &mut egui::Ui, app: &mut AlbercaApp) {
    if ui.button("⚙ Calcular Diagnóstico").clicked() {
        let alberca = if app.es_rectangular {
            Alberca::Rectangular {
                largo: app.largo,
                ancho: app.ancho,
                profundidad: app.profundidad,
            }
        } else {
            Alberca::Circular {
                diametro: app.diametro,
                profundidad: app.profundidad,
            }
        };

        let datos = DatosReporte {
            alberca,
            ph_actual: app.ph_actual,
            cloro_actual: app.cloro_actual,
            temperatura_c: app.temperatura_c,
            dureza_calcio_ppm: app.dureza_calcio_ppm,
            alcalinidad_ppm: app.alcalinidad_ppm,
        };

        app.reporte_generado = generar_reporte(&datos);
        app.mensaje_estado.clear();
    }

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("3. Diagnóstico y Reporte");
        if !app.reporte_generado.is_empty() {
            ui.code_editor(&mut app.reporte_generado.as_str());

            if ui.button("💾 Guardar reporte").clicked() {
                match guardar_reporte_en_archivo(&app.reporte_generado, "reporte_alberca.txt") {
                    Ok(_) => app.mensaje_estado = "Guardado con éxito".to_string(),
                    Err(e) => app.mensaje_estado = format!("Error al guardar: {}", e),
                }
            }
            if !app.mensaje_estado.is_empty() {
                ui.label(&app.mensaje_estado);
            }
        } else {
            ui.label("Haz clic en 'Calcular Diagnóstico' para generar el reporte.");
        }
    });
}