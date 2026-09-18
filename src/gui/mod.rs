//! Interfaz gráfica de la aplicación de cálculo para albercas.
//!
//! Este módulo mantiene el estado compartido de [`AlbercaApp`] y coordina las
//! vistas de geometría, parámetros del agua y diagnóstico. Cada vista modifica
//! únicamente los campos necesarios del estado central.

pub mod diagnostico;
pub mod geometria;
pub mod parametros;

use eframe::egui;
use crate::quimica::cloro::TipoCloro;

/// Estado editable de la interfaz y datos necesarios para generar un reporte.
pub struct AlbercaApp {
    // Geometría seleccionada y sus dimensiones en metros.
    pub es_rectangular: bool,

    // Dimensiones
    pub largo: f64,
    pub ancho: f64,
    pub diametro: f64,
    pub profundidad: f64,

    // Parámetros químicos medidos o establecidos como objetivo.
    pub ph_actual: f64,
    pub cloro_actual: f64,
    pub cloro_objetivo: f64,
    pub temperatura_c: f64,
    pub dureza_calcio_ppm: f64,
    pub alcalinidad_ppm: f64,
    pub tipo_cloro: TipoCloro,

    // Reporte visible y mensaje de estado de las operaciones de archivo.
    pub reporte_generado: String,
    pub mensaje_estado: String,
}

impl Default for AlbercaApp {
    fn default() -> Self {
        Self {
            es_rectangular: true,
            largo: 10.0,
            ancho: 5.0,
            diametro: 6.0,
            profundidad: 1.5,
            ph_actual: 7.4,
            cloro_actual: 1.0,
            cloro_objetivo: 3.0,
            temperatura_c: 26.0,
            dureza_calcio_ppm: 250.0,
            alcalinidad_ppm: 100.0,
            tipo_cloro: TipoCloro::Tricloro,
            reporte_generado: String::new(),
            mensaje_estado: String::new(),
        }
    }
}

impl eframe::App for AlbercaApp {
    /// Dibuja la barra superior y compone las vistas desplazables de la GUI.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let top_margin = if cfg!(target_os = "android") { 40.0 } else { 10.0 };

        egui::TopBottomPanel::top("top_panel_albercas")
            .frame(egui::Frame::none().inner_margin(egui::Margin {
                top: top_margin,
                left: 10.0,
                right: 10.0,
                bottom: 5.0,
            }))
            .show(ctx, |ui| {
                ui.heading("Calculador de Tratamiento de Albercas");
                ui.separator();
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                geometria::ui_geometria(ui, self);
                ui.add_space(10.0);

                parametros::ui_parametros(ui, self);
                ui.add_space(10.0);

                diagnostico::ui_diagnostico(ui, self);
            });
        });
    }
}