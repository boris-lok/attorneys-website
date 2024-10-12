<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { Articles } from '$lib/services';
	import type { Article } from '$lib/models/Articles';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import Loading from '$lib/components/Loading.svelte';
	import { t } from 'svelte-i18n';
	import SvelteMarkdown from 'svelte-markdown';

	let article_id = $page.params.slug;
	let isLoading = false;
	let article: Article;

	onMount(() => {
		Articles
			.retrieve(article_id)
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => article = e)
			)
			.subscribe();
	});

</script>

{#if isLoading}
	<Loading />
{:else}
	<div class="article-section">
		{#if article}
			<h1>{article.title}</h1>
			<SvelteMarkdown source={article.content} />
		{:else}
			<p>{$t('no_article_message')}</p>
		{/if}
	</div>
{/if}

<style lang="scss">
  .article-section {
    display: flex;
    flex-direction: column;
    padding: 0 5% 1.25rem 5%;

    p {
      text-align: center;
    }
  }

  @media (min-width: 768px) {
    .article-section {
      max-width: 1024px;
      padding: 0 10% 1.5rem 10%;
      margin: 0 auto;
    }
  }
</style>