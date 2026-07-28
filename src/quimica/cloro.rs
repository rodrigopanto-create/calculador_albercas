/// Calcula la cantidad de producto comercial necesario (en gramos o ml según el tipo)
use crate::quimica::volumen::Alberca;

// ... (se mantiene igual tu enum TipoCloro e impl TipoCloro) ...

/// Calcula la cantidad de producto comercial pasando una referencia a la Alberca
pub fn calcular_dosis_producto(
    alberca: &Alberca, 
    ppm_actual: f64, 
    ppm_deseado: f64, 
    producto: TipoCloro
) -> f64 {
    let delta_ppm = (ppm_deseado - ppm_actual).max(0.0);
    
    // Accedemos directamente a alberca.volumen_litros
    let cloro_puro_gramos = (delta_ppm * alberca.volumen_litros) / 1000.0;

    cloro_puro_gramos / producto.concentracion()
}

/// Representa los distintos productos comerciales de cloro
/// y su porcentaje promedio de cloro activo/útil.
#[derive(Debug, Clone, Copy)]
pub enum TipoCloro {
    Liquido,    // Hipoclorito de sodio (~12% cloro útil)
    Tricloro,   // Pastillas o tabletas (~90% cloro útil)
    Dicloro,    // Cloro granulado de disolución rápida (~56% cloro útil)
    CalHipoclorito, // Hipoclorito de calcio (~65% cloro útil)
}

impl TipoCloro {
    /// Devuelve la concentración de cloro activo del producto (de 0.0 a 1.0)
    pub fn concentracion(&self) -> f64 {
        match self {
            TipoCloro::Liquido => 0.12,
            TipoCloro::Tricloro => 0.90,
            TipoCloro::Dicloro => 0.56,
            TipoCloro::CalHipoclorito => 0.65,
        }
    }

    /// Nombre amigable para mostrar en consola
    pub fn nombre(&self) -> &'static str {
        match self {
            TipoCloro::Liquido => "Cloro Líquido (12%)",
            TipoCloro::Tricloro => "Tricloro en Pastillas (90%)",
            TipoCloro::Dicloro => "Dicloro Granulado (56%)",
            TipoCloro::CalHipoclorito => "Hipoclorito de Calcio (65%)",
        }
    }
}