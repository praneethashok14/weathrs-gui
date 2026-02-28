# WeathRS-GUI

Weather app built with Rust + a vanilla JS/HTML/CSS frontend.

## Project Structure

```
weathrs-gui/
├── Cargo.toml          # Rust dependencies
├── src/
│   ├── main.rs         # HTTP server (axum) — serves frontend + proxies WeatherAPI
│   └── lib.rs          # WebAssembly bindings (wasm32 target only, optional)
├── static/
│   ├── index.html      # Frontend HTML
│   ├── style.css       # Styles
│   └── app.js          # App logic
└── cities/             # Created at runtime — one .txt file per save key
```

## Prerequisites

- **Rust** — install from https://rustup.rs

## Run

```bash
cargo run
```

Opens at `http://localhost:8000`.

For a production build:

```bash
cargo build --release
./target/release/weathrs-gui
```

Copy `target/release/weathrs-gui` and the `static/` folder to your server.

## Features

- Search for any city worldwide
- Save favourite cities with a 6-digit save key
- Time-of-day background that reflects local time at the searched city
- Real-time weather from WeatherAPI (proxied server-side — no CORS issues)
- Dynamic weather icons based on conditions

## Save Keys

On first visit a modal prompts you to enter an existing 6-digit key or generate a new one. Cities are stored server-side in `cities/<key>.txt`. The key is kept in `localStorage` so returning visitors load automatically.

## API

Uses [WeatherAPI.com](https://www.weatherapi.com/) for weather data.
API key is configured in `src/main.rs`.

## WASM (optional)

`src/lib.rs` exports WASM bindings (`fetch_weather`, `get_weather_icon`, etc.) for the `wasm32` target. These are not used by the server-based frontend but can be built separately:

```bash
wasm-pack build --target web
```

## License

MIT
