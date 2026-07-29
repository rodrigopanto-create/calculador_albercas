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

// Código existente de volumen.rs ...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculo_volumen_rectangular() {
        // Alberca de 10m x 5m x 1.5m = 75 m³ = 75,000 Litros
        let alberca = Alberca::nueva_rectangular(10.0, 5.0, 1.5);
        
        assert_eq!(alberca.volumen_m3(), 75.0);
        assert_eq!(alberca.volumen_litros, 75000.0);
    }

    #[test]
    fn test_volumen_dimensiones_cero() {
        let alberca = Alberca::nueva_rectangular(0.0, 5.0, 1.5);
        assert_eq!(alberca.volumen_litros, 0.0);
    }
}