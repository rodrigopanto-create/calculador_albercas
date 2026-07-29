//! # Módulo de Análisis de pH y Equilíbrio Químico (ISL)

/// Representa el estado del agua según el Índice de Saturación de Langelier.
#[derive(Debug, PartialEq, Clone)]
pub enum EstadoAgua {
    Corrosiva,
    Equilibrada,
    Incrustante,
}

impl EstadoAgua {
    pub fn descripcion(&self) -> &'static str {
        match self {
            EstadoAgua::Corrosiva => "Agua corrosiva (Riesgo de daño en metal y azulejos)",
            EstadoAgua::Equilibrada => "Agua equilibrada (Rango óptimo)",
            EstadoAgua::Incrustante => "Agua incrustante (Riesgo de formación de sarro)",
        }
    }
}

/// Parámetros necesarios para calcular el ISL.
#[derive(Debug, Clone)]
pub struct ParametrosIsl {
    pub ph: f64,
    pub temperatura_c: f64,      // Temperatura en Celsius
    pub dureza_calcio_ppm: f64,  // Dureza por calcio en ppm
    pub alcalinidad_ppm: f64,    // Alcalinidad total en ppm
    pub tds_ppm: f64,            // Sólidos Disueltos Totales (default usual ~1000)
}

/// Convierte la temperatura en °C a su factor numérico (TF).
fn factor_temperatura(temp_c: f64) -> f64 {
    match temp_c {
        t if t <= 0.0 => 0.0,
        t if t <= 12.0 => 0.3,
        t if t <= 19.0 => 0.4,
        t if t <= 24.0 => 0.5,
        t if t <= 29.0 => 0.6,
        t if t <= 34.0 => 0.7,
        _ => 0.8,
    }
}

/// Convierte la Dureza por Calcio en ppm a su factor numérico (CF).
fn factor_dureza(dureza_ppm: f64) -> f64 {
    if dureza_ppm <= 25.0 { 1.0 }
    else if dureza_ppm <= 75.0 { 1.5 }
    else if dureza_ppm <= 150.0 { 1.8 }
    else if dureza_ppm <= 300.0 { 2.1 }
    else if dureza_ppm <= 600.0 { 2.4 }
    else { 2.7 }
}

/// Convierte la Alcalinidad Total en ppm a su factor numérico (AF).
fn factor_alcalinidad(alcalinidad_ppm: f64) -> f64 {
    if alcalinidad_ppm <= 25.0 { 1.4 }
    else if alcalinidad_ppm <= 75.0 { 1.9 }
    else if alcalinidad_ppm <= 150.0 { 2.2 }
    else if alcalinidad_ppm <= 300.0 { 2.5 }
    else { 2.8 }
}

/// Calcula el Índice de Saturación de Langelier (ISL) y determina la condición del agua.
pub fn calcular_isl(p: &ParametrosIsl) -> (f64, EstadoAgua) {
    let tf = factor_temperatura(p.temperatura_c);
    let cf = factor_dureza(p.dureza_calcio_ppm);
    let af = factor_alcalinidad(p.alcalinidad_ppm);

    // Ajuste por TDS (~12.1 para TDS menor a 1000 ppm)
    let constante_tds = if p.tds_ppm > 1000.0 { 12.2 } else { 12.1 };

    let isl = p.ph + tf + cf + af - constante_tds;

    let estado = match isl {
        val if val < -0.3 => EstadoAgua::Corrosiva,
        val if val > 0.3 => EstadoAgua::Incrustante,
        _ => EstadoAgua::Equilibrada,
    };

    (isl, estado)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isl_equilibrado() {
        let params = ParametrosIsl {
            ph: 7.4,
            temperatura_c: 26.0,
            dureza_calcio_ppm: 250.0,
            alcalinidad_ppm: 100.0,
            tds_ppm: 800.0,
        };
        let (val, estado) = calcular_isl(&params);
        assert!(val >= -0.3 && val <= 0.3);
        assert_eq!(estado, EstadoAgua::Equilibrada);
    }
}