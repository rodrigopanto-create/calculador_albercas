//! # Módulo de Análisis y Ajuste de pH
//!
//! Este módulo contiene los algoritmos para diagnosticar el nivel de pH
//! en albercas residenciales y calcular la dosificación exacta de correctores químicos.

use crate::quimica::volumen::Alberca;

/// Define el tipo de corrector químico necesario para nivelar el pH.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AjustePh {
    /// Se utiliza cuando el pH es menor a 7.2.
    SubirPh,
    /// Se utiliza cuando el pH es mayor a 7.6.
    BajarPh,
    /// Se utiliza cuando el pH está en el rango óptimo (7.2 - 7.6).
    Ninguno,
}

impl AjustePh {
    /// Retorna el nombre comercial/químico del producto recomendado.
    ///
    /// # Ejemplo
    /// ```rust
    /// use calculador_albercas::quimica::ph::AjustePh;
    ///
    /// let ajuste = AjustePh::SubirPh;
    /// assert_eq!(ajuste.nombre(), "Carbonato de sodio (Incrementador de pH)");
    /// ```
    pub fn nombre(&self) -> &'static str {
        match self {
            AjustePh::SubirPh => "Carbonato de sodio (Incrementador de pH)",
            AjustePh::BajarPh => "Ácido muriático (Reductor de pH)",
            AjustePh::Ninguno => "Ninguno",
        }
    }
}

/// Diagnóstico numérico del pH.
#[derive(Debug, PartialEq)]
pub enum EstadoPh {
    /// pH equilibrado entre 7.2 y 7.6.
    Ideal,
    /// pH menor a 7.2. Contiene la desviación ($\Delta$).
    Bajo(f64),
    /// pH mayor a 7.6. Contiene la desviación ($\Delta$).
    Alto(f64),
}

/// Evalúa una lectura de pH y determina el diagnóstico de balance.
///
/// # Parámetros
/// * `ph_actual` - Valor flotante entre 0.0 y 14.0.
///
/// # Retorno
/// Devuelve un valor de [`EstadoPh`] con el diagnóstico correspondiente.
pub fn evaluar_ph(ph_actual: f64) -> EstadoPh {
    if ph_actual < 7.2 {
        let delta = 7.2 - ph_actual;
        EstadoPh::Bajo(delta)
    } else if ph_actual > 7.6 {
        let delta = ph_actual - 7.6;
        EstadoPh::Alto(delta)
    } else {
        EstadoPh::Ideal
    }
}

/// Calcula la cantidad requerida de producto químico (en gramos o ml) según el volumen de la alberca.
///
/// Utiliza una tasa estándar de 10 unidades de producto por $m^3$ por cada 0.1 de ajuste.
pub fn calcular_dosis_ph(alberca: &Alberca, delta: f64) -> f64 {
    delta * 10.0 * 10.0 * alberca.volumen_m3()
}