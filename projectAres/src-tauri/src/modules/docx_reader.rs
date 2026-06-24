use std::io::Read;
use std::path::Path;

pub fn leer_docx(ruta: &str) -> Result<String, String> {
    let path = Path::new(ruta);
    if !path.exists() {
        return Err(format!("Archivo no encontrado: {ruta}"));
    }

    let file = std::fs::File::open(path)
        .map_err(|e| format!("Error abriendo archivo: {e}"))?;

    let mut zip = zip::ZipArchive::new(file)
        .map_err(|e| format!("Error leyendo ZIP del .docx: {e}"))?;

    let mut xml_file = zip
        .by_name("word/document.xml")
        .map_err(|_| "No se encontró word/document.xml — ¿es un .docx válido?".to_string())?;

    let mut xml_contenido = String::new();
    xml_file
        .read_to_string(&mut xml_contenido)
        .map_err(|e| format!("Error leyendo document.xml: {e}"))?;

    Ok(xml_a_texto_plano(&xml_contenido))
}

fn xml_a_texto_plano(xml: &str) -> String {
    let con_saltos = xml
        .replace("<w:p ", "\n<w:p ")
        .replace("<w:p>", "\n<w:p>")
        .replace("<w:br/>", "\n")
        .replace("<w:br />", "\n");

    let re = regex::Regex::new(r"<[^>]+>").unwrap();
    let texto_raw = re.replace_all(&con_saltos, "").to_string();

    let texto = texto_raw
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&apos;", "'")
        .replace("&quot;", "\"")
        .replace("&#xD;", "\n");

    let re_blancos = regex::Regex::new(r"\n{3,}").unwrap();
    re_blancos.replace_all(&texto, "\n\n").trim().to_string()
}
