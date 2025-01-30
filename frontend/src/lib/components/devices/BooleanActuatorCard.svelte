<script lang="ts">
	import { cn } from '$lib/utils';
	import Button from '../ui/button/button.svelte';

	interface Props {
		title: string;
		isOn: boolean | undefined;
		toggle: () => Promise<void>;
	}
	let { title, isOn, toggle }: Props = $props();

	let loading = $state(false);

	async function onToggle() {
		loading = true;
		await toggle();
		loading = false;
	}
</script>

<div
	class={cn(
		'bg-sidebar flex flex-col gap-2 rounded border p-4 shadow',
		loading ? 'animate-pulse' : ''
	)}
>
	<div class="text-muted-foreground text-sm font-medium uppercase">{title}</div>
	<div class="flex flex-col gap-2">
		<div class="flex items-center gap-2">
			<div
				class={cn('size-4 rounded-full ', isOn ? 'animate-pulse bg-green-500' : 'bg-gray-500')}
			></div>
			<div class="text-sm font-bold uppercase">
				STATUS: {isOn ? 'On' : 'Off'}
			</div>
		</div>
		<Button
			onclick={onToggle}
			variant={isOn ? 'destructive' : 'success'}
			disabled={loading}
			class="w-full"
		>
			{loading ? 'Waiting...' : isOn ? 'Turn OFF' : 'Turn ON'}
		</Button>
	</div>
</div>
