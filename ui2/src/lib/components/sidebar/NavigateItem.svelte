<script lang="ts">
	import Icon from '@iconify/svelte';

	let { icon, label } = $props();
</script>

<div class="relative flex flex-row gap-1 items-center text-lg">
	{#if typeof icon === 'string' && icon !== ''}
		<Icon icon={icon} />
	{/if}
	<p>{ label }</p>
</div>
