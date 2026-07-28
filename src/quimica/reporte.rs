use std::fs::File;
use std::io::{self, Write};
use chrono::Local;

use crate::quimica::alcalinidad::EstadoAlcalinidad;
use crate::quimica::cloro::TipoCloro;
use crate::quimica::isl::ClasificacionIsl;
use crate::quimica::ph::{AjustePh, EstadoPh};
use crate::quimica::volumen::Alberca;

/// Genera un nombre de archivo dinámico basado en la fecha y hora actual del sistema
/// Ejemplo: "reporte_2026-07-28_125300.txt"
pub fn generar_nombre_archivo() -> String {
    let ahora = Local::now();
    ahora.format("reporte_%Y-%m-%d_%H%M%S.txt").to_string()
}

/// Estructura contenedora con todos los resultados del cálculo
pub struct DatosReporte<'a> {
    pub alberca: &'a Alberca,
    pub producto_cloro: TipoCloro,
    pub dosis_cloro: f64,
    pub ph_actual: f64,
    pub estado_ph: EstadoPh,
    pub alcalinidad_actual: f64,
    pub estado_alcalinidad: EstadoAlcalinidad,
    pub resultado_isl: ClasificacionIsl,
}

/// Genera una cadena formateada con todo el reporte
pub fn construir_texto_reporte(datos: &DatosReporte) -> String {
    let mut texto = String::new();

    texto.push_str("====================================================\n");
    texto.push_str("     ORDEN DE SERVICIO Y TRATAMIENTO QUÍMICO        \n");
    texto.push_str("====================================================\n\n");

    // Datos físicos
    texto.push_str("1. ESPECIFICACIONES DE LA ALBERCA:\n");
    texto.push_str(&format!("   - Largo: {:.2} m | Ancho: {:.2} m | Profundidad: {:.2} m\n", 
            datos.alberca.largo, datos.alberca.ancho, datos.alberca.profundidad_media));
    texto.push_str(&format!("   - Volumen Total: {:.2} Litros ({:.2} m³)\n\n", 
            datos.alberca.volumen_litros, datos.alberca.volumen_m3()));

    // Instrucciones Químicas
    texto.push_str("2. DOSIFICACIÓN DE QUÍMICOS REQUERIDA:\n");

    // Cloro
    match datos.producto_cloro {
        TipoCloro::Liquido => {
            texto.push_str(&format!("   [ ] CLORO: Agregar {:.2} ml de {} ({:.2} L)\n", 
                datos.dosis_cloro, datos.producto_cloro.nombre(), datos.dosis_cloro / 1000.0));
        }
        _ => {
            texto.push_str(&format!("   [ ] CLORO: Agregar {:.2} g de {}\n", 
                datos.dosis_cloro, datos.producto_cloro.nombre()));
        }
    }

    // Alcalinidad
    match datos.estado_alcalinidad {
        EstadoAlcalinidad::Ideal => {
            texto.push_str(&format!("   [✓] ALCALINIDAD: {:.0} ppm (Correcta - Rango 80-120 ppm)\n", datos.alcalinidad_actual));
        }
        EstadoAlcalinidad::Baja(delta) => {
            let gramos = crate::quimica::alcalinidad::calcular_bicarbonato_sodio(datos.alberca, delta);
            texto.push_str(&format!("   [ ] ALCALINIDAD: {:.0} ppm (Baja). Agregar {:.2} g de Bicarbonato de Sodio.\n", datos.alcalinidad_actual, gramos));
        }
        EstadoAlcalinidad::Alta(delta) => {
            let ml = crate::quimica::alcalinidad::calcular_acido_bajar_alcalinidad(datos.alberca, delta);
            texto.push_str(&format!("   [ ] ALCALINIDAD: {:.0} ppm (Alta). Agregar {:.2} ml de Ácido Muriático.\n", datos.alcalinidad_actual, ml));
        }
    }

    // pH
    match datos.estado_ph {
        EstadoPh::Ideal => {
            texto.push_str(&format!("   [✓] pH: {:.2} (Correcto - Rango 7.2-7.6)\n", datos.ph_actual));
        }
        EstadoPh::Bajo(delta) => {
            let dosis = crate::quimica::ph::calcular_dosis_ph(datos.alberca, delta);
            texto.push_str(&format!("   [ ] pH: {:.2} (Bajo). Agregar {:.2} g de {}\n", datos.ph_actual, dosis, AjustePh::SubirPh.nombre()));
        }
        EstadoPh::Alto(delta) => {
            let dosis = crate::quimica::ph::calcular_dosis_ph(datos.alberca, delta);
            texto.push_str(&format!("   [ ] pH: {:.2} (Alto). Agregar {:.2} ml de {}\n", datos.ph_actual, dosis, AjustePh::BajarPh.nombre()));
        }
    }

    // Diagnóstico ISL
    texto.push_str("\n3. DIAGNÓSTICO ISL (ÍNDICE DE LANGELIER):\n");
    match datos.resultado_isl {
        ClasificacionIsl::Equilibrada(v) => {
            texto.push_str(&format!("   - Valor: {:.2} -> AGUA BALANCEADA.\n", v));
        }
        ClasificacionIsl::Corrosiva(v) => {
            texto.push_str(&format!("   - Valor: {:.2} -> ALERTA: AGUA CORROSIVA / AGRESIVA.\n", v));
        }
        ClasificacionIsl::Incrustante(v) => {
            texto.push_str(&format!("   - Valor: {:.2} -> ALERTA: AGUA INCRUSTANTE (Formación de sarro).\n", v));
        }
    }

    texto.push_str("\n====================================================\n");
    texto.push_str("  Firma del Técnico: ______________________________ \n");
    texto.push_str("====================================================\n");

    texto
}

/// Guarda el reporte generado en un archivo .txt en el disco duro
pub fn guardar_archivo_reporte(nombre_archivo: &str, contenido: &str) -> io::Result<()> {
    let mut archivo = File::create(nombre_archivo)?;
    archivo.write_all(contenido.as_bytes())?;
    Ok(())
}