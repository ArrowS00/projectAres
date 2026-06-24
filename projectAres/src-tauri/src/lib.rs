mod commands;
mod modules;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::procesar_archivo,
            commands::extraer_texto,
            commands::guardar_resultado,
            commands::cargar_historial,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
