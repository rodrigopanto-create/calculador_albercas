//! # Módulo de Tratamiento con Cloro

use crate::quimica::volumen::Alberca;

/// Representa los tipos de cloro comercial y su concentración.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TipoCloro {
    Tricloro,  // 90% concentración
    Dicloro,   // 60% concentración
    Liquido,   // 12% concentración (Hipoclorito de sodio)
}

impl TipoCloro {
    pub fn porcentaje_cloro_puro(&self) -> f64 {
        match self {
            TipoCloro::Tricloro => 0.90,
            TipoCloro::Dicloro => 0.60,
            TipoCloro::Liquido => 0.12,
        }
    }

    pub fn unidad_medida(&self) -> &'static str {
        match self {
            TipoCloro::Tricloro | TipoCloro::Dicloro => "gramos",
            TipoCloro::Liquido => "mililitros",
        }
    }
}

/// Estructura que representa el resultado del cálculo de cloración.
#[derive(Debug, PartialEq)]
pub struct DosisCloro {
    pub cantidad: f64,
    pub unidad: &'static str,
    pub tipo: TipoCloro,
}

pub fn calcular_dosis_producto(
    alberca: &Alberca,
    cloro_actual: f64,
    cloro_objetivo: f64,
    tipo: TipoCloro,
) -> DosisCloro {
    if cloro_actual >= cloro_objetivo {
        return DosisCloro {
            cantidad: 0.0,
            unidad: tipo.unidad_medida(),
            tipo,
        };
    }

    let delta_ppm = cloro_objetivo - cloro_actual;
    // Uso del método con ()
    let cloro_puro_gramos = (delta_ppm * alberca.volumen_litros()) / 1000.0;
    let cantidad_producto = cloro_puro_gramos / tipo.porcentaje_cloro_puro();

    DosisCloro {
        cantidad: cantidad_producto,
        unidad: tipo.unidad_medida(),
        tipo,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dosis_tricloro() {
        let alberca = Alberca::nueva_rectangular(10.0, 5.0, 2.0).unwrap();
        let dosis = calcular_dosis_producto(&alberca, 3.0, 2.0, TipoCloro::Tricloro);
        assert_eq!(dosis.cantidad, 0.0);
    }

    #[test]
    fn test_dosis_liquido() {
        let alberca = Alberca::nueva_rectangular(10.0, 5.0, 2.0).unwrap();
        let dosis = calcular_dosis_producto(&alberca, 1.0, 3.0, TipoCloro::Liquido);
        assert!(dosis.cantidad > 0.0);
    }

    #[test]
    fn test_dosis_granulado() {
        let alberca = Alberca::nueva_rectangular(10.0, 5.0, 2.0).unwrap();
        let dosis = calcular_dosis_producto(&alberca, 1.0, 3.0, TipoCloro::Tricloro);
        assert!(dosis.cantidad > 0.0);
    }
}