<script lang="ts">
	import Radio from 'lucide-svelte/icons/radio';
	import Dashboard from 'lucide-svelte/icons/layout-dashboard';
	import RadioTower from 'lucide-svelte/icons/radio-tower';
	import Database from 'lucide-svelte/icons/database';
	import Settings from 'lucide-svelte/icons/settings';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { page } from '$app/stores';
	import ThemeButton from './ThemeButton.svelte';

	// Menu items.
	const items = [
		{
			title: 'Overview',
			url: '/',
			icon: Dashboard
		},
		{
			title: 'Sensors',
			url: '/sensors',
			icon: Radio
		}
		// {
		// 	title: 'LoRaWAN Gateway',
		// 	url: '/gateway',
		// 	icon: RadioTower
		// },
		// {
		// 	title: 'Database',
		// 	url: '/database',
		// 	icon: Database
		// },
		// {
		// 	title: 'Settings',
		// 	url: '/settings',
		// 	icon: Settings
		// }
	];
</script>

<Sidebar.Root>
	<Sidebar.Content>
		<Sidebar.Header class="flex items-center justify-center border-b font-bold">
			Homesteader
		</Sidebar.Header>
		<Sidebar.Group>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					{#each items as item (item.title)}
						{@const active = $page.route.id === item.url}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton isActive={active}>
								{#snippet child({ props }: any)}
									<a href={item.url} {...props}>
										<item.icon />
										<span>{item.title}</span>
									</a>
								{/snippet}
							</Sidebar.MenuButton>
						</Sidebar.MenuItem>
					{/each}
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
	<Sidebar.Footer class="flex items-end">
		<ThemeButton />
	</Sidebar.Footer>
</Sidebar.Root>
