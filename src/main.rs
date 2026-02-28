use axum::{
    extract::Query,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::{fs, net::SocketAddr, path::PathBuf};
use tokio::net::TcpListener;

const API_KEY: &str = "3c732f03fa2d4f9fa49112749250410";
const STATIC_DIR: &str = "static";
const CITIES_DIR: &str = "cities";
const PORT: u16 = 8000;

fn valid_key(key: &str) -> bool {
    key.len() == 6 && key.chars().all(|c| c.is_ascii_digit())
}

fn cities_path(key: &str) -> PathBuf {
    PathBuf::from(CITIES_DIR).join(format!("{}.txt", key))
}

#[tokio::main]
async fn main() {
    fs::create_dir_all(CITIES_DIR).expect("Failed to create cities directory");

    let app = Router::new()
        .route("/api/weather", get(weather_handler))
        .route("/api/cities", get(get_cities_handler))
        .route("/api/save-cities", post(save_cities_handler))
        .route("/api/new-key", get(new_key_handler))
        .fallback(static_handler);

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("WeathRS running at http://localhost:{PORT}");
    axum::serve(listener, app).await.unwrap();
}

// ============ Weather API proxy ============

#[derive(Deserialize)]
struct WeatherQuery {
    q: String,
}

async fn weather_handler(Query(params): Query<WeatherQuery>) -> Response {
    let url = format!(
        "https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
        API_KEY,
        urlencoding::encode(&params.q)
    );

    match reqwest::get(&url).await {
        Ok(resp) => {
            let status = resp.status();
            match resp.json::<serde_json::Value>().await {
                Ok(data) => {
                    let code = if status.is_success() { StatusCode::OK } else { StatusCode::BAD_GATEWAY };
                    (code, Json(data)).into_response()
                }
                Err(e) => (StatusCode::BAD_GATEWAY, e.to_string()).into_response(),
            }
        }
        Err(e) => (StatusCode::BAD_GATEWAY, e.to_string()).into_response(),
    }
}

// ============ Cities (per save key) ============

#[derive(Deserialize)]
struct KeyQuery {
    key: String,
}

async fn get_cities_handler(Query(params): Query<KeyQuery>) -> Response {
    if !valid_key(&params.key) {
        return (StatusCode::BAD_REQUEST, "Invalid key").into_response();
    }
    let data = fs::read_to_string(cities_path(&params.key)).unwrap_or_default();
    ([(header::CONTENT_TYPE, "text/plain")], data).into_response()
}

#[derive(Deserialize)]
struct SaveCitiesBody {
    key: String,
    cities: String,
}

async fn save_cities_handler(Json(body): Json<SaveCitiesBody>) -> Response {
    if !valid_key(&body.key) {
        return (StatusCode::BAD_REQUEST, "Invalid key").into_response();
    }
    match fs::write(cities_path(&body.key), &body.cities) {
        Ok(_) => Json(serde_json::json!({ "success": true })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn new_key_handler() -> Response {
    use std::time::{SystemTime, UNIX_EPOCH};

    for attempt in 0u32..100 {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        let key = format!("{:06}", (nanos.wrapping_add(attempt.wrapping_mul(999983))) % 1_000_000);
        if !cities_path(&key).exists() {
            let _ = fs::write(cities_path(&key), "");
            return Json(serde_json::json!({ "key": key })).into_response();
        }
    }
    (StatusCode::INTERNAL_SERVER_ERROR, "Could not generate unique key").into_response()
}

// ============ Static file server ============

async fn static_handler(uri: axum::http::Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };
    let file_path = PathBuf::from(STATIC_DIR).join(path);

    match fs::read(&file_path) {
        Ok(data) => {
            let mime = match file_path.extension().and_then(|e| e.to_str()) {
                Some("html") => "text/html; charset=utf-8",
                Some("css")  => "text/css; charset=utf-8",
                Some("js")   => "application/javascript; charset=utf-8",
                _            => "application/octet-stream",
            };
            ([(header::CONTENT_TYPE, mime)], data).into_response()
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
