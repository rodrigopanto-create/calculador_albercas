mod quimica;

use quimica::alcalinidad;
use quimica::cloro::{self, TipoCloro};
use quimica::isl;
use quimica::ph;
use quimica::reporte::{self, DatosReporte};
use quimica::volumen::Alberca;
use std::io::{self, Write};

/// Tipo de error personalizado para la entrada de la consola
#[derive(Debug)]
pub enum ErrorEntrada {
    FormatoInvalido,
}

/// Intenta leer un número `f64` desde la consola devolviendo un Result
fn intentar_leer_f64(prompt: &str) -> Result<f64, ErrorEntrada> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut entrada = String::new();
    if io::stdin().read_line(&mut entrada).is_err() {
        return Err(ErrorEntrada::FormatoInvalido);
    }

    match entrada.trim().parse::<f64>() {
        Ok(num) => Ok(num),
        Err(_) => Err(ErrorEntrada::FormatoInvalido),
    }
}

/// Lee un número de forma interactiva validando sus rangos con Option
fn leer_numero_con_rango(prompt: &str, min: Option<f64>, max: Option<f64>) -> f64 {
    loop {
        match intentar_leer_f64(prompt) {
            Ok(num) => {
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
                return num;
            }
            Err(ErrorEntrada::FormatoInvalido) => {
                println!("  [!] Entrada no válida. Por favor ingresa únicamente números.");
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
        let opcion = leer_numero_con_rango("Selecciona una opción (1-4): ", Some(1.0), Some(4.0));
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
    println!("   CALCULADORA DE ALBERCAS v0.8  ");
    println!("=================================\n");

    println!("--- 1. Datos de la Alberca ---");
    let largo = leer_numero_con_rango("Largo (m): ", Some(0.1), None);
    let ancho = leer_numero_con_rango("Ancho (m): ", Some(0.1), None);
    let prof = leer_numero_con_rango("Profundidad media (m): ", Some(0.1), None);

    let mi_alberca = Alberca::nueva_rectangular(largo, ancho, prof);

    println!(
        "\n>> Alberca creada: {:.2} L ({:.2} m³)\n",
        mi_alberca.volumen_litros,
        mi_alberca.volumen_m3()
    );

    println!("--- 2. Parámetros Químicos Principales ---");
    let ppm_cloro_act = leer_numero_con_rango("Concentración actual de cloro (ppm): ", Some(0.0), Some(50.0));
    let ppm_cloro_des = leer_numero_con_rango("Concentración deseada de cloro (ppm): ", Some(0.0), Some(50.0));
    let producto_cloro = seleccionar_tipo_cloro();

    let dosis_cloro = cloro::calcular_dosis_producto(
        &mi_alberca,
        ppm_cloro_act,
        ppm_cloro_des,
        producto_cloro,
    );

    let ph_actual = leer_numero_con_rango("\nNivel de pH actual (0-14, ej. 7.4): ", Some(0.0), Some(14.0));
    let alcalinidad_actual = leer_numero_con_rango("Alcalinidad Total (ppm, ej. 100): ", Some(0.0), Some(500.0));

    println!("\n--- 3. Parámetros para Índice de Langelier ---");
    let temp_celsius = leer_numero_con_rango("Temperatura del agua (°C, ej. 26): ", Some(0.0), Some(60.0));
    let dureza_calcica = leer_numero_con_rango("Dureza Cálcica (ppm, ej. 250): ", Some(0.0), Some(1000.0));

    // Evaluaciones
    let estado_ph = ph::evaluar_ph(ph_actual);
    let estado_alcalinidad = alcalinidad::evaluar_alcalinidad(alcalinidad_actual);
    let resultado_isl = isl::calcular_isl(ph_actual, temp_celsius, dureza_calcica, alcalinidad_actual);

    // Empaquetamos todo en el struct DatosReporte
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

    // Imprimir el reporte
    let texto_reporte = reporte::construir_texto_reporte(&reporte_datos);
    println!("\n{}", texto_reporte);

    // Guardar en disco
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