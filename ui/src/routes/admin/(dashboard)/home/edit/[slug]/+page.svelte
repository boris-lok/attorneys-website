<script lang="ts">
	import type { PageProps } from './$types';
	import HomeEditor from '$lib/components/dashboard/HomeEditor.svelte';
	import { HomeServices } from '$lib/services/home.service';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import Loading from '$lib/components/common/Loading.svelte';

	let { data }: PageProps = $props();

	let isLoading = $state(false);
	let content = $state('');

	function fetchData() {
		console.log(data.id);
		HomeServices.retrieve(data.id, 'zh')
			.pipe(
				startWithTap(() => (isLoading = true)),
				finalize(() => (isLoading = false)),
				tap((resp) => {
					content = resp?.data.data ?? '';
				})
			)
			.subscribe({
				error: console.error
			});
	}

	$effect(() => fetchData());
</script>

{#if isLoading}
	<Loading />
{:else}
	<HomeEditor data={content} id={data.id} />
{/if}
