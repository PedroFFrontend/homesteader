<script lang="ts">
	import { cn } from '$lib/utils';
	import { Button } from '../ui/button';

	interface Props {
		isOn: boolean | undefined;
		toggle: () => Promise<void>;
	}
	let { isOn, toggle }: Props = $props();

	let loading = $state(false);
</script>

<Button
	class={cn(
		'bg-sidebar h-[100px] w-[100px] rounded-lg border shadow',
		isOn ? 'bg-green-500 hover:bg-green-600' : 'bg-gray-500 hover:bg-gray-600'
	)}
	onclick={() => {
		loading = true;
		toggle().then(() => (loading = false));
	}}
>
	{#if loading}
		...
	{:else if isOn}
		ON
	{:else}
		OFF
	{/if}
</Button>
