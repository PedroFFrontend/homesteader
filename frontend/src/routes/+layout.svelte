<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import SidebarProvider from '$lib/components/ui/sidebar/sidebar-provider.svelte';
	import { SidebarTrigger } from '$lib/components/ui/sidebar';
	import Sidebar from '$lib/components/Sidebar.svelte';

	let { children } = $props();
</script>

<ModeWatcher />
<SidebarProvider>
	<Sidebar />
	<div class="flex w-full flex-col">
		<header class="bg-sidebar flex h-[41px] items-center border-b px-2">
			<div class="flex h-full items-center border-r pr-2">
				<SidebarTrigger />
			</div>
		</header>
		<main class="p-4">
			{@render children?.()}
		</main>
	</div>
</SidebarProvider>
