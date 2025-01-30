<script>
	import sensors from '$lib/backend/sensors.svelte';
	import BooleanActuatorCard from '$lib/components/devices/BooleanActuatorCard.svelte';
	import SensorCard from '$lib/components/devices/SensorCard.svelte';
	import Cpu from 'lucide-svelte/icons/cpu';
</script>

<div class="bg-sidebar w-full rounded-lg border shadow">
	<div class="text-muted-foreground border-b p-4 text-sm font-bold uppercase">
		<h2 class="flex items-center gap-2">
			<Cpu class="inline" />
			Sensors & Actuators
		</h2>
	</div>
	<div class="grid grid-cols-3 gap-4 p-4">
		<SensorCard title="Greenhouse Temperature" description="25°C (Normal)" />
		<BooleanActuatorCard
			title="Greenhouse Fan"
			isOn={sensors.actuatorState}
			toggle={() => sensors.toggleActuatorState()}
		/>
		<SensorCard title="Soil Moisture (Bed A)" description="38% (Low)" />
		<SensorCard title="Barn Temperature" description="18°C (Normal)" />
		<BooleanActuatorCard
			title="Irrigation Valve"
			isOn={sensors.actuatorState}
			toggle={() => sensors.toggleActuatorState()}
		/>
		<BooleanActuatorCard
			title="Feed Dispenser (Poultry)"
			isOn={sensors.actuatorState}
			toggle={() => sensors.toggleActuatorState()}
		/>
	</div>
</div>
