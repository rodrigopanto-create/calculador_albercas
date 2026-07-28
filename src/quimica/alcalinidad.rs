use crate::quimica::volumen::Alberca;

/// Rango idóneo de Alcalinidad Total: 80 - 120 ppm
pub enum EstadoAlcalinidad {
    Baja(f64),  // Requiere subir con Bicarbonato de Sodio
    Ideal,      // En rango seguro
    Alta(f64),  // Requiere bajar con Ácido Muriático
}

/// Evalúa la alcalinidad actual
pub fn evaluar_alcalinidad(ppm_actual: f64) -> EstadoAlcalinidad {
    if ppm_actual < 80.0 {
        EstadoAlcalinidad::Baja(100.0 - ppm_actual) // Apuntamos al centro ideal (~100 ppm)
    } else if ppm_actual > 120.0 {
        EstadoAlcalinidad::Alta(ppm_actual - 100.0)
    } else {
        EstadoAlcalinidad::Ideal
    }
}

/// Dosis para SUBIR la alcalinidad usando Bicarbonato de Sodio.
/// Regla general: ~18 gramos por m³ incrementan 10 ppm de alcalinidad.
pub fn calcular_bicarbonato_sodio(alberca: &Alberca, delta_ppm: f64) -> f64 {
    let m3 = alberca.volumen_m3();
    // 1.8g por m³ por cada 1 ppm que se quiera elevar
    m3 * delta_ppm * 1.8
}

/// Dosis aproximada de Ácido Muriático para BAJAR alcalinidad.
/// Regla general: ~25 ml de Ácido Muriático (31.45%) por m³ reducen 10 ppm.
pub fn calcular_acido_bajar_alcalinidad(alberca: &Alberca, delta_ppm: f64) -> f64 {
    let m3 = alberca.volumen_m3();
    // 2.5 ml por m³ por cada 1 ppm que se quiera reducir
    m3 * delta_ppm * 2.5
}