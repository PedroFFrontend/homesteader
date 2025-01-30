<script lang="ts">
	import Check from 'lucide-svelte/icons/check';
	import Bell from 'lucide-svelte/icons/bell';
	import Alert from './Alert.svelte';
	import { alertManager } from '$lib/models/alerts.svelte';
</script>

<div class="bg-sidebar w-full rounded-lg border shadow">
	<div class="text-muted-foreground border-b p-4 text-sm font-bold uppercase">
		<h2 class="flex items-center gap-2">
			<Bell class="inline" />
			Alerts & Notifications
		</h2>
	</div>
	<div class="p- flex flex-col gap-4">
		{#if alertManager.alerts.length > 0}
			<div class="[*:not(:last-child)]:border-b">
				{#each alertManager.alerts as alert}
					<Alert {alert} />
				{/each}
			</div>
		{:else}
			<div class="text-muted-foreground grid h-full w-full place-items-center py-8">
				Nothing requires your attention.
			</div>
		{/if}
	</div>
</div>
