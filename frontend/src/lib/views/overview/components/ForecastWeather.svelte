<script lang="ts">
	import weather from '$lib/backend/weather.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Table from '$lib/components/ui/table/index';
	import { kelvinToCelsius } from '$lib/utils';
	import { onMount } from 'svelte';
	import ThermometerSnowflake from 'lucide-svelte/icons/thermometer-snowflake';
	import ThermometerSun from 'lucide-svelte/icons/thermometer-sun';

	onMount(() => {
		if (!weather.forecast) return;
		if (!weather.forecast.data) weather.forecast.load();
	});

	let data = $derived(weather.forecast?.data);
</script>

<div class="bg-card rounded-lg border shadow">
	<div class="flex min-w-[250px] flex-col">
		<div class="flex items-center justify-between border-b p-2 pl-3">
			<h2 class="text-muted-foreground text-sm font-bold uppercase">Forecast Weather</h2>
			<Button
				onclick={() => weather.forecast?.load()}
				disabled={weather.forecast?.loading}
				variant="outline"
			>
				{weather.forecast?.loading ? 'Loading...' : 'Load'}
			</Button>
		</div>
		{#if data}
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Time</Table.Head>
						<Table.Head>Conditions</Table.Head>
						<Table.Head>Temperature</Table.Head>
						<Table.Head>Humidity</Table.Head>
						<Table.Head>Pressure</Table.Head>
						<Table.Head>Clouds</Table.Head>
						<Table.Head class="border-l border-l-blue-700/20 bg-blue-700/10">Wind Speed</Table.Head>
						<Table.Head class="bg-blue-700/10">Direction</Table.Head>
						<Table.Head class="border-r border-r-blue-700/20 bg-blue-700/10">Gust Speed</Table.Head>
						<Table.Head>Rain Chance</Table.Head>
						<Table.Head>Predicted Rainfall</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if data}
						{#each data.list as row}
							<Table.Row>
								<Table.Cell class="font-medium">
									{new Date(row.dt * 1000).toLocaleString()}
								</Table.Cell>
								<Table.Cell>
									{row.weather[0].description}
									<!-- ({row.weather[0].id}) -->
								</Table.Cell>
								<Table.Cell
									class={kelvinToCelsius(row.main.temp) < 0
										? 'text-sky-500'
										: kelvinToCelsius(row.main.temp) > 30
											? 'text-orange-500'
											: ''}
								>
									<div class="flex items-center gap-2">
										{#if kelvinToCelsius(row.main.temp) < 0}
											<ThermometerSnowflake size={20} />
										{:else if kelvinToCelsius(row.main.temp) > 30}
											<ThermometerSun size={20} />
										{/if}
										{kelvinToCelsius(row.main.temp).toFixed(0)} ºC
									</div>
								</Table.Cell>
								<Table.Cell>
									{row.main.humidity}%
								</Table.Cell>
								<Table.Cell>
									{row.main.grnd_level} hPa
								</Table.Cell>
								<Table.Cell>
									{row.clouds.all} %
								</Table.Cell>
								<Table.Cell class="border-l border-l-blue-700/20">
									{row.wind.speed} m/s
								</Table.Cell>
								<Table.Cell>
									{row.wind.deg}º
								</Table.Cell>
								<Table.Cell class="border-r border-r-blue-700/20">
									{row.wind.gust} m/s
								</Table.Cell>
								<Table.Cell class={!row.pop ? 'text-muted-foreground' : ''}>
									{(row.pop * 100).toFixed(0)}%
								</Table.Cell>
								<Table.Cell class={!row.rain['3h'] ? 'text-muted-foreground' : ''}>
									{#if row.rain}
										{(row.rain['3h'] / 3).toFixed(2)} mm/h
									{/if}
								</Table.Cell>
							</Table.Row>
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		{/if}
	</div>
</div>
