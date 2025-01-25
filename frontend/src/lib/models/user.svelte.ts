import { browser } from '$app/environment';
import weather from '$lib/backend/weather.svelte';

export type GeospatialLocation = {
	latitude: number;
	longitude: number;
};

export class User {
	location = $state<GeospatialLocation | undefined>(undefined);
	usdaZone: number | undefined;

	constructor() {
		if (!browser) return;
		const stored = localStorage.getItem('user-location');
		const location = stored ? (JSON.parse(stored) as GeospatialLocation) : null;
		if (location) this.location = location;
	}

	setLocation(location: GeospatialLocation) {
		localStorage.setItem('user-location', JSON.stringify(location));
		this.location = { latitude: location.latitude, longitude: location.longitude };
		weather.setLocation(location);
	}
}

const user = new User();
export default user;
