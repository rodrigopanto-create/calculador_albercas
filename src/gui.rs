use eframe::egui;
use crate::quimica::{
    cloro::{self, TipoCloro},
    reporte::{generar_reporte, guardar_reporte_en_archivo, DatosReporte},
    volumen::Alberca,
};

pub struct AlbercaApp {
    // Selección de tipo
    es_rectangular: bool,

    // Dimensiones
    largo: f64,
    ancho: f64,
    diametro: f64,
    profundidad: f64,

    // Química
    ph_actual: f64,
    cloro_actual: f64,
    cloro_objetivo: f64,
    temperatura_c: f64,
    dureza_calcio_ppm: f64,
    alcalinidad_ppm: f64,
    tipo_cloro: TipoCloro,

    // Salida / Estado
    reporte_generado: String,
    mensaje_estado: String,
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🏊 Calculador de Tratamiento de Albercas");
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                // --- SECCIÓN 1: DIMENSIONES ---
                ui.group(|ui| {
                    ui.label(egui::RichText::new("1. Geometría de la Alberca").strong());
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.es_rectangular, true, "Rectangular");
                        ui.radio_value(&mut self.es_rectangular, false, "Circular");
                    });

                    if self.es_rectangular {
                        ui.add(egui::Slider::new(&mut self.largo, 1.0..=30.0).text("Largo (m)"));
                        ui.add(egui::Slider::new(&mut self.ancho, 1.0..=20.0).text("Ancho (m)"));
                    } else {
                        ui.add(egui::Slider::new(&mut self.diametro, 1.0..=20.0).text("Diámetro (m)"));
                    }
                    ui.add(egui::Slider::new(&mut self.profundidad, 0.5..=5.0).text("Profundidad Media (m)"));
                });

                ui.add_space(8.0);

                // --- SECCIÓN 2: QUÍMICA ---
                ui.group(|ui| {
                    ui.label(egui::RichText::new("2. Parámetros del Agua").strong());
                    
                    ui.add(egui::Slider::new(&mut self.ph_actual, 6.0..=8.5).text("pH Actual"));
                    ui.add(egui::Slider::new(&mut self.cloro_actual, 0.0..=5.0).text("Cloro Actual (ppm)"));
                    ui.add(egui::Slider::new(&mut self.cloro_objetivo, 1.0..=5.0).text("Cloro Objetivo (ppm)"));
                    ui.add(egui::Slider::new(&mut self.temperatura_c, 10.0..=40.0).text("Temperatura (°C)"));
                    ui.add(egui::Slider::new(&mut self.dureza_calcio_ppm, 50.0..=800.0).text("Dureza Calcio (ppm)"));
                    ui.add(egui::Slider::new(&mut self.alcalinidad_ppm, 20.0..=300.0).text("Alcalinidad Total (ppm)"));

                    ui.horizontal(|ui| {
                        ui.label("Producto de Cloro:");
                        ui.selectable_value(&mut self.tipo_cloro, TipoCloro::Tricloro, "Tricloro");
                        ui.selectable_value(&mut self.tipo_cloro, TipoCloro::Dicloro, "Dicloro");
                    });
                });

                ui.add_space(10.0);

                // --- BOTÓN DE CÁLCULO ---
                if ui.button(egui::RichText::new("⚙️ Calcular Diagnóstico").size(16.0)).clicked() {
                    let res_alberca = if self.es_rectangular {
                        Alberca::nueva_rectangular(self.largo, self.ancho, self.profundidad)
                    } else {
                        Alberca::nueva_circular(self.diametro, self.profundidad)
                    };

                    match res_alberca {
                        Ok(alberca) => {
                            let dosis = cloro::calcular_dosis_producto(
                                &alberca,
                                self.cloro_actual,
                                self.cloro_objetivo,
                                self.tipo_cloro.clone(),
                            );

                            let datos = DatosReporte {
                                alberca,
                                ph_actual: self.ph_actual,
                                cloro_actual: self.cloro_actual,
                                temperatura_c: self.temperatura_c,
                                dureza_calcio_ppm: self.dureza_calcio_ppm,
                                alcalinidad_ppm: self.alcalinidad_ppm,
                            };

                            let rep = generar_reporte(&datos);
                            let dosificacion = format!(
                                "\n--- DOSIFICACIÓN RECOMENDADA ---\nRequiere: {:.2} {} de producto.",
                                dosis.cantidad, dosis.unidad
                            );

                            self.reporte_generado = format!("{}{}", rep, dosificacion);
                            self.mensaje_estado = "✅ Cálculo realizado correctamente.".to_string();
                        }
                        Err(e) => {
                            self.mensaje_estado = format!("❌ Error: {}", e);
                        }
                    }
                }

                // --- RESULTADOS Y REPORTE ---
                if !self.reporte_generado.is_empty() {
                    ui.add_space(10.0);
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("3. Diagnóstico y Reporte").strong());
                        ui.text_edit_multiline(&mut self.reporte_generado.as_str());

                        if ui.button("💾 Guardar en reporte_alberca.txt").clicked() {
                            match guardar_reporte_en_archivo("reporte_alberca.txt", &self.reporte_generado) {
                                Ok(_) => self.mensaje_estado = "📄 ¡Guardado en 'reporte_alberca.txt'!".to_string(),
                                Err(err) => self.mensaje_estado = format!("❌ Error al guardar: {}", err),
                            }
                        }
                    });
                }

                if !self.mensaje_estado.is_empty() {
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new(&self.mensaje_estado).italics());
                }
            });
        });
    }
}