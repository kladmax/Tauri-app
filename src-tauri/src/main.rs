use std::process::Command;
use tauri::Manager;

#[tauri::command]
fn run_hibernate() -> Result<String, String> {
    let bat_file = "hibernate.bat";
    let output = Command::new("cmd")
        .args(&["/C", bat_file])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Hibernate script executed successfully.".into())
    } else {
        Err("Failed to execute hibernate script.".into())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_hibernate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
