<script lang="ts">

	import { Fa } from 'svelte-fa';
	import type { IconDefinition } from '@fortawesome/free-solid-svg-icons';

	let { icon, label, topBar }: {
		icon: IconDefinition,
		label: string,
		topBar: boolean,
	} = $props();

</script>


{#if topBar}
	<div
		class="relative flex flex-row gap-2 group cursor-pointer items-center">
		<Fa icon={icon} class="text-xl" />
		<p>{label}</p>
	</div>
{:else}
	<div
		class="relative flex flex-col gap-2 w-32 h-32 items-center justify-around p-4 rounded border-black group border-2 hover:border-4 cursor-pointer">
		<Fa icon={icon} class="text-4xl" />
		<p class=" group-hover:font-bold">{label}</p>
	</div>
{/if}

