//! # Módulo de Reportes

use std::fs::File;
use std::io::Write;

use crate::quimica::ph::{calcular_isl, ParametrosIsl};
use crate::quimica::volumen::Alberca;

#[derive(Debug, Clone)]
pub struct DatosReporte {
    pub alberca: Alberca,
    pub ph_actual: f64,
    pub cloro_actual: f64,
    pub temperatura_c: f64,
    pub dureza_calcio_ppm: f64,
    pub alcalinidad_ppm: f64,
}

pub fn generar_reporte(datos: &DatosReporte) -> String {
    let mut reporte = String::new();

    reporte.push_str("=====================================\n");
    reporte.push_str("    REPORTE DE DIAGNÓSTICO QUÍMICO   \n");
    reporte.push_str("=====================================\n\n");

    // Información de volumen
    match &datos.alberca {
        Alberca::Rectangular { largo, ancho, profundidad, .. } => {
            reporte.push_str(&format!(
                "Dimensiones: Rectangular ({:.1}m x {:.1}m x {:.1}m)\n",
                largo, ancho, profundidad
            ));
        }
        Alberca::Circular { diametro, profundidad, .. } => {
            reporte.push_str(&format!(
                "Dimensiones: Circular (Ø {:.1}m x {:.1}m)\n",
                diametro, profundidad
            ));
        }
    }

    reporte.push_str(&format!(
        "Volumen Total: {:.0} L ({:.1} m³)\n\n",
        datos.alberca.volumen_litros(),
        datos.alberca.volumen_m3()
    ));

    reporte.push_str("--- PARÁMETROS MEDIDOS ---\n");
    reporte.push_str(&format!("* pH: {:.1}\n", datos.ph_actual));
    reporte.push_str(&format!("* Cloro Libre: {:.1} ppm\n", datos.cloro_actual));
    reporte.push_str(&format!("* Temperatura: {:.1} °C\n", datos.temperatura_c));
    reporte.push_str(&format!("* Dureza por Calcio: {:.0} ppm\n", datos.dureza_calcio_ppm));
    reporte.push_str(&format!("* Alcalinidad Total: {:.0} ppm\n\n", datos.alcalinidad_ppm));

    // Cálculo del ISL
    let params_isl = ParametrosIsl {
        ph: datos.ph_actual,
        temperatura_c: datos.temperatura_c,
        dureza_calcio_ppm: datos.dureza_calcio_ppm,
        alcalinidad_ppm: datos.alcalinidad_ppm,
        tds_ppm: 800.0,
    };

    let (val_isl, estado) = calcular_isl(&params_isl);

    reporte.push_str("--- BALANCE DE LANGELIER (ISL) ---\n");
    reporte.push_str(&format!("* Índice ISL: {:.2}\n", val_isl));
    reporte.push_str(&format!("* Diagnóstico: {}\n", estado.descripcion()));

    reporte
}

/// Guarda el contenido del reporte en un archivo .txt especificado.
pub fn guardar_reporte_en_archivo(nombre_archivo: &str, contenido: &str) -> std::io::Result<()> {
    let mut archivo = File::create(nombre_archivo)?;
    archivo.write_all(contenido.as_bytes())?;
    Ok(())
}