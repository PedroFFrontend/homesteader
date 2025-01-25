import type { GeospatialLocation } from '$lib/models/user.svelte';
import user from '$lib/models/user.svelte';
import { BACKEND_URL } from '$lib/utils';
import Fetched from './fetched.svelte';

type CurrentWeatherData = {
	coord: { lat: number; lon: number };
	main: { temp: number; grnd_level: number; humidity: number };
	weather: [{ id: number; description: string }];
	clouds: { all: number };
	wind: { speed: number; deg: number; gust: number };
	rain: { '1h': number };
};

type ForecastWeatherData = {
	cnt: number;
	list: {
		dt: number;
		main: { temp: number; grnd_level: number; humidity: number };
		weather: [{ id: number; description: string }];
		clouds: { all: number };
		wind: { speed: number; deg: number; gust: number };
		pop: number;
		rain: { '3h': number };
	}[];
};
function getCurrentWeatherUrl(location: GeospatialLocation) {
	return BACKEND_URL + `/weather/current?lat=${location.latitude}&lon=${location.longitude}`;
}
function getForecastWeatherUrl(location: GeospatialLocation) {
	return BACKEND_URL + `/weather/forecast?lat=${location.latitude}&lon=${location.longitude}`;
}

class Weather {
	current: Fetched<CurrentWeatherData> | undefined = $state();
	forecast: Fetched<ForecastWeatherData> | undefined = $state();
	location: GeospatialLocation | undefined = $state();
	constructor() {
		if (user.location) {
			this.location = user.location;
			this.current = new Fetched<CurrentWeatherData>(getCurrentWeatherUrl(this.location));
			this.forecast = new Fetched<ForecastWeatherData>(getForecastWeatherUrl(this.location));
		}
	}
	setLocation(location: GeospatialLocation | undefined) {
		if (location) {
			this.location = location;
			this.current = new Fetched<CurrentWeatherData>(getCurrentWeatherUrl(this.location));
			this.forecast = new Fetched<ForecastWeatherData>(getForecastWeatherUrl(this.location));
			this.current.load();
			this.forecast.load();
		}
	}
}

const weather = new Weather();

export default weather;
