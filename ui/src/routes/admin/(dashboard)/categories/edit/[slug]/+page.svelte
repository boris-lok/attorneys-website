<script lang="ts">
	import type { PageProps } from './$types';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import CategoryEditor from '$lib/components/dashboard/CategoryEditor.svelte';
	import { CategoryService } from '$lib/services/category.service';
	import Loading from '$lib/components/common/Loading.svelte';

	let { data }: PageProps = $props();

	let isLoading = $state(false);
	let icon: string | undefined = $state(undefined);
	let name = $state('');

	function fetchData() {
		CategoryService.retrieve(data.id, 'zh')
			.pipe(
				startWithTap(() => (isLoading = true)),
				finalize(() => (isLoading = false)),
				tap((resp) => {
					icon = resp?.data.icon;
					name = resp?.data.name ?? '';
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
	<CategoryEditor icon={icon} name={name} />
{/if}
