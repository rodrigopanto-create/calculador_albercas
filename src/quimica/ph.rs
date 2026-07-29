use crate::quimica::volumen::Alberca;

/// Tipos de ajuste de pH disponibles con sus nombres descriptivos
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AjustePh {
    SubirPh,
    BajarPh,
    Ninguno,
}

impl AjustePh {
    /// Retorna el nombre legible del producto o acción requerida
    pub fn nombre(&self) -> &'static str {
        match self {
            AjustePh::SubirPh => "Carbonato de sodio (Incrementador de pH)",
            AjustePh::BajarPh => "Ácido muriático (Reductor de pH)",
            AjustePh::Ninguno => "Ninguno",
        }
    }
}

/// Estado o diagnóstico de la prueba de pH
#[derive(Debug, PartialEq)]
pub enum EstadoPh {
    Ideal,
    Bajo(f64),
    Alto(f64),
}

/// Evalúa el nivel de pH actual
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

/// Calcula la dosis en gramos o ml según el tipo de desbalance
pub fn calcular_dosis_ph(alberca: &Alberca, delta: f64) -> f64 {
    // 10g/ml por m3 por cada 0.1 de ajuste necesario
    delta * 10.0 * 10.0 * alberca.volumen_m3()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ph_ideal() {
        let evaluacion = evaluar_ph(7.4);
        assert_eq!(evaluacion, EstadoPh::Ideal);
    }

    #[test]
    fn test_ph_acido_requiere_incrementador() {
        let evaluacion = evaluar_ph(6.8);
        match evaluacion {
            EstadoPh::Bajo(delta) => {
                assert!(delta > 0.0);
            }
            _ => panic!("Se esperaba EstadoPh::Bajo para pH 6.8"),
        }
    }

    #[test]
    fn test_ph_alcalino_requiere_reductor() {
        let evaluacion = evaluar_ph(8.2);
        match evaluacion {
            EstadoPh::Alto(delta) => {
                assert!(delta > 0.0);
            }
            _ => panic!("Se esperaba EstadoPh::Alto para pH 8.2"),
        }
    }

    #[test]
    fn test_calculo_dosis() {
        let alberca = Alberca::nueva_rectangular(10.0, 5.0, 2.0); // 100 m3
        let dosis = calcular_dosis_ph(&alberca, 0.4);
        assert!(dosis > 0.0);
    }
}