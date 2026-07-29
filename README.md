# 🏊 Calculadora de Tratamiento Químico para Albercas v0.7

Aplicación de consola escrita en **Rust** diseñada para calcular dosis de químicos, evaluar el estado del agua y calcular el **Índice de Saturación de Langelier (ISL)** para el mantenimiento profesional de piscinas.

---

# 🏊‍♂️ Calculadora de Albercas (FOSS)

Una herramienta de consola escrita en **Rust** para el cálculo exacto de volumen, dosificación de productos químicos y el **Índice de Langelier (ISL)** en albercas.

Proporciona una alternativa libre, de código abierto (FOSS) y sin paywalls a las aplicaciones comerciales.

---

## 🚀 Características

- 📐 **Cálculo de Volumen:** Convierte dimensiones a litros ($L$) y metros cúbicos ($m^3$).
- 🧪 **Ajuste Químico:** Dosificación exacta para Cloro Líquido (12%), Tricloro (90%), Dicloro (56%) e Hipoclorito de Calcio (65%).
- ⚖️ **Índice de Sat. de Langelier (ISL):** Evaluación de agua corrosiva o incrustante contemplando pH, temperatura, dureza cálcica y alcalinidad.
- 📄 **Generación de Reportes:** Exportación de diagnósticos en archivos `.txt`.

---

## 🛠️ Instalación y Uso

Asegúrate de tener [Rust y Cargo](https://www.rust-lang.org/) instalados en tu sistema.

```bash
# Clonar el repositorio
git clone [https://github.com/rodrigopanto-create/calculador_albercas.git](https://github.com/rodrigopanto-create/calculador_albercas.git)

# Entrar a la carpeta
cd calculador_albercas

# Compilar y ejecutar
cargo run

---

## 🛠️ Arquitectura del Proyecto

El proyecto sigue una arquitectura modular en Rust:

```text
calcalb/
├── src/
│   ├── main.rs              # Punto de entrada y flujo de CLI
│   ├── lib.rs               # Exportación de biblioteca
│   └── quimica/             # Módulo principal de lógica de negocio
│       ├── mod.rs           # Exposición de submódulos
│       ├── volumen.rs       # Modelado geométrico de albercas
│       ├── cloro.rs         # Lógica de dosificación por producto
│       ├── ph.rs            # Evaluación y ajustes de pH
│       ├── alcalinidad.rs   # Evaluación de buffer de alcalinidad
│       ├── isl.rs           # Cálculo del Índice de Langelier
│       └── reporte.rs       # Formateo DTO y exportación I/O
├── Cargo.toml               # Configuración del proyecto y dependencias
└── .gitignore               # Exclusión de binarios y temporales

