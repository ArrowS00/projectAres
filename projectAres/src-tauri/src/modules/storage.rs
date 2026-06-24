use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

const DB_NAME: &str = "testapp.db";

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultadoTest {
    pub id: Option<i64>,
    pub titulo: String,
    pub fecha: String,
    pub total: u32,
    pub correctas: u32,
    pub incorrectas: u32,
    pub porcentaje: f64,
}

pub fn conectar(data_dir: &str) -> Result<Connection> {
    let ruta = format!("{}/{}", data_dir, DB_NAME);
    let conn = Connection::open(&ruta)?;
    crear_tablas(&conn)?;
    Ok(conn)
}

fn crear_tablas(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS resultados (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo      TEXT    NOT NULL,
            fecha       TEXT    NOT NULL,
            total       INTEGER NOT NULL,
            correctas   INTEGER NOT NULL,
            incorrectas INTEGER NOT NULL,
            porcentaje  REAL    NOT NULL
        );"
    )
}

pub fn guardar_resultado(conn: &Connection, r: &ResultadoTest) -> Result<i64> {
    conn.execute(
        "INSERT INTO resultados (titulo, fecha, total, correctas, incorrectas, porcentaje)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![r.titulo, r.fecha, r.total, r.correctas, r.incorrectas, r.porcentaje],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn cargar_historial(conn: &Connection) -> Result<Vec<ResultadoTest>> {
    let mut stmt = conn.prepare(
        "SELECT id, titulo, fecha, total, correctas, incorrectas, porcentaje
         FROM resultados ORDER BY fecha DESC LIMIT 50"
    )?;

    let resultados = stmt.query_map([], |row| {
        Ok(ResultadoTest {
            id:          Some(row.get(0)?),
            titulo:      row.get(1)?,
            fecha:       row.get(2)?,
            total:       row.get(3)?,
            correctas:   row.get(4)?,
            incorrectas: row.get(5)?,
            porcentaje:  row.get(6)?,
        })
    })?.collect::<Result<Vec<_>>>()?;

    Ok(resultados)
}
