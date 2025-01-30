<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { Alert } from '$lib/models/alerts.svelte';
	import { cn } from '$lib/utils';
	import CircleAlert from 'lucide-svelte/icons/circle-alert';
	import Info from 'lucide-svelte/icons/info';

	interface Props {
		alert: Alert;
	}
	let { alert }: Props = $props();

	function getAlertColor(severity: Alert['severity']) {
		if (severity === 'critical') return 'bg-red-500 shadow-red-500 shadow-lg';
		else if (severity === 'severe') return 'bg-orange-500';
		else return 'bg-muted-foreground';
	}

	function getAlertTextColor(severity: Alert['severity']) {
		if (severity === 'critical') return 'text-red-500';
		else if (severity === 'severe') return 'text-orange-500';
		else return 'text-muted-foreground';
	}

	let alertColor = getAlertColor(alert.severity);
	let alertTextColor = getAlertTextColor(alert.severity);
</script>

<div class="relative flex items-center justify-between border-b py-1 pl-4 pr-1">
	<div class={cn('absolute left-0 h-full w-2', alertColor)}></div>
	<div>
		<div>
			<span class={cn('font-medium', alertTextColor)}>
				{alert.description}
			</span>
			<span class="text-muted-foreground"> • </span>
			<span>
				{alert.suggestedAction}
			</span>
		</div>
		<div class="text-muted-foreground text-xs">
			{new Date().toLocaleString()}
		</div>
	</div>
	<Button variant="outline" onclick={() => alert.dismiss()}>Dismiss</Button>
</div>
