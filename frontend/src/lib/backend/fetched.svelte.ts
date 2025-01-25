export default class Fetched<T> {
	loading = $state<boolean>(false);
	loadedOnce = $state<boolean>(false);
	url: string;
	data: T | undefined = $state(undefined);
	error = $state('');

	constructor(url: string) {
		this.url = url;
	}

	async load() {
		this.loading = true;
		try {
			const response = await fetch(this.url);
			const result = (await response.json()) as T;
			console.log(result);
			this.data = result;
		} catch (error) {
			console.error('failed to load data: ' + error);
			this.error = String(error);
		}
		this.loading = false;
		this.loadedOnce = true;
	}
}
