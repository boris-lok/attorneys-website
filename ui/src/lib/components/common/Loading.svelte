<script lang="ts">
	import IconifyIcon from '@iconify/svelte';
</script>

<svelte:window on:wheel|nonpassive|preventDefault />
<div class="w-full h-screen top-0 left-0 bg-gray-100/50 flex fixed items-center justify-center z-50">
	<IconifyIcon class="w-12 h-12" icon="eos-icons:bubble-loading" />
</div>