mod quimica;

use quimica::alcalinidad;
use quimica::cloro::{self, TipoCloro};
use quimica::isl;
use quimica::ph;
use quimica::reporte::{self, DatosReporte};
use quimica::volumen::Alberca;
use std::io::{self, Write};

use std::io::{self, Write};

/// Define un tipo de error claro para la entrada de datos
#[derive(Debug)]
pub enum ErrorEntrada {
    FormatoInvalido,
    NumeroNegativo,
    RangoInvalido { min: f64, max: f64 },
}

/// Lee una línea desde la consola y devuelve un Result con el f64 parseado
fn intentar_leer_f64(prompt: &str) -> Result<f64, ErrorEntrada> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut entrada = String::new();
    if io::stdin().read_line(&mut entrada).is_err() {
        return Err(ErrorEntrada::FormatoInvalido);
    }

    // Usamos el Result retornado por parse()
    match entrada.trim().parse::<f64>() {
        Ok(num) => Ok(num),
        Err(_) => Err(ErrorEntrada::FormatoInvalido),
    }
}

/// Función robusta que buclea hasta obtener un número válido dentro de un rango opcional
fn leer_numero_con_rango(prompt: &str, min: Option<f64>, max: Option<f64>) -> f64 {
    loop {
        match intentar_leer_f64(prompt) {
            Ok(num) => {
                // Validación opcional usando Option<f64>
                if let Some(limite_min) = min {
                    if num < limite_min {
                        println!("  [!] El valor no puede ser menor a {}.", limite_min);
                        continue;
                    }
                }
                if let Some(limite_max) = max {
                    if num > limite_max {
                        println!("  [!] El valor no puede ser mayor a {}.", limite_max);
                        continue;
                    }
                }
                return num; // Si pasó las verificaciones, retornamos el valor
            }
            Err(ErrorEntrada::FormatoInvalido) => {
                println!("  [!] Entrada no válida. Por favor ingresa únicamente números.");
            }
            Err(_) => {
                println!("  [!] Ocurrió un error al procesar el dato.");
            }
        }
    }
}

fn seleccionar_tipo_cloro() -> TipoCloro {
    println!("\n¿Qué tipo de cloro vas a usar?");
    println!("  1) Cloro Líquido (12%)");
    println!("  2) Tricloro en Pastillas (90%)");
    println!("  3) Dicloro Granulado (56%)");
    println!("  4) Hipoclorito de Calcio (65%)");

    loop {
        let opcion = leer_numero("Selecciona una opción (1-4): ");
        match opcion as u32 {
            1 => return TipoCloro::Liquido,
            2 => return TipoCloro::Tricloro,
            3 => return TipoCloro::Dicloro,
            4 => return TipoCloro::CalHipoclorito,
            _ => println!("Opción no válida. Elige entre 1 y 4."),
        }
    }
}

fn main() {
    println!("=================================");
    println!("   CALCULADORA DE ALBERCAS v0.7  ");
    println!("=================================\n");

    println!("--- 1. Datos de la Alberca ---");
// Solo aceptamos números mayores a 0 para dimensiones
let largo = leer_numero_con_rango("Largo (m): ", Some(0.1), None);
let ancho = leer_numero_con_rango("Ancho (m): ", Some(0.1), None);
let prof = leer_numero_con_rango("Profundidad media (m): ", Some(0.1), None);

println!("\n--- 2. Parámetros Químicos Principales ---");
let ppm_cloro_act = leer_numero_con_rango("Cloro actual (ppm): ", Some(0.0), Some(20.0));
let ph_actual = leer_numero_con_rango("Nivel de pH (0-14): ", Some(0.0), Some(14.0));

    println!("\n--- 3. Parámetros para Índice de Langelier ---");
    let temp_celsius = leer_numero("Temperatura del agua (°C, ej. 26): ");
    let dureza_calcica = leer_numero("Dureza Cálcica (ppm, ej. 250): ");

    // Evaluaciones
    let estado_ph = ph::evaluar_ph(ph_actual);
    let estado_alcalinidad = alcalinidad::evaluar_alcalinidad(alcalinidad_actual);
    let resultado_isl = isl::calcular_isl(ph_actual, temp_celsius, dureza_calcica, alcalinidad_actual);

    // Empaquetamos todo en nuestro DTO / Struct de DatosReporte
    let reporte_datos = DatosReporte {
        alberca: &mi_alberca,
        producto_cloro,
        dosis_cloro,
        ph_actual,
        estado_ph,
        alcalinidad_actual,
        estado_alcalinidad,
        resultado_isl,
    };

    // Imprimir en consola el reporte construido
    let texto_reporte = reporte::construir_texto_reporte(&reporte_datos);
    println!("\n{}", texto_reporte);

    // Preguntar si desea guardar en disco
    print!("¿Deseas guardar este reporte en un archivo .txt? (s/n): ");
    io::stdout().flush().unwrap();
    let mut respuesta = String::new();
    io::stdin().read_line(&mut respuesta).expect("Error al leer");

    if respuesta.trim().to_lowercase().starts_with('s') {
        let nombre_archivo = reporte::generar_nombre_archivo();
        
        match reporte::guardar_archivo_reporte(&nombre_archivo, &texto_reporte) {
            Ok(_) => println!("\n[✔] ¡Reporte guardado con éxito como '{}'!", nombre_archivo),
            Err(e) => println!("\n[✘] Error al guardar el archivo: {}", e),
        }
    }

    println!("\nGracias por usar la Calculadora de Albercas.");
}