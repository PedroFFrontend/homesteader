<script lang="ts">
	import CurrentWeather from './components/CurrentWeather.svelte';
	import ForecastWeather from './components/ForecastWeather.svelte';
	import GeolocationInput from './components/GeolocationInput.svelte';
</script>

<div class="flex w-full flex-row gap-4">
	<GeolocationInput />
	<CurrentWeather />
	<ForecastWeather />
</div>
