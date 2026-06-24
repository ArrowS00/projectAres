use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opcion {
    pub letra: String,
    pub texto: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pregunta {
    pub num: u32,
    pub enunciado: String,
    pub opciones: Vec<Opcion>,
    pub correcta: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultadoParser {
    pub titulo: String,
    pub preguntas: Vec<Pregunta>,
    pub total: usize,
    pub con_clave: bool,
}

pub fn parsear_documento(texto: &str, nombre_archivo: &str) -> Result<ResultadoParser, String> {
    let texto_limpio = limpiar_texto(texto);
    let (cuerpo, clave_map) = separar_clave(&texto_limpio);
    let preguntas_raw = extraer_preguntas(&cuerpo)?;

    let preguntas = if clave_map.is_empty() {
        buscar_respuesta_inline(preguntas_raw)
    } else {
        aplicar_clave(preguntas_raw, &clave_map)
    };

    let con_clave = preguntas.iter().any(|p| p.correcta.is_some());
    let titulo = inferir_titulo(&texto_limpio, nombre_archivo);
    let total = preguntas.len();

    Ok(ResultadoParser { titulo, preguntas, total, con_clave })
}

fn limpiar_texto(texto: &str) -> String {
    let t = texto.replace("\r\n", "\n").replace('\r', "\n");
    let re_decorativa = Regex::new(r"(?m)^[\s\-_=*]{5,}$").unwrap();
    let t = re_decorativa.replace_all(&t, "").to_string();
    let re_blancos = Regex::new(r"\n{4,}").unwrap();
    let t = re_blancos.replace_all(&t, "\n\n").to_string();
    separar_si_concatenado(&t)
}

// Si el texto viene todo en líneas muy largas (p.ej. ODT sin saltos de párrafo),
// inserta saltos antes de cada número de pregunta y letra de opción con formato ".-"
fn separar_si_concatenado(texto: &str) -> String {
    let hay_linea_larga = texto.lines().any(|l| l.len() > 200);
    if !hay_linea_larga {
        return texto.to_string();
    }
    let re_q = Regex::new(r" (\d{1,3})\.-\s").unwrap();
    let re_o = Regex::new(r" ([a-dA-D])\.-\s").unwrap();
    let t = re_q.replace_all(texto, "\n$1.- ").to_string();
    re_o.replace_all(&t, "\n$1.- ").to_string()
}

fn separar_clave(texto: &str) -> (String, HashMap<u32, String>) {
    let mut clave_map: HashMap<u32, String> = HashMap::new();

    let re_inicio_clave = Regex::new(
        r"(?im)^(soluciones?|respuestas?(\s*correctas?)?|clave\s*de\s*respuestas?|contestaciones?|answers?)[\s:.]*$"
    ).unwrap();

    if let Some(m) = re_inicio_clave.find(texto) {
        let cuerpo = texto[..m.start()].to_string();
        let bloque_clave = &texto[m.end()..];

        // Formato numerado: "1. A", "2.- B", "10) C", etc.
        let re_entrada = Regex::new(r"(?m)(\d{1,3})[\.\)\-:\s]+([A-Da-d])\b").unwrap();
        for cap in re_entrada.captures_iter(bloque_clave) {
            if let Ok(num) = cap[1].parse::<u32>() {
                clave_map.insert(num, cap[2].to_uppercase());
            }
        }

        // Formato posicional: una letra sola por línea (línea 1 = pregunta 1, etc.)
        if clave_map.is_empty() {
            let re_solo_letra = Regex::new(r"(?m)^\s*([A-Da-d])\s*$").unwrap();
            let mut num: u32 = 1;
            for cap in re_solo_letra.captures_iter(bloque_clave) {
                clave_map.insert(num, cap[1].to_uppercase());
                num += 1;
            }
        }

        return (cuerpo, clave_map);
    }

    (texto.to_string(), clave_map)
}

fn extraer_preguntas(texto: &str) -> Result<Vec<Pregunta>, String> {
    let re_inicio = Regex::new(r"^(\d{1,3})[\.\)]\-?\s*(.+)").unwrap();
    let re_opcion = Regex::new(r"^\s*([A-Da-d])[\.\)]\s*\-?\s*(.+)").unwrap();
    let re_num_inicial = Regex::new(r"^\d{1,3}[\.\)]\-?\s*").unwrap();

    // Agrupa líneas en bloques: cada bloque empieza con una línea de pregunta numerada
    let lineas: Vec<&str> = texto.lines().collect();
    let mut bloques: Vec<Vec<&str>> = Vec::new();
    let mut bloque_actual: Vec<&str> = Vec::new();

    for linea in &lineas {
        if re_inicio.is_match(linea) && !bloque_actual.is_empty() {
            bloques.push(bloque_actual.clone());
            bloque_actual = vec![linea];
        } else if re_inicio.is_match(linea) {
            bloque_actual = vec![linea];
        } else if !bloque_actual.is_empty() {
            bloque_actual.push(linea);
        }
    }
    if !bloque_actual.is_empty() {
        bloques.push(bloque_actual);
    }

    let mut preguntas: Vec<Pregunta> = Vec::new();

    for bloque in &bloques {
        let primera = bloque[0];
        let caps = match re_inicio.captures(primera) {
            Some(c) => c,
            None => continue,
        };
        let num: u32 = caps[1].parse().unwrap_or(0);

        let mut enunciado_lineas: Vec<&str> = Vec::new();
        let mut opciones: Vec<Opcion> = Vec::new();
        let mut en_opciones = false;

        for linea in bloque {
            if re_opcion.is_match(linea) {
                en_opciones = true;
            }
            if en_opciones {
                if let Some(c) = re_opcion.captures(linea) {
                    opciones.push(Opcion {
                        letra: c[1].to_uppercase(),
                        texto: c[2].trim().to_string(),
                    });
                } else if let Some(ultima) = opciones.last_mut() {
                    let cont = linea.trim();
                    if !cont.is_empty() {
                        ultima.texto.push(' ');
                        ultima.texto.push_str(cont);
                    }
                }
            } else {
                enunciado_lineas.push(linea);
            }
        }

        let enunciado_raw = enunciado_lineas.join(" ");
        let enunciado = re_num_inicial.replace(&enunciado_raw, "").trim().to_string();

        if enunciado.is_empty() || opciones.is_empty() {
            continue;
        }

        preguntas.push(Pregunta { num, enunciado, opciones, correcta: None });
    }

    if preguntas.is_empty() {
        return Err("No se encontraron preguntas. Comprueba el formato del documento.".to_string());
    }

    preguntas.sort_by_key(|p| p.num);
    Ok(preguntas)
}

fn aplicar_clave(mut preguntas: Vec<Pregunta>, clave: &HashMap<u32, String>) -> Vec<Pregunta> {
    for p in &mut preguntas {
        if let Some(letra) = clave.get(&p.num) {
            p.correcta = Some(letra.clone());
        }
    }
    preguntas
}

fn buscar_respuesta_inline(mut preguntas: Vec<Pregunta>) -> Vec<Pregunta> {
    let re_marcador = Regex::new(r"(?i)\*{1,2}([A-D])\*{1,2}|\[([A-D])\]|\(correcta?\)").unwrap();
    for p in &mut preguntas {
        for opcion in &p.opciones {
            if re_marcador.is_match(&opcion.texto) || opcion.texto.to_lowercase().contains("correcta") {
                p.correcta = Some(opcion.letra.clone());
                break;
            }
        }
    }
    preguntas
}

fn inferir_titulo(texto: &str, nombre_archivo: &str) -> String {
    let primeras: Vec<&str> = texto
        .lines()
        .filter(|l| !l.trim().is_empty())
        .take(5)
        .collect();

    if let Some(primera) = primeras.first() {
        let primera = primera.trim();
        let re_no_es_pregunta = Regex::new(r"^\d{1,3}[\.\)]").unwrap();
        if primera.len() < 120 && !re_no_es_pregunta.is_match(primera) {
            return primera.to_string();
        }
    }

    nombre_archivo
        .rsplit('/')
        .next()
        .unwrap_or(nombre_archivo)
        .trim_end_matches(".pdf")
        .trim_end_matches(".docx")
        .trim_end_matches(".doc")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parseo_con_clave() {
        let texto = r#"
Test de ejemplo

1. ¿Cuál es la capital de España?
a) Barcelona
b) Madrid
c) Valencia
d) Sevilla

2. ¿En qué año se aprobó la Constitución española?
a) 1975
b) 1977
c) 1978
d) 1982

SOLUCIONES
1. B
2. C
"#;
        let r = parsear_documento(texto, "test.pdf").unwrap();
        assert_eq!(r.preguntas.len(), 2);
        assert!(r.con_clave);
        assert_eq!(r.preguntas[0].correcta.as_deref(), Some("B"));
        assert_eq!(r.preguntas[1].correcta.as_deref(), Some("C"));
    }
}
