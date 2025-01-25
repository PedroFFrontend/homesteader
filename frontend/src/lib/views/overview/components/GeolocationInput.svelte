<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import user from '$lib/models/user.svelte';

	let latitude = $state(user.location?.latitude.toString() ?? '');
	let longitude = $state(user.location?.longitude.toString() ?? '');
</script>

<div class="bg-card flex h-fit flex-col gap-2 rounded-lg border p-2 shadow">
	<div>
		<Label for="input-lat">Latitude</Label>
		<Input id="input-lat" bind:value={latitude} />
	</div>
	<div>
		<Label for="input-lon">Longitude</Label>
		<Input id="input-lon" bind:value={longitude} />
	</div>
	<Button
		onclick={() => user.setLocation({ latitude: Number(latitude), longitude: Number(longitude) })}
	>
		Set
	</Button>
</div>
