#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub mod domains;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            domains::todos::controller::create,
            domains::todos::controller::find,
            domains::todos::controller::find_all,
            domains::todos::controller::update,
            domains::todos::controller::delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
