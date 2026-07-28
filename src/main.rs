mod quimica;

use quimica::alcalinidad;
use quimica::cloro::{self, TipoCloro};
use quimica::isl;
use quimica::ph;
use quimica::reporte::{self, DatosReporte};
use quimica::volumen::Alberca;
use std::io::{self, Write};

fn leer_numero(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Error al leer");

        match entrada.trim().parse::<f64>() {
            Ok(num) if num >= 0.0 => return num,
            _ => println!("Por favor, ingresa un número válido y positivo."),
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
    let largo = leer_numero("Largo (m): ");
    let ancho = leer_numero("Ancho (m): ");
    let prof = leer_numero("Profundidad media (m): ");

    let mi_alberca = Alberca::nueva_rectangular(largo, ancho, prof);
    
    println!(
        "\n>> Alberca creada: {:.2} L ({:.2} m³)\n", 
        mi_alberca.volumen_litros, 
        mi_alberca.volumen_m3()
    );

    println!("--- 2. Parámetros Químicos Principales ---");
    let ppm_cloro_act = leer_numero("Concentración actual de cloro (ppm): ");
    let ppm_cloro_des = leer_numero("Concentración deseada de cloro (ppm): ");
    let producto_cloro = seleccionar_tipo_cloro();

    let dosis_cloro = cloro::calcular_dosis_producto(
        &mi_alberca, 
        ppm_cloro_act, 
        ppm_cloro_des, 
        producto_cloro
    );

    let ph_actual = leer_numero("\nNivel de pH actual (ej. 7.4): ");
    let alcalinidad_actual = leer_numero("Alcalinidad Total (ppm, ej. 100): ");

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