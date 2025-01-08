<script lang="ts">
	import IconifyIcon from '@iconify/svelte';

	let { icon, label, topBar }: {
		icon: string,
		label: string,
		topBar: boolean,
	} = $props();

</script>


{#if topBar}
	<div
		class="relative flex flex-row gap-2 group cursor-pointer items-center">
		<IconifyIcon icon={icon} class="w-6 h-6" />
		<p>{label}</p>
	</div>
{:else}
	<div
		class="relative flex flex-col gap-2 w-32 h-32 items-center justify-around p-4 rounded border-black group border-2 hover:border-4 cursor-pointer">
		<IconifyIcon icon={icon} class="w-12 h-12" />
		<p class=" group-hover:font-bold">{label}</p>
	</div>
{/if}

