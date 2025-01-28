import { BACKEND_URL } from '$lib/utils';
import Fetched from './fetched.svelte';
import axios from 'axios';

type SensorData = { id: number; src_timestamp: number; cpu_temp: number; cpu_volt: number };

class Sensors {
	sensorFetcher = new Fetched<SensorData[]>(BACKEND_URL + '/sensors');
	actuatorState = $state<boolean | undefined>();

	constructor() {
		this.refreshActuatorState();
	}

	async toggleActuatorState() {
		try {
			const config = { headers: { 'Content-Type': 'application/json' } };
			const body = { new_state: !this.actuatorState };
			const response = await axios.post(BACKEND_URL + '/actuator/state', body, config);
			this.actuatorState = response.data;
		} catch (error) {
			console.error('failed to toggleActuatorState' + error);
		}
	}

	async refreshActuatorState() {
		try {
			const response = await axios.get(BACKEND_URL + '/actuator/state');
			this.actuatorState = response.data;
		} catch (error) {
			console.error('failed to toggleActuatorState' + error);
		}
	}
}

const sensors = new Sensors();
export default sensors;
