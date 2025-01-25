import { BACKEND_URL } from '$lib/utils';
import Fetched from './fetched.svelte';

type SensorData = { id: number; src_timestamp: number; cpu_temp: number; cpu_volt: number };

class Sensors extends Fetched<SensorData[]> {
	constructor() {
		super(BACKEND_URL + '/sensors');
	}
}

const sensors = new Sensors();
export default sensors;
