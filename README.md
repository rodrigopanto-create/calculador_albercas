# 🏊‍♂️ Calculadora de Tratamiento Químico para Albercas v1.0.0

Una aplicación gráfica (GUI) de escritorio escrita en **Rust** usando **`eframe`** y **`egui`**, diseñada para el cálculo exacto de volumen, dosificación de productos químicos y la evaluación del **Índice de Saturación de Langelier (ISL)** para el mantenimiento profesional de piscinas.

Proporciona una alternativa libre, de código abierto (FOSS), nativa y ultra rápida frente a software comercial o apps con suscripción.

---

## 🚀 Características

- 🖥️ **Interfaz Gráfica Nativa (GUI):** Construida en modo inmediato (*Immediate Mode*) con `egui`.
- 📐 **Geometría y Volumen:** Soporte para albercas rectangulares y circulares con cálculo automático en metros cúbicos ($m^3$) y litros ($L$).
- 🧪 **Ajuste Químico Exacto:** Dosificación personalizada según el tipo de producto (Tricloro al 90%, Dicloro al 56%, etc.).
- ⚖️ **Índice de Saturación de Langelier (ISL):** Diagnóstico dinámico sobre la tendencia del agua (corrosiva, balanceada o incrustante) considerando pH, temperatura, dureza cálcica y alcalinidad.
- 📄 **Exportación de Reportes:** Generación e impresión de diagnósticos técnicos en archivos de texto `.txt`.
- 🏗️ **Arquitectura Modular:** Lógica de negocio (física/química) 100% desacoplada de la capa de presentación.

---

## 🛠️ Estructura del Proyecto

```text
calculador_albercas/
├── Cargo.toml               # Configuración del crate y dependencias (eframe/egui)
├── README.md                # Documentación del proyecto
├── .gitignore               # Exclusión de target/ y binarios compilados
│
└── src/
    ├── main.rs              # Punto de entrada ejecutable (Inicializa eframe/egui)
    ├── lib.rs               # Exportador principal del crate y módulos
    │
    ├── gui.rs               # Interfaz Gráfica de Usuario (AlbercaApp, Sliders y Eventos)
    │
    └── quimica/             # Módulo de lógica de negocio (Backend / Dominio)
        ├── mod.rs           # Re-exportador de submódulos químicos
        ├── volumen.rs       # Geometría y cálculo volumétrico
        ├── ph.rs            # Cálculo del Índice de Saturación de Langelier (ISL)
        ├── cloro.rs         # Cálculo de dosificación por producto químico
        └── reporte.rs       # Generación de reportes y persistencia I/O (std::fs)

 ## 🛠️ Instalación y Uso


Asegúrate de tener [Rust y Cargo](https://www.rust-lang.org/) instalados en tu sistema

# 1. Clonar el repositorio
git clone [https://github.com/rodrigopanto-create/calculador_albercas.git](https://github.com/rodrigopanto-create/calculador_albercas.git)

# 2. Entrar a la carpeta del proyecto
cd calculador_albercas

# 3. Compilar y ejecutar la interfaz gráfica
cargo run --release