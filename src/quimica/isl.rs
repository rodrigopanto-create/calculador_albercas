/// Resultado del análisis de saturación del agua
pub enum ClasificacionIsl {
    Corrosiva(f64),  // ISL < -0.3 (Daña equipos y superficies)
    Equilibrada(f64), // -0.3 <= ISL <= 0.3 (Perfecto)
    Incrustante(f64), // ISL > 0.3 (Forma sarro y turbidez)
}

/// Convierte Temperatura (°C) a su factor de conversión logarítmico (TF)
fn factor_temperatura(temp_celsius: f64) -> f64 {
    // Tabla aproximada estándar para albercas
    if temp_celsius <= 0.0 { 0.0 }
    else if temp_celsius <= 10.0 { 0.3 }
    else if temp_celsius <= 15.0 { 0.4 }
    else if temp_celsius <= 20.0 { 0.5 }
    else if temp_celsius <= 24.0 { 0.6 }
    else if temp_celsius <= 29.0 { 0.7 }
    else if temp_celsius <= 34.0 { 0.8 }
    else { 0.9 }
}

/// Convierte Dureza Cálcica (ppm) a su factor de conversión (CF)
fn factor_dureza(dureza_ppm: f64) -> f64 {
    if dureza_ppm <= 25.0 { 1.0 }
    else if dureza_ppm <= 50.0 { 1.3 }
    else if dureza_ppm <= 100.0 { 1.6 }
    else if dureza_ppm <= 150.0 { 1.8 }
    else if dureza_ppm <= 200.0 { 1.9 }
    else if dureza_ppm <= 300.0 { 2.1 }
    else if dureza_ppm <= 400.0 { 2.2 }
    else if dureza_ppm <= 800.0 { 2.5 }
    else { 2.7 }
}

/// Convierte Alcalinidad Total (ppm) a su factor de conversión (AF)
fn factor_alcalinidad(alcalinidad_ppm: f64) -> f64 {
    if alcalinidad_ppm <= 25.0 { 1.4 }
    else if alcalinidad_ppm <= 50.0 { 1.7 }
    else if alcalinidad_ppm <= 75.0 { 1.9 }
    else if alcalinidad_ppm <= 100.0 { 2.0 }
    else if alcalinidad_ppm <= 150.0 { 2.2 }
    else if alcalinidad_ppm <= 200.0 { 2.3 }
    else if alcalinidad_ppm <= 300.0 { 2.5 }
    else if alcalinidad_ppm <= 400.0 { 2.6 }
    else { 2.8 }
}

/// Calcula el valor final del Índice de Saturación de Langelier (ISL)
pub fn calcular_isl(
    ph: f64,
    temp_celsius: f64,
    dureza_calcica_ppm: f64,
    alcalinidad_ppm: f64,
) -> ClasificacionIsl {
    let tf = factor_temperatura(temp_celsius);
    let cf = factor_dureza(dureza_calcica_ppm);
    let af = factor_alcalinidad(alcalinidad_ppm);
    
    // Constante TDS estándar (~12.1 para agua dulce de alberca)
    let factor_tds = 12.1;

    let isl_valor = ph + tf + cf + af - factor_tds;

    if isl_valor < -0.3 {
        ClasificacionIsl::Corrosiva(isl_valor)
    } else if isl_valor > 0.3 {
        ClasificacionIsl::Incrustante(isl_valor)
    } else {
        ClasificacionIsl::Equilibrada(isl_valor)
    }
}