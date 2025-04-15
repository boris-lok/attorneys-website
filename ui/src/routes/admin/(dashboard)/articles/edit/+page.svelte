<script lang="ts">
	import ArticleEditor from '$lib/components/dashboard/ArticleEditor.svelte';
	import { CategoryService } from '$lib/services/category.service';
	import type { CategoryData } from '$lib/types';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import Loading from '$lib/components/common/Loading.svelte';

	let isLoading = $state(false);
	let categories: CategoryData[] = $state([]);

	function fetchCategories() {
		CategoryService.list('zh')
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap((resp) => {
					categories = resp;
				})
			)
			.subscribe(
				{ error: console.error }
			);
	}

	$effect(() => fetchCategories());
</script>

{#if isLoading}
	<Loading />
{:else}
	<ArticleEditor {categories} />

{/if}