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
pub fn registrar_test_iniciado(
    app: AppHandle,
    titulo: String,
    total: u32,
    datos_test: String,
) -> Result<i64, String> {
    let data_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .to_string();

    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    let conn = storage::conectar(&data_dir).map_err(|e| e.to_string())?;
    let fecha = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();

    let resultado = storage::ResultadoTest {
        id: None, titulo, fecha, total,
        correctas: 0, incorrectas: 0, porcentaje: 0.0,
        estado: "iniciado".to_string(),
        datos_test: Some(datos_test),
    };

    storage::guardar_resultado(&conn, &resultado).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cargar_test_desde_historial(app: AppHandle, id: i64) -> Result<Value, String> {
    let data_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .to_string();

    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    let conn = storage::conectar(&data_dir).map_err(|e| e.to_string())?;
    let json = storage::cargar_datos_test(&conn, id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Test sin datos guardados".to_string())?;

    serde_json::from_str(&json).map_err(|e| e.to_string())
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

    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    let conn = storage::conectar(&data_dir).map_err(|e| e.to_string())?;

    let porcentaje = if total > 0 { (correctas as f64 / total as f64) * 100.0 } else { 0.0 };
    let fecha = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();

    let resultado = storage::ResultadoTest {
        id: None, titulo, fecha, total, correctas, incorrectas, porcentaje,
        estado: "completado".to_string(),
        datos_test: None,
    };

    storage::guardar_resultado(&conn, &resultado).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn limpiar_historial(app: AppHandle) -> Result<(), String> {
    let data_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .to_string();

    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    let conn = storage::conectar(&data_dir).map_err(|e| e.to_string())?;
    storage::limpiar_historial(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cargar_historial(app: AppHandle) -> Result<Value, String> {
    let data_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .to_string();

    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    let conn = storage::conectar(&data_dir).map_err(|e| e.to_string())?;
    let historial = storage::cargar_historial(&conn).map_err(|e| e.to_string())?;
    serde_json::to_value(historial).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn crear_test_mezclado(app: AppHandle, por_test: u32) -> Result<Value, String> {
    let data_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .to_string();

    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    let conn = storage::conectar(&data_dir).map_err(|e| e.to_string())?;
    let datos_json = storage::obtener_datos_para_mezcla(&conn).map_err(|e| e.to_string())?;

    let resultado = construir_test_mezclado(datos_json, por_test)?;
    serde_json::to_value(resultado).map_err(|e| e.to_string())
}

fn construir_test_mezclado(datos_json: Vec<String>, por_test: u32) -> Result<parser::ResultadoParser, String> {
    if datos_json.len() < 2 {
        return Err("Necesitas al menos 2 tests distintos en el historial para crear una mezcla.".to_string());
    }

    let mut preguntas_mezcladas: Vec<parser::Pregunta> = Vec::new();
    let mut num_tests = 0;

    for json in &datos_json {
        let test: parser::ResultadoParser = match serde_json::from_str(json) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let mut preguntas = test.preguntas;
        barajar(&mut preguntas);
        preguntas.truncate(por_test as usize);
        if !preguntas.is_empty() {
            num_tests += 1;
            preguntas_mezcladas.extend(preguntas);
        }
    }

    if preguntas_mezcladas.is_empty() {
        return Err("Los tests del historial no tienen preguntas disponibles.".to_string());
    }

    barajar(&mut preguntas_mezcladas);
    for (i, p) in preguntas_mezcladas.iter_mut().enumerate() {
        p.num = (i + 1) as u32;
    }

    let con_clave = preguntas_mezcladas.iter().any(|p| p.correcta.is_some());
    let total = preguntas_mezcladas.len();

    Ok(parser::ResultadoParser {
        titulo: format!("Test mezclado ({num_tests} tests)"),
        preguntas: preguntas_mezcladas,
        total,
        con_clave,
    })
}

/// Baraja un vector in-place usando claves de hash aleatorias (sin dependencias externas).
fn barajar<T>(v: &mut Vec<T>) {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    let estado = RandomState::new();
    let mut orden: Vec<(u64, usize)> = (0..v.len())
        .map(|i| {
            let mut hasher = estado.build_hasher();
            hasher.write_usize(i);
            (hasher.finish(), i)
        })
        .collect();
    orden.sort_by_key(|&(h, _)| h);

    let mut restante: Vec<Option<T>> = v.drain(..).map(Some).collect();
    let mut nuevo = Vec::with_capacity(restante.len());
    for (_, i) in orden {
        nuevo.push(restante[i].take().unwrap());
    }
    *v = nuevo;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_json(titulo: &str, n_preguntas: u32) -> String {
        let preguntas: Vec<parser::Pregunta> = (1..=n_preguntas).map(|i| parser::Pregunta {
            num: i,
            enunciado: format!("Pregunta {i} de {titulo}"),
            opciones: vec![
                parser::Opcion { letra: "A".into(), texto: "uno".into() },
                parser::Opcion { letra: "B".into(), texto: "dos".into() },
            ],
            correcta: Some("A".into()),
        }).collect();
        let r = parser::ResultadoParser {
            titulo: titulo.to_string(),
            total: preguntas.len(),
            preguntas,
            con_clave: true,
        };
        serde_json::to_string(&r).unwrap()
    }

    #[test]
    fn mezcla_coge_hasta_por_test_de_cada_test() {
        let datos = vec![test_json("Test A", 15), test_json("Test B", 20)];
        let resultado = construir_test_mezclado(datos, 10).unwrap();
        assert_eq!(resultado.total, 20);
        assert_eq!(resultado.preguntas.len(), 20);
        assert!(resultado.con_clave);
        // renumeradas de forma secuencial 1..=20
        let nums: Vec<u32> = resultado.preguntas.iter().map(|p| p.num).collect();
        assert_eq!(nums, (1..=20).collect::<Vec<_>>());
    }

    #[test]
    fn mezcla_respeta_tests_con_menos_preguntas_que_por_test() {
        let datos = vec![test_json("Test A", 3), test_json("Test B", 10)];
        let resultado = construir_test_mezclado(datos, 10).unwrap();
        assert_eq!(resultado.total, 13);
    }

    #[test]
    fn mezcla_falla_con_menos_de_dos_tests() {
        let datos = vec![test_json("Test A", 10)];
        let err = construir_test_mezclado(datos, 10).unwrap_err();
        assert!(err.contains("al menos 2 tests"));
    }

    #[test]
    fn barajar_mantiene_todos_los_elementos() {
        let mut v: Vec<i32> = (0..50).collect();
        barajar(&mut v);
        let mut ordenado = v.clone();
        ordenado.sort();
        assert_eq!(ordenado, (0..50).collect::<Vec<_>>());
    }
}
