type SensorData = { id: number; src_timestamp: number; cpu_temp: number; cpu_volt: number };

const backendUrl = 'http://localhost:8080';

class Sensors {
	data = $state<SensorData[]>([]);
	loading = $state<boolean>(false);
	loadedOnce = $state<boolean>(false);
	url: string;

	constructor(url: string) {
		this.url = url;
	}

	async load() {
		this.loading = true;
		try {
			const response = await fetch(this.url);
			const result = (await response.json()) as SensorData[];
			console.log(result);
			this.data = result;
		} catch (error) {
			console.error('failed to load sensor data: ' + error);
		}
		this.loading = false;
		this.loadedOnce = true;
	}
}

const sensors = new Sensors(backendUrl);
export default sensors;
