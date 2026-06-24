use std::path::Path;

pub fn leer_pdf(ruta: &str) -> Result<String, String> {
    let path = Path::new(ruta);
    if !path.exists() {
        return Err(format!("Archivo no encontrado: {ruta}"));
    }

    let bytes = std::fs::read(path)
        .map_err(|e| format!("Error leyendo archivo: {e}"))?;

    pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| format!("Error extrayendo texto del PDF: {e}"))
}
