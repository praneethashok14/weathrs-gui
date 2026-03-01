# WeathRS

Weather desktop app built with Rust + Tauri and a vanilla JS/HTML/CSS frontend.

## Download

A macOS `.dmg` installer is available in [Releases](../../releases).

## Project Structure

```
weathrs-gui/
├── Cargo.toml              # Workspace root
├── src-tauri/
│   ├── Cargo.toml          # Rust dependencies (tauri, reqwest)
│   ├── build.rs
│   ├── tauri.conf.json     # App config (window size, icons, identifier)
│   ├── capabilities/
│   │   └── default.json    # IPC permissions
│   ├── icons/
│   │   ├── icon.png
│   │   └── icon.icns
│   └── src/
│       └── main.rs         # Tauri commands: get_weather, load_cities, save_cities
└── static/
    ├── index.html          # Frontend HTML
    ├── style.css           # Styles (responsive)
    └── app.js              # App logic (calls Rust via invoke())
```

## Prerequisites

- [Rust](https://rustup.rs)
- [Tauri CLI](https://tauri.app/start/prerequisites/) — `cargo install tauri-cli --version "^2"`

## Dev

```bash
cargo tauri dev
```

## Build

```bash
cargo tauri build
```

Produces a native app + installer in `src-tauri/target/release/bundle/`.

## Features

- Search any city worldwide
- Save favourite cities — stored automatically in the OS app data folder
- Time-of-day background reflecting local time at the searched city
- Weather data fetched server-side via Rust (API key never exposed to the frontend)
- Dynamic weather icons and animated clouds based on conditions
- Responsive layout — works on any screen size

## API

Uses [WeatherAPI.com](https://www.weatherapi.com/) for weather data.
API key is set in `src-tauri/src/main.rs`.

## License

MIT
