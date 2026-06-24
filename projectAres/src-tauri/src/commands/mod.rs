use tauri::{AppHandle, Manager};
use serde_json::Value;
use crate::modules::{pdf_reader, docx_reader, odt_reader, parser, storage};

#[tauri::command]
pub fn procesar_archivo(ruta: String) -> Result<Value, String> {
    let ruta_lower = ruta.to_lowercase();
    let texto = if ruta_lower.ends_with(".pdf") {
        pdf_reader::leer_pdf(&ruta)?
    } else if ruta_lower.ends_with(".docx") {
        docx_reader::leer_docx(&ruta)?
    } else if ruta_lower.ends_with(".doc") {
        return Err("El formato .doc (Word 97-2003) no está soportado directamente.\n\nÁbrelo en Word o LibreOffice y guárdalo como .docx, luego vuelve a intentarlo.".to_string());
    } else if ruta_lower.ends_with(".odt") || ruta_lower.ends_with(".odf") {
        odt_reader::leer_odt(&ruta)?
    } else {
        return Err("Formato no soportado. Usa PDF, .docx o .odt".to_string());
    };

    let nombre = ruta.rsplit('/').next().unwrap_or(&ruta);
    let resultado = parser::parsear_documento(&texto, nombre).map_err(|e| {
        let preview: String = texto.lines().take(20).collect::<Vec<_>>().join("\n");
        format!("{e}\n\n--- Texto extraído (primeras 20 líneas) ---\n{preview}")
    })?;
    serde_json::to_value(resultado).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn extraer_texto(ruta: String) -> Result<String, String> {
    let ruta_lower = ruta.to_lowercase();
    if ruta_lower.ends_with(".pdf") {
        pdf_reader::leer_pdf(&ruta)
    } else if ruta_lower.ends_with(".docx") {
        docx_reader::leer_docx(&ruta)
    } else if ruta_lower.ends_with(".odt") || ruta_lower.ends_with(".odf") {
        odt_reader::leer_odt(&ruta)
    } else {
        Err("Formato no soportado".to_string())
    }
}

#[tauri::command]
pub fn guardar_resultado(
    app: AppHandle,
    titulo: String,
    total: u32,
    correctas: u32,
    incorrectas: u32,
) -> Result<i64, String> {
    let data_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .to_string();

    let conn = storage::conectar(&data_dir).map_err(|e| e.to_string())?;

    let porcentaje = if total > 0 { (correctas as f64 / total as f64) * 100.0 } else { 0.0 };
    let fecha = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();

    let resultado = storage::ResultadoTest {
        id: None, titulo, fecha, total, correctas, incorrectas, porcentaje,
    };

    storage::guardar_resultado(&conn, &resultado).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cargar_historial(app: AppHandle) -> Result<Value, String> {
    let data_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .to_string();

    let conn = storage::conectar(&data_dir).map_err(|e| e.to_string())?;
    let historial = storage::cargar_historial(&conn).map_err(|e| e.to_string())?;
    serde_json::to_value(historial).map_err(|e| e.to_string())
}
