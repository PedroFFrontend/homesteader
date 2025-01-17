<script lang="ts">
	import sensors from '$lib/backend/backend.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Table from '$lib/components/ui/table/index.js';
	import { onMount } from 'svelte';

	onMount(() => {
		sensors.load();
	});
</script>

<div class="flex w-full flex-col items-center">
	<div class="bg-card max-w-[700px] rounded-lg border shadow">
		<div class="flex items-center justify-between border-b p-2 pl-3">
			<h2 class="text-muted-foreground text-sm font-bold uppercase">Sensor Data</h2>
			<Button onclick={() => sensors.load()} disabled={sensors.loading}>
				{sensors.loading ? 'Loading...' : 'Load'}
			</Button>
		</div>
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head class="w-[100px]">ID</Table.Head>
					<Table.Head>Sensor Timestamp</Table.Head>
					<Table.Head>CPU Temperature</Table.Head>
					<Table.Head class="text-right">CPU Voltage</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each sensors.data as row}
					<Table.Row>
						<Table.Cell class="font-medium">
							{row.id}
						</Table.Cell>
						<Table.Cell>
							{new Date(row.src_timestamp).toLocaleString()}
						</Table.Cell>
						<Table.Cell>
							{row.cpu_temp}
						</Table.Cell>
						<Table.Cell class="text-right">
							{row.cpu_volt}
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
	<div>
		{#if !sensors.loadedOnce}
			Loading...
		{/if}
	</div>
</div>
