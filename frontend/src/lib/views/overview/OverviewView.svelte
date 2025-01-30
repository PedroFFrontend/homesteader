<script lang="ts">
	import Alerts from './components/Alerts.svelte';
	import CurrentWeather from './components/CurrentWeather.svelte';
	import ForecastWeather from './components/ForecastWeather.svelte';
	import GeolocationInput from './components/GeolocationInput.svelte';
	import SensorsActuators from './components/SensorsActuators.svelte';
</script>

<div class="flex w-full flex-col gap-4">
	<Alerts />
	<div class="flex flex-col gap-4 lg:flex-row">
		<SensorsActuators />
		<CurrentWeather />
		<GeolocationInput />
	</div>
	<ForecastWeather />
</div>
