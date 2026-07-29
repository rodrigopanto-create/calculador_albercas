//! # Módulo de Cálculo de Volumen
//!
//! Proporciona modelos geométricos para calcular el volumen de albercas
//! con validación de dimensiones.

/// Representa los tipos de alberca soportados y sus dimensiones en metros.
#[derive(Debug, PartialEq, Clone)]
pub enum Alberca {
    Rectangular { largo: f64, ancho: f64, profundidad: f64 },
    Circular { diametro: f64, profundidad: f64 },
}

impl Alberca {
    /// Crea una alberca rectangular validando que las dimensiones sean mayores a cero.
    ///
    /// # Errores
    /// Devuelve `Err(&str)` si alguna dimensión es menor o igual a 0.0.
    pub fn nueva_rectangular(largo: f64, ancho: f64, profundidad: f64) -> Result<Self, &'static str> {
        if largo <= 0.0 || ancho <= 0.0 || profundidad <= 0.0 {
            return Err("Las dimensiones de la alberca deben ser mayores a cero.");
        }
        Ok(Alberca::Rectangular { largo, ancho, profundidad })
    }

    /// Crea una alberca circular validando que el diámetro y profundidad sean mayores a cero.
    pub fn nueva_circular(diametro: f64, profundidad: f64) -> Result<Self, &'static str> {
        if diametro <= 0.0 || profundidad <= 0.0 {
            return Err("El diámetro y la profundidad deben ser mayores a cero.");
        }
        Ok(Alberca::Circular { diametro, profundidad })
    }

    /// Calcula el volumen en metros cúbicos ($m^3$).
    pub fn volumen_m3(&self) -> f64 {
        match self {
            Alberca::Rectangular { largo, ancho, profundidad } => largo * ancho * profundidad,
            Alberca::Circular { diametro, profundidad } => {
                let radio = diametro / 2.0;
                std::f64::consts::PI * radio.powi(2) * profundidad
            }
        }
    }

    /// Convierte el volumen de metros cúbicos a litros.
    pub fn volumen_litros(&self) -> f64 {
        self.volumen_m3() * 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alberca_valida() {
        let alberca = Alberca::nueva_rectangular(10.0, 5.0, 2.0);
        assert!(alberca.is_ok());
        assert_eq!(alberca.unwrap().volumen_m3(), 100.0);
    }

    #[test]
    fn test_alberca_dimensiones_invalidas() {
        let alberca = Alberca::nueva_rectangular(-10.0, 5.0, 2.0);
        assert!(alberca.is_err());
        assert_eq!(alberca.unwrap_err(), "Las dimensiones de la alberca deben ser mayores a cero.");
    }
}