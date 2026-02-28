#![cfg(target_arch = "wasm32")]

use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use js_sys::Promise;
use web_sys::console;

// ============ Data Structures ============

#[derive(Deserialize)]
struct WeatherResponse {
    location: Location,
    current: Current,
}

#[derive(Deserialize)]
struct Location {
    name: String,
    country: String,
}

#[derive(Deserialize)]
struct Current {
    temp_c: f64,
    feelslike_c: f64,
    humidity: u64,
    wind_kph: f64,
    condition: Condition,
}

#[derive(Deserialize)]
struct Condition {
    text: String,
}

#[derive(serde::Serialize)]
struct WeatherData {
    city: String,
    country: String,
    temp_c: f64,
    feels_like_c: f64,
    humidity: u64,
    wind_kph: f64,
    condition: String,
}

// ============ WASM Bindings ============

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console::log_1(&"WeathRS-GUI WebAssembly loaded!".into());
}

#[wasm_bindgen]
pub fn fetch_weather(city: &str) -> Promise {
    let city = city.to_string();

    future_to_promise(async move {
        let api_key = "3c732f03fa2d4f9fa49112749250410";
        let api_url = format!(
            "https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
            api_key,
            urlencoding::encode(&city)
        );
        let proxy_url = format!("https://corsproxy.io/?{}", urlencoding::encode(&api_url));

        match reqwest::get(&proxy_url).await {
            Ok(response) if response.status().is_success() => {
                match response.json::<WeatherResponse>().await {
                    Ok(data) => {
                        let weather_data = WeatherData {
                            city: data.location.name,
                            country: data.location.country,
                            temp_c: data.current.temp_c,
                            feels_like_c: data.current.feelslike_c,
                            humidity: data.current.humidity,
                            wind_kph: data.current.wind_kph,
                            condition: data.current.condition.text,
                        };
                        serde_wasm_bindgen::to_value(&weather_data)
                            .map_err(|e| JsValue::from_str(&e.to_string()))
                    }
                    Err(e) => Err(JsValue::from_str(&e.to_string())),
                }
            }
            Ok(response) => Err(JsValue::from_str(&format!("HTTP error: {}", response.status()))),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    })
}

#[wasm_bindgen]
pub fn get_weather_icon(condition: &str) -> String {
    let c = condition.to_lowercase();
    if c.contains("sunny") || c.contains("clear")          { "☀️".into() }
    else if c.contains("partly cloudy")                    { "⛅".into() }
    else if c.contains("cloudy") || c.contains("overcast") { "☁️".into() }
    else if c.contains("rain") || c.contains("drizzle")    { "🌧️".into() }
    else if c.contains("thunder") || c.contains("storm")   { "⛈️".into() }
    else if c.contains("snow") || c.contains("blizzard")   { "❄️".into() }
    else if c.contains("mist") || c.contains("fog")        { "🌫️".into() }
    else                                                    { "☁️".into() }
}

#[wasm_bindgen]
pub fn format_temp(temp_c: f64) -> String {
    format!("{:.0}°C", temp_c)
}

#[wasm_bindgen]
pub fn format_wind(wind_kph: f64) -> String {
    format!("{:.1} kph", wind_kph)
}
