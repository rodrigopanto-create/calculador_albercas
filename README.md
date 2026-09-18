# Calculador de Tratamiento de Albercas

Aplicación gráfica nativa para calcular el volumen de una alberca, consultar
parámetros de tratamiento y generar un diagnóstico químico con el Índice de
Saturación de Langelier (ISL). Está escrita en Rust con `eframe` y `egui`.

## Características

- Geometría rectangular y circular, con volumen en metros cúbicos y litros.
- Captura de pH, cloro, temperatura, dureza cálcica y alcalinidad.
- Selección de producto de cloro: tricloro o dicloro en la interfaz actual.
- Diagnóstico del equilibrio del agua mediante el ISL.
- Generación y guardado de reportes de texto en `reporte_alberca.txt`.
- Interfaz modular para escritorio y Android.

## Arquitectura

```text
CalcAlb/
├── Cargo.toml                 # Dependencias y configuración del crate
├── README.md                  # Documentación del proyecto
├── mobile.toml                # Identidad y configuración de cargo-apk
├── Trunk.toml                 # Configuración de compilación WASM
├── index.html                 # Entrada de la aplicación web
└── src/
    ├── main.rs                # Arranque de escritorio
    ├── lib.rs                 # Exportación de módulos y entradas Android/WASM
    ├── gui/
    │   ├── mod.rs             # Estado de AlbercaApp y composición de vistas
    │   ├── geometria.rs       # Forma y dimensiones de la alberca
    │   ├── parametros.rs      # Parámetros químicos y tipo de cloro
    │   └── diagnostico.rs     # Cálculo, visualización y guardado del reporte
    └── quimica/
        ├── mod.rs             # Módulos del dominio químico
        ├── volumen.rs         # Modelos y cálculo de volumen
        ├── cloro.rs           # Dosificación de productos de cloro
        ├── ph.rs              # Cálculo y clasificación del ISL
        ├── isl.rs             # Implementación auxiliar del ISL
        ├── alcalinidad.rs     # Evaluación y dosificación de alcalinidad
        └── reporte.rs         # Construcción y escritura de reportes
```

`AlbercaApp` contiene el estado editable de la interfaz. Las funciones
`ui_geometria`, `ui_parametros` y `ui_diagnostico` reciben una referencia a ese
estado y dibujan cada sección dentro del `ScrollArea` principal. La lógica de
cálculo vive en `src/quimica`, separada de los controles de `egui`.

## Requisitos

- Rust y Cargo mediante [rustup](https://rustup.rs/).
- Para Android: Android SDK, Android NDK y `cargo-apk`.
- Para web: `trunk` y el target `wasm32-unknown-unknown`.

## Uso en escritorio

```bash
cargo run --release
```

Para comprobar el proyecto sin iniciar la aplicación:

```bash
cargo check
cargo test
cargo fmt --check
```

## Compilación para Android

La configuración de Android se encuentra en `mobile.toml` y define el paquete
`com.rodrigo.calc_alb`. Instala el target ARM64 y `cargo-apk` antes de compilar:

```bash
rustup target add aarch64-linux-android
cargo install cargo-apk
cargo apk build --release
```

El APK generado queda dentro de `target/` en la salida de `cargo-apk`. Para
instalarlo en un dispositivo conectado con ADB:

```bash
adb install -r target/release/apk/calculador_albercas.apk
```

La ruta exacta puede variar según la versión de `cargo-apk` y el target activo.

## Compilación web

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve
```

Después abre la dirección local mostrada por Trunk en el navegador.

## Reportes

El diagnóstico se genera desde la sección **Diagnóstico y Reporte**. El botón
**Guardar reporte** escribe el contenido en `reporte_alberca.txt` usando el
directorio de trabajo de la aplicación.