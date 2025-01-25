<script lang="ts">
	import weather from '$lib/backend/weather.svelte';
	import { Button } from '$lib/components/ui/button';
	import { kelvinToCelsius } from '$lib/utils';
	import { onMount } from 'svelte';

	onMount(() => {
		if (!weather.current?.data) weather.current?.load();
	});

	let data = $derived(weather.current?.data);
</script>

<div class="bg-card h-fit rounded-lg border shadow">
	<div class="flex min-w-[250px] flex-col">
		<div class="flex items-center justify-between border-b p-2 pl-3">
			<h2 class="text-muted-foreground text-sm font-bold uppercase">Current Weather</h2>
			<Button
				onclick={() => weather.current?.load()}
				disabled={weather.current?.loading}
				variant="outline"
			>
				{weather.current?.loading ? 'Loading...' : 'Load'}
			</Button>
		</div>
		{#if data}
			<div class="flex flex-col text-sm *:border-b *:p-2">
				<div class="flex justify-between">
					<div>Conditions</div>
					<div>
						{data.weather[0].description}
					</div>
				</div>

				<div class="flex justify-between">
					<div>Temperature</div>
					<div>{kelvinToCelsius(data.main.temp).toFixed(0)} ºC</div>
				</div>

				<div class="flex justify-between">
					<div>Humidity</div>
					<div>{data.main.humidity}%</div>
				</div>

				<div class="flex justify-between">
					<div>Pressure</div>
					<div>{data.main.grnd_level} hPa</div>
				</div>

				<div class="flex justify-between">
					<div>Cloud Coverage</div>
					<div>{data.clouds.all} %</div>
				</div>

				<div class="flex justify-between">
					<div>Wind Speed</div>
					<div>{data.wind.speed} m/s</div>
				</div>
				<div class="flex justify-between">
					<div>Direction</div>
					<div>{data.wind.deg}º</div>
				</div>
				<div class="flex justify-between">
					<div>Gust Speed</div>
					<div>{data.wind.gust} m/s</div>
				</div>

				<div class="flex justify-between">
					<div>Rainfall</div>
					<div>{data.rain?.['1h']} mm/h</div>
				</div>
			</div>
		{/if}
	</div>
</div>
