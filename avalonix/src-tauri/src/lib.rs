use avalonix_api::audio::media_player::MediaPlayer;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
//#[cfg(not(test))]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
