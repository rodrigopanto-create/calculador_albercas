use std::io::{self, Write};

use calculador_albercas::quimica::{
    cloro::{self, TipoCloro},
    reporte::{generar_reporte, guardar_reporte_en_archivo, DatosReporte},
    volumen::Alberca,
};

fn main() -> Result<(), &'static str> {
    println!("==================================================");
    println!("   CALCULADOR ESTRUCTURADO DE ALBERCAS (CLI)      ");
    println!("==================================================\n");

    // 1. Selección de Forma
    println!("Selecciona el tipo de alberca:");
    println!("  1. Rectangular");
    println!("  2. Circular");
    let opcion_tipo = leer_linea("Ingresa 1 o 2: ");

    let alberca = match opcion_tipo.trim() {
        "1" => {
            let largo = leer_f64("Ingresa el Largo (metros): ");
            let ancho = leer_f64("Ingresa el Ancho (metros): ");
            let profundidad = leer_f64("Ingresa la Profundidad Media (metros): ");
            Alberca::nueva_rectangular(largo, ancho, profundidad)?
        }
        "2" => {
            let diametro = leer_f64("Ingresa el Diámetro (metros): ");
            let profundidad = leer_f64("Ingresa la Profundidad Media (metros): ");
            Alberca::nueva_circular(diametro, profundidad)?
        }
        _ => return Err("Opción de alberca no válida. Debe ser 1 o 2."),
    };

    println!("\n--- PARÁMETROS DEL AGUA ---");
    let ph_actual = leer_f64("pH actual (ej. 7.4): ");
    let cloro_actual = leer_f64("Cloro libre actual (ppm): ");
    let cloro_objetivo = leer_f64("Cloro libre objetivo (ppm): ");
    let temperatura_c = leer_f64("Temperatura (°C): ");
    let dureza_calcio_ppm = leer_f64("Dureza por Calcio (ppm): ");
    let alcalinidad_ppm = leer_f64("Alcalinidad Total (ppm): ");

    println!("\nSelecciona el producto clorante a usar:");
    println!("  1. Tricloro");
    println!("  2. Dicloro");
    let opcion_cloro = leer_linea("Ingresa 1 o 2: ");

    let tipo_cloro = match opcion_cloro.trim() {
        "1" => TipoCloro::Tricloro,
        "2" => TipoCloro::Dicloro,
        _ => return Err("Opción de producto de cloro no válida."),
    };

    // 2. Cálculos y Salida
    println!("\n==================================================");
    println!("                 RESULTADOS                       ");
    println!("==================================================");

    let dosis = cloro::calcular_dosis_producto(
        &alberca,
        cloro_actual,
        cloro_objetivo,
        tipo_cloro,
    );

    let datos = DatosReporte {
        alberca,
        ph_actual,
        cloro_actual,
        temperatura_c,
        dureza_calcio_ppm,
        alcalinidad_ppm,
    };

    let reporte_texto = generar_reporte(&datos);
    let resumen_dosificacion = format!(
        "--- DOSIFICACIÓN RECOMENDADA ---\nPara ajustar de {:.1} ppm a {:.1} ppm requiere:\n-> {:.2} {} de producto químico.\n",
        cloro_actual, cloro_objetivo, dosis.cantidad, dosis.unidad
    );

    // Imprimir en consola
    println!("{}", reporte_texto);
    println!("{}", resumen_dosificacion);

    // 3. Opción de Guardado en Archivo
    let guardar = leer_linea("¿Deseas guardar este reporte en un archivo .txt? (s/n): ");
    if guardar.trim().eq_ignore_ascii_case("s") {
        let nombre_archivo = "reporte_alberca.txt";
        let contenido_completo = format!("{}\n{}", reporte_texto, resumen_dosificacion);

        match guardar_reporte_en_archivo(nombre_archivo, &contenido_completo) {
            Ok(_) => println!("  📄 ¡Reporte guardado exitosamente en '{}'!", nombre_archivo),
            Err(e) => eprintln!("  ❌ Error al guardar el archivo: {}", e),
        }
    }

    Ok(())
}

/// Lee una cadena de texto estándar desde la terminal mostrando un prompt previo.
fn leer_linea(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error al leer desde la terminal");
    buffer
}

/// Convierte la entrada del usuario a un valor numérico f64 flotante.
fn leer_f64(prompt: &str) -> f64 {
    loop {
        let texto = leer_linea(prompt);
        match texto.trim().parse::<f64>() {
            Ok(num) if num >= 0.0 => return num,
            _ => println!("  ⚠️ Entrada inválida. Ingresa un número positivo válido."),
        }
    }
}
