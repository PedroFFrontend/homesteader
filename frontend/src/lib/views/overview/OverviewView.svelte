<script lang="ts">
	import sensors from '$lib/backend/backend.svelte';
	import { onMount } from 'svelte';

	onMount(() => {
		sensors.load();
	});
</script>

<div>
	{#if sensors.loadedOnce}
		{#each sensors.data as row}
			<div class="flex gap-4">
				<div>
					{row.id}
				</div>
				<div>
					{row.src_timestamp}
				</div>
				<div>
					{row.cpu_temp}
				</div>
				<div>
					{row.cpu_volt}
				</div>
			</div>
		{/each}
	{:else if sensors.loading}
		Loading...
	{/if}
</div>
<div>
	<button onclick={() => sensors.load()} disabled={sensors.loading}> Load </button>
</div>
