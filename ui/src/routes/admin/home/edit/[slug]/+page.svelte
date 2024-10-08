<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { Home } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import CreateOrEditHome from '$lib/components/dashboard/CreateOrEditHome.svelte';
	import Loading from '$lib/components/Loading.svelte';

	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;
	// The ID of the Home data resource
	let id = $page.params.slug;
	// content is the actual content of the Markdown document.
	let content = '';

	onMount(() => {
		Home.retrieve(id, 'zh')
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => content = e?.data.data ?? '')
			).subscribe({ error: console.error });
	});
</script>

{#if isLoading}
	<Loading />
{:else }
	<CreateOrEditHome content={content} id={id} />
{/if}