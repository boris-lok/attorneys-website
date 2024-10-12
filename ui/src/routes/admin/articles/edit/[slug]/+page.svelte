<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { Articles } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import Loading from '$lib/components/Loading.svelte';
	import type { Language } from '$lib/models/Language';
	import type { ArticleData } from '$lib/models/Articles';
	import CreateOrEditArticle from '$lib/components/dashboard/CreateOrEditArticle.svelte';
	import { t } from 'svelte-i18n';

	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;
	// The ID of the Home data resource
	let id = $page.params.slug;
	// content is the actual content of the Markdown document.
	let article: ArticleData | null = null;
	let language: Language = 'zh';

	onMount(() => {
		Articles.retrieve(id, language)
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => article = e)
			).subscribe({ error: console.error });
	});
</script>

{#if isLoading}
	<Loading />
{:else if article}
	<CreateOrEditArticle title={article.data.title} content={article.data.content} id={id} />
{:else}
	<p>{$t('no_data_available')}</p>
{/if}