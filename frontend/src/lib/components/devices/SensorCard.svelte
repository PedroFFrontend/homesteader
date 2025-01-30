<script lang="ts">
	import { cn } from '$lib/utils';

	interface Props {
		title: string;
		description: string;
	}
	let { title, description }: Props = $props();
</script>

<div class={cn('bg-sidebar flex flex-col gap-4 rounded border p-4 shadow')}>
	<div class="text-muted-foreground text-sm font-medium uppercase">{title}</div>
	<div class="flex flex-col gap-4">
		<div class="flex items-center gap-2">
			<div class="text-sm font-bold uppercase">
				{description}
			</div>
		</div>
	</div>
</div>
