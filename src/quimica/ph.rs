/// Productos comerciales habituales para ajustar el pH
#[derive(Debug, Clone, Copy)]
pub enum AjustePh {
    SubirPh,  // Carbonato de Sodio (Soda Ash)
    BajarPh,  // Ácido Muriático / Clorhídrico (~31.45%)
}

impl AjustePh {
    pub fn nombre(&self) -> &'static str {
        match self {
            AjustePh::SubirPh => "Carbonato de Sodio (Incrementador de pH)",
            AjustePh::BajarPh => "Ácido Muriático / Clorhídrico (Reductor de pH)",
        }
    }
}

/// Estado del nivel de pH actual
pub enum EstadoPh {
    Bajo(f64),  // Requiere subir pH
    Ideal,      // En rango perfecto
    Alto(f64),  // Requiere bajar pH
}

/// Evalúa el pH actual contra el rango estándar (7.2 - 7.6)
pub fn evaluar_ph(ph_actual: f64) -> EstadoPh {
    if ph_actual < 7.2 {
        EstadoPh::Bajo(7.4 - ph_actual) // Calculamos la diferencia hacia el ideal neutro (7.4)
    } else if ph_actual > 7.6 {
        EstadoPh::Alto(ph_actual - 7.4)
    } else {
        EstadoPh::Ideal
    }
}

/// Calcula la dosis aproximada del producto químico
/// Dosis base promedio: ~10g de Carbonato o ~10ml de Ácido por m³ (1000L) para mover 0.1 de pH
use crate::quimica::volumen::Alberca;

// ... (se mantienen igual tus enums AjustePh y EstadoPh, y la función evaluar_ph) ...

/// Calcula la dosis de producto químico recibiendo la Alberca por referencia
pub fn calcular_dosis_ph(alberca: &Alberca, delta_ph: f64) -> f64 {
    // Usamos el método de la struct para obtener los m³
    let metros_cubicos = alberca.volumen_m3();
    metros_cubicos * delta_ph * 100.0
}