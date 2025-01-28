<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import SidebarProvider from '$lib/components/ui/sidebar/sidebar-provider.svelte';
	import { SidebarTrigger } from '$lib/components/ui/sidebar';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { page } from '$app/state';

	let { children } = $props();

	let title = 'Home';
	let description = 'Description of your website.';
	let image = 'https://example.com/your-logo.png';
</script>

<svelte:head>
	<title>{title} | Homesteader</title>
	<meta name="description" content={description} />
	<meta property="og_site_name" content="Homesteader" />
	<meta property="og:url" content="https://www.example.com{page.url.pathname.toString()}" />
	<meta property="og:type" content="website" />
	<meta property="og:title" content={title} />
	<meta property="og:description" content={description} />
	<meta property="og:image" content={image} />

	<meta name="twitter:card" content="summary_large_image" />
	<meta property="twitter:domain" content="example.com" />
	<meta property="twitter:url" content="https://www.example.com{page.url.pathname.toString()}" />
	<meta name="twitter:title" content={title} />
	<meta name="twitter:description" content={description} />
	<meta name="twitter:image" content={image} />
	{@html `  <script type="application/ld+json">{
   "@context": "https://schema.org",
   "@type": "Website",
   "name": "${title} | example.com",
   "url": "https//www.example.com${page.url.pathname}",
   "logo": ${image}  }</script>`}
</svelte:head>

<ModeWatcher />
<SidebarProvider>
	<Sidebar />
	<div class="flex w-full flex-col">
		<header class="bg-sidebar flex h-[41px] items-center border-b pr-2">
			<div class="flex h-full w-[40px] items-center justify-center border-r">
				<SidebarTrigger />
			</div>
		</header>
		<main class="p-4">
			{@render children?.()}
		</main>
	</div>
</SidebarProvider>
