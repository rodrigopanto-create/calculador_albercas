/// Representa una piscina con sus dimensiones físicas y volumen
#[derive(Debug, Clone)]
pub struct Alberca {
    pub largo: f64,
    pub ancho: f64,
    pub profundidad_media: f64,
    pub volumen_litros: f64,
}

impl Alberca {
    /// Constructor: crea una alberca rectangular y calcula su volumen automáticamente
    pub fn nueva_rectangular(largo: f64, ancho: f64, profundidad_media: f64) -> Self {
        let volumen_litros = (largo * ancho * profundidad_media) * 1000.0;

        Alberca {
            largo,
            ancho,
            profundidad_media,
            volumen_litros,
        }
    }

    /// Método para obtener el volumen en metros cúbicos
    pub fn volumen_m3(&self) -> f64 {
        self.volumen_litros / 1000.0
    }
}