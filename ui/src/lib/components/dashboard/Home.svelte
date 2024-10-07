<script lang="ts">
	import { onMount } from 'svelte';
	import { Home } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import type { HomeData } from '$lib/models/Home';
	import SvelteMarkdown from 'svelte-markdown';

	let isLoading = false;
	let data: HomeData | null = null;

	onMount(() => {
		Home.list('zh')
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => {
					data = e;
				})
			)
			.subscribe();
	});
</script>

<div class="home-wrapper">
	{#if isLoading}
		<p>Loading...</p>
	{:else if data}
		<SvelteMarkdown source={data.data.data} />
	{:else}
		<p>No data available.</p>
	{/if}
</div>

<style lang="scss">

</style>