use std::io::Read;
use std::path::Path;

pub fn leer_odt(ruta: &str) -> Result<String, String> {
    let path = Path::new(ruta);
    if !path.exists() {
        return Err(format!("Archivo no encontrado: {ruta}"));
    }

    let file = std::fs::File::open(path)
        .map_err(|e| format!("Error abriendo archivo: {e}"))?;

    let mut zip = zip::ZipArchive::new(file)
        .map_err(|e| format!("Error leyendo el archivo .odt/.odf: {e}"))?;

    let mut xml_file = zip
        .by_name("content.xml")
        .map_err(|_| "No se encontró content.xml — ¿es un .odt válido?".to_string())?;

    let mut xml_contenido = String::new();
    xml_file
        .read_to_string(&mut xml_contenido)
        .map_err(|e| format!("Error leyendo content.xml: {e}"))?;

    Ok(xml_a_texto_plano(&xml_contenido))
}

fn xml_a_texto_plano(xml: &str) -> String {
    // Espacios explícitos: <text:s text:c="N"/> = N espacios, <text:s/> = 1 espacio
    let re_space_n = regex::Regex::new(r#"<text:s[^/]*/>"#).unwrap();
    let t = re_space_n.replace_all(xml, " ").to_string();
    let t = t.replace("<text:s/>", " ").replace("<text:s />", " ");

    // Tabulaciones
    let t = t.replace("<text:tab/>", "\t").replace("<text:tab />", "\t");

    // Insertar saltos de línea antes de elementos de bloque
    let t = t
        .replace("<text:p ", "\n<text:p ")
        .replace("<text:p>", "\n<text:p>")
        .replace("<text:h ", "\n<text:h ")
        .replace("<text:h>", "\n<text:h>")
        .replace("<text:list-item>", "\n<text:list-item>")
        .replace("<text:list-item ", "\n<text:list-item ")
        .replace("<table:table-row ", "\n<table:table-row ")
        .replace("<table:table-row>", "\n<table:table-row>")
        .replace("<text:line-break/>", "\n")
        .replace("<text:line-break />", "\n");

    // Quitar todas las etiquetas XML restantes
    let re_tags = regex::Regex::new(r"<[^>]+>").unwrap();
    let texto_raw = re_tags.replace_all(&t, "").to_string();

    // Decodificar entidades XML
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
