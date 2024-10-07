<script lang="ts">
	import { onMount } from 'svelte';
	import { Articles } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import type { SimpleArticle } from '$lib/models/Articles';
	import Loading from '$lib/components/Loading.svelte';
	import { t } from 'svelte-i18n';

	let isLoading = false;
	let articles: SimpleArticle[] = [];

	onMount(() => {
		Articles.list()
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(response => articles = response)
			)
			.subscribe();
	});
</script>

{#if isLoading}
	<Loading />
{:else}
	<div class="articles-section">
		<h1>{$t('articles')}</h1>
		{#each articles as article}
			<div class="article-wrapper">
				<a href="/app/articles/{article.id}">
					<h3>{article.title}</h3>
				</a>
			</div>
		{/each}
	</div>
{/if}

<style lang="scss">
  .articles-section {
    display: flex;
    flex-direction: column;
    gap: .5rem;
    padding: 0 5%;
    width: 100%;

    .article-wrapper {
      width: 100%;

      a {
        display: inline-block;
        padding: 0.25rem 0.5rem;
        width: 100%;
        text-decoration: none;
        color: $black;
        border-bottom: 1px solid $grey;

        h3 {
          font-size: 1rem;
          font-weight: 500;
          line-height: 1.25rem;
        }

        &:hover {
          color: $deep-orange;
        }
      }
    }
  }

  @media (min-width: 768px) {
    .articles-section {
      justify-content: center;
      align-items: center;
      padding: 0 10%;
      max-width: 1024px;
      width: 1024px;
      overflow: clip;

      h1 {
        padding: 0 0 1rem 0;
        line-height: 2rem;
      }
    }
  }
</style>