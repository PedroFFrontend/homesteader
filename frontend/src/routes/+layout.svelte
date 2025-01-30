<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import SidebarProvider from '$lib/components/ui/sidebar/sidebar-provider.svelte';
	import Sidebar from '$lib/views/layout/Sidebar.svelte';
	import Header from '$lib/views/layout/Header.svelte';

	let { children } = $props();
</script>

<ModeWatcher />
<SidebarProvider>
	<Sidebar />
	<div class="flex w-full flex-col">
		<Header />
		<main class="p-4">
			{@render children?.()}
		</main>
	</div>
</SidebarProvider>
