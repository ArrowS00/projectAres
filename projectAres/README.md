projectAres

# ProjectAres
> Aplicación de escritorio para practicar tests de oposición a partir de documentos PDF y Word. Sin publicidad, sin internet, completamente gratuita.

---

## ¿Qué es ProjectZero?

ProjectZero lee tus documentos de test (PDF o Word), detecta automáticamente las preguntas, opciones y clave de respuestas, y te permite practicar con corrección inmediata. Todo funciona localmente en tu ordenador — ningún dato sale al exterior.

### Características

- Carga documentos PDF y Word (.docx)
- Detecta preguntas numeradas y sus opciones automáticamente
- Lee la clave de respuestas al final del documento
- Corrige cada respuesta al momento e indica cuál era la correcta
- Guarda un historial de todos tus tests con tu puntuación
- Funciona sin internet y sin cuenta

---

## Para usuarios — Instalación

### Requisitos

- Windows 10 o superior (64 bits)

### Pasos

1. Ve a la sección [Releases](../../releases) de este repositorio
2. Descarga el archivo `ProjectZero_x.x.x_x64-setup.exe`
3. Ejecuta el instalador y sigue los pasos
4. Abre ProjectZero desde el menú inicio o el acceso directo del escritorio

No necesitas instalar nada más.

### Formato de los documentos

Para que ProjectZero detecte las preguntas correctamente, el documento debe seguir este formato:

```
1. Texto de la pregunta
a) Opción A
b) Opción B
c) Opción C
d) Opción D

2. Texto de la segunda pregunta
a) Opción A
...

SOLUCIONES
1. C
2. A
...
```

El bloque de soluciones puede llamarse `SOLUCIONES`, `RESPUESTAS CORRECTAS`, `CLAVE` o `CONTESTACIONES`.

---

## Para desarrolladores

### Tecnologías

- [Tauri 2](https://tauri.app/) — framework de escritorio
- [React](https://react.dev/) + TypeScript — interfaz de usuario
- [Rust](https://www.rust-lang.org/) — lógica de negocio y lectura de documentos

### Requisitos previos

- [Node.js](https://nodejs.org/) 18 o superior
- [Rust](https://rustup.rs/) (edición estable)
- Microsoft Visual Studio Build Tools 2022 con el workload **"Desarrollo para el escritorio con C++"**
- [Git](https://git-scm.com/)

### Instalación del entorno

```bash
# Clonar el repositorio
git clone https://github.com/tuusuario/projectzero.git
cd projectzero

# Instalar dependencias Node
npm install

# Arrancar en modo desarrollo
npm run tauri dev
```

### Estructura del proyecto

```
projectzero/
├── src/                          # Frontend React
│   ├── views/
│   │   ├── UploadView.tsx        # Pantalla de carga de documento
│   │   ├── TestView.tsx          # Pantalla de test
│   │   ├── ResultsView.tsx       # Pantalla de resultados
│   │   └── HistorialView.tsx     # Historial de tests
│   ├── App.tsx
│   └── types.ts
└── src-tauri/
    └── src/
        ├── commands/             # Comandos Tauri (puente UI ↔ Rust)
        │   └── mod.rs
        ├── modules/              # Módulos independientes
        │   ├── pdf_reader.rs     # Lectura de PDFs
        │   ├── docx_reader.rs    # Lectura de Word
        │   ├── parser.rs         # Extracción de preguntas
        │   └── storage.rs        # Historial en SQLite
        └── lib.rs
```

### Añadir un módulo nuevo

1. Crea el archivo en `src-tauri/src/modules/nuevo_modulo.rs`
2. Decláralo en `src-tauri/src/modules/mod.rs`:
   ```rust
   pub mod nuevo_modulo;
   ```
3. Si necesita un comando Tauri, añádelo en `src-tauri/src/commands/mod.rs`
4. Registra el comando en `src-tauri/src/lib.rs` dentro de `generate_handler![]`

### Generar el instalador

```bash
npm run tauri build
```

El instalador `.exe` se genera en `src-tauri/target/release/bundle/nsis/`.

### Tests unitarios (Rust)

```bash
cd src-tauri
cargo test
```

---

## Contribuir

Las contribuciones son bienvenidas. Abre un issue para reportar bugs o proponer mejoras, o un pull request directamente si ya tienes la solución.

---

## Licencia

MIT — puedes usar, modificar y distribuir este proyecto libremente.

