const { invoke } = window.__TAURI__.core;

let cities = [];
let currentCity = null;
let currentSearchedLocation = null;
let citiesWeatherData = {};

const weatherIcons = {
    'sunny':         '☀️',
    'clear':         '🌙',
    'partly cloudy': '⛅',
    'cloudy':        '☁️',
    'overcast':      '☁️',
    'mist':          '🌫️',
    'fog':           '🌫️',
    'rain':          '🌧️',
    'drizzle':       '🌧️',
    'thunder':       '⛈️',
    'storm':         '⛈️',
    'snow':          '❄️',
    'blizzard':      '❄️',
};

function getWeatherIcon(condition) {
    const lower = condition.toLowerCase();
    for (const [key, icon] of Object.entries(weatherIcons)) {
        if (lower.includes(key)) return icon;
    }
    return '☁️';
}

// ============ Cities (Tauri-backed) ============

async function loadCities() {
    try {
        cities = await invoke('load_cities');
        renderCitiesList();
        cities.forEach(city => fetchWeatherForCity(city));
    } catch (e) {
        console.error('Failed to load cities:', e);
    }
}

async function saveCities() {
    try {
        await invoke('save_cities', { cities });
    } catch (e) {
        console.error('Failed to save cities:', e);
    }
}

// ============ Weather API ============

async function apiCall(location) {
    const data = await invoke('get_weather', { q: location });
    if (data.error) throw new Error(data.error.message || 'Location not found');
    return data;
}

async function fetchWeatherForCity(location) {
    try {
        const data = await apiCall(location);
        citiesWeatherData[location] = data;
        renderCitiesList();
    } catch (e) {
        console.error('Failed to fetch weather for', location, e);
    }
}

async function fetchWeather(location) {
    try {
        const data = await apiCall(location);
        currentSearchedLocation = data.location.name;
        citiesWeatherData[location] = data;
        updateWeatherDisplay(data);
        renderCitiesList();

        const addBtn = document.getElementById('addCurrentCityBtn');
        addBtn.style.display = cities.includes(currentSearchedLocation) ? 'none' : 'block';
    } catch (e) {
        console.error('Search failed:', e);
        document.getElementById('cityName').textContent = 'City not found';
        document.getElementById('currentTemp').textContent = '-°C';
        document.getElementById('feelsLike').textContent = 'Try: London, Paris, Tokyo';
    }
}

// ============ Display ============

function applyTimeOfDay(localtime) {
    const hour = parseInt(localtime.split(' ')[1].split(':')[0]);
    let cls, dark;
    if      (hour < 6)  { cls = 'time-night';   dark = true;  }
    else if (hour < 8)  { cls = 'time-dawn';    dark = false; }
    else if (hour < 12) { cls = 'time-morning'; dark = false; }
    else if (hour < 17) { cls = 'time-day';     dark = false; }
    else if (hour < 20) { cls = 'time-evening'; dark = false; }
    else                { cls = 'time-dusk';    dark = true;  }

    const main = document.querySelector('.main-content');
    main.className = 'main-content ' + cls + (dark ? ' dark-bg' : '');
}

function updateClouds(condition, temp) {
    const container = document.getElementById('cloudsContainer');
    container.innerHTML = '';

    const lower = condition.toLowerCase();
    let emoji = '☁️', count = 5;

    if (lower.includes('sunny') || lower.includes('clear')) {
        emoji = '☀️'; count = temp > 20 ? 1 : 2;
    } else if (lower.includes('thunder') || lower.includes('storm')) {
        emoji = '⛈️'; count = 4;
    } else if (lower.includes('rain') || lower.includes('drizzle')) {
        emoji = '🌧️'; count = 4;
    } else if (lower.includes('snow') || lower.includes('blizzard')) {
        emoji = '🌨️'; count = 4;
    } else if (lower.includes('partly cloudy')) {
        emoji = '⛅'; count = 3;
    } else if (lower.includes('mist') || lower.includes('fog')) {
        emoji = '🌫️'; count = 4;
    }

    const positions = [
        { left: '10%', top: '15%' },
        { left: '30%', top: '5%' },
        { left: '50%', top: '20%' },
        { left: '70%', top: '10%' },
        { left: '85%', top: '25%' },
    ];

    for (let i = 0; i < count; i++) {
        const el = document.createElement('div');
        el.className = 'cloud';
        el.textContent = emoji;
        el.style.left = positions[i].left;
        el.style.top = positions[i].top;
        container.appendChild(el);
    }
}

