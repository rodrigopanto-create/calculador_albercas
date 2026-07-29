# 🏊 Calculadora de Tratamiento Químico para Albercas v1.0

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
├── .gitignore               # Ignora /target y binarios de compilación
├── Cargo.toml               # Dependencias (eframe/egui) y metadata del crate
├── Cargo.lock               # Árbol exacto de dependencias bloqueadas
├── README.md                # Documentación del proyecto
├── reporte_alberca.txt      # Reporte generado (salida de prueba local)
│
└── src/
    ├── main.rs              # Punto de entrada principal (Lanza la GUI eframe)
    ├── lib.rs               # Exportador de módulos principales (gui y quimica)
    │
    ├── gui.rs               # Interfaz Gráfica de Usuario (Immediate Mode con egui)
    │                        └── AlbercaApp (State, Sliders, Formulario y Render)
    │
    └── quimica/             # Módulo de lógica de negocio (Backend / Dominio)
        ├── mod.rs           # Re-exporta volumen, ph, cloro y reporte
        ├── volumen.rs       # Geometría (Alberca::Rectangular / Circular)
        ├── ph.rs            # Cálculo del Índice de Saturación de Langelier (ISL)
        ├── cloro.rs         # Dosificación por tipo (Tricloro vs Dicloro)
        └── reporte.rs       # Formateador de diagnósticos y persitencia (std::fs)

