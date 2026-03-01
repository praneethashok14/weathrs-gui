use tauri::Manager;

const API_KEY: &str = "3c732f03fa2d4f9fa49112749250410";

#[tauri::command]
async fn get_weather(q: String) -> Result<serde_json::Value, String> {
    let url = format!(
        "https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
        API_KEY,
        urlencoding::encode(&q)
    );
    reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn load_cities(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("cities.txt");
    let text = std::fs::read_to_string(path).unwrap_or_default();
    Ok(text
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(String::from)
        .collect())
}

#[tauri::command]
async fn save_cities(app: tauri::AppHandle, cities: Vec<String>) -> Result<(), String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("cities.txt"), cities.join("\n")).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_weather, load_cities, save_cities])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