function updateWeatherDisplay(data) {
    const { location, current } = data;
    const temp = Math.round(current.temp_c);
    applyTimeOfDay(location.localtime);

    document.getElementById('cityName').textContent = location.name;
    document.getElementById('currentTemp').textContent = `${temp}°C`;
    document.getElementById('conditionName').textContent = current.condition.text;
    document.getElementById('feelsLike').textContent = `Feels like ${current.feelslike_c.toFixed(1)}°C`;
    document.getElementById('humidity').textContent = `Humidity: ${current.humidity}%`;
    document.getElementById('wind').textContent = `Wind: ${current.wind_kph.toFixed(1)} kph`;

    updateClouds(current.condition.text, temp);
}

function renderCitiesList() {
    const list = document.getElementById('citiesList');

    if (cities.length === 0) {
        list.innerHTML = '<div class="empty-state">No cities added yet.<br>Search and add your first city!</div>';
        return;
    }

    list.innerHTML = '';
    cities.forEach(city => {
        const item = document.createElement('div');
        item.className = 'city-item' + (city === currentCity ? ' active' : '');

        const data = citiesWeatherData[city];
        let tempText = 'Loading...';
        if (data) {
            const temp = Math.round(data.current.temp_c);
            const icon = getWeatherIcon(data.current.condition.text);
            tempText = `${icon} ${temp}°C`;
        }

        item.innerHTML = `
            <div class="city-item-content">
                <div class="city-item-name">${city}</div>
                <div class="city-item-temp">${tempText}</div>
            </div>
            <button class="delete-btn" data-city="${city}">×</button>
        `;

        item.querySelector('.delete-btn').addEventListener('click', e => {
            e.stopPropagation();
            deleteCity(city);
        });

        item.addEventListener('click', () => {
            currentCity = city;
            fetchWeather(city);
            renderCitiesList();
            document.getElementById('addCurrentCityBtn').style.display = 'none';
        });

        list.appendChild(item);
    });
}

function addCurrentCity() {
    if (!currentSearchedLocation) return;
    if (cities.includes(currentSearchedLocation)) return;
    cities.push(currentSearchedLocation);
    saveCities();
    fetchWeatherForCity(currentSearchedLocation);
    document.getElementById('addCurrentCityBtn').style.display = 'none';
    renderCitiesList();
}

function deleteCity(cityName) {
    cities = cities.filter(c => c !== cityName);
    delete citiesWeatherData[cityName];
    saveCities();

    if (currentCity === cityName) {
        currentCity = null;
        document.getElementById('cityName').textContent = 'Search for a city';
        document.getElementById('currentTemp').textContent = '-°C';
        document.getElementById('conditionName').textContent = '-';
        document.getElementById('feelsLike').textContent = 'to see weather';
    }

    renderCitiesList();
}

// ============ Event listeners ============

document.getElementById('addCurrentCityBtn').addEventListener('click', addCurrentCity);

const searchInput = document.getElementById('searchInput');
let searchTimeout;

searchInput.addEventListener('input', e => {
    clearTimeout(searchTimeout);
    const value = e.target.value.trim();
    if (value.length > 2) {
        searchTimeout = setTimeout(() => fetchWeather(value), 500);
    }
});

searchInput.addEventListener('keypress', e => {
    if (e.key === 'Enter') {
        clearTimeout(searchTimeout);
        const value = e.target.value.trim();
        if (value) fetchWeather(value);
    }
});

// ============ Init ============
loadCities();
