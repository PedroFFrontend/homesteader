<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	let latitude = $state('');
	let longitude = $state('');

	$effect(() => {});
</script>

<div>40.21782213237546, -7.496239025718687</div>
<div>
	<Label for="input-lat">Latitude</Label>
	<Input id="input-lat" bind:value={latitude} />
</div>
<div>
	<Label for="input-lon">Longitude</Label>
	<Input id="input-lon" bind:value={longitude} />
</div>

<div>
	{latitude},{longitude}
</div>
