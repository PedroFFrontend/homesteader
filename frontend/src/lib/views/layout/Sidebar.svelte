<script lang="ts">
	import Dashboard from 'lucide-svelte/icons/layout-dashboard';
	import Cpu from 'lucide-svelte/icons/cpu';
	import RadioTower from 'lucide-svelte/icons/radio-tower';
	import Database from 'lucide-svelte/icons/database';
	import Settings from 'lucide-svelte/icons/settings';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { page } from '$app/stores';
	import ThemeButton from '../../components/ThemeButton.svelte';
	import Origami from 'lucide-svelte/icons/origami';
	import { Button } from '$lib/components/ui/button';
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
			icon: Cpu
		}
		// {
		// 	title: 'LoRaWAN Gateway',
		// 	url: '/gateway',
		// 	icon: RadioTower
		// }
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
		<Sidebar.Header class="flex items-center justify-center font-bold">
			<Sidebar.Menu>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton class="h-12">
						{#snippet child({ props }: any)}
							<a href="/" {...props}>
								<div class="rounded-lg bg-sky-500 p-1">
									<Origami class="!h-6 !w-6" />
								</div>
								Homesteader
							</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
			</Sidebar.Menu>

			<!-- <Button href="/" variant="ghost" class="h-12 max-h-none w-full justify-start p-2 text-left">
				
			</Button> -->
		</Sidebar.Header>
		<Sidebar.Group>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					{#each items as item (item.title)}
						{@const active = $page.route.id === item.url}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton isActive={active}>
								{#snippet child({ props }: any)}
									{#if item.url}
										<a href={item.url} {...props}>
											<item.icon />
											<span>{item.title}</span>
										</a>
									{:else}
										<span {...props}>
											<item.icon />
											<span>{item.title}</span>
										</span>
									{/if}
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
