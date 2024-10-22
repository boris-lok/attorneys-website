<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { Articles } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { BehaviorSubject, finalize, Subscription, tap } from 'rxjs';
	import Loading from '$lib/components/Loading.svelte';
	import { t } from 'svelte-i18n';
	import type { Language } from '$lib/models/Language';
	import type { ArticleData } from '$lib/models/Articles';

	let isLoading = false;
	let articles: ArticleData[] = [];
	let language: Language = 'zh';

	// Pagination
	let page = 0;
	// Observable for the current page number. We use BehaviorSubject to ensure that the page number is updated correctly when we navigate to a new page.
	const page$ = new BehaviorSubject(0);
	// The size of the articles per page. We define it as 5.
	let pageSize = 10;
	// The flag indicates that we have previous page
	let hasPreviousPage = false;
	// The flag indicates that we have next page
	let hasNextPage = false;
	// The disposer for the subscription. We use it to unsubscribe when the component is destroyed.
	let disposer: Subscription | null = null;

	function onPreviousButtonClicked() {
		page = page - 1;
		page$.next(page);
	}

	function onNextButtonClicked() {
		page = page + 1;
		page$.next(page);
	}

	onMount(() => {
		disposer = page$
			.subscribe(p => {
				Articles.list(language, p, pageSize)
					.pipe(
						startWithTap(() => isLoading = true),
						finalize(() => isLoading = false),
						tap(e => {
							articles = e.articles;
							hasPreviousPage = page > 0;
							hasNextPage = (e.total - ((page + 1) * pageSize)) > 0;
						})
					)
					.subscribe();
			});
	});

	onDestroy(() => disposer?.unsubscribe());
</script>

{#if isLoading}
	<Loading />
{:else}
	<div class="articles-section">
		<h1>{$t('articles')}</h1>
		{#each articles as article}
			<div class="article-wrapper">
				<a href="/articles/{article.id}">
					<h3>{article.data.title}</h3>
				</a>
			</div>
		{/each}
		<div class="pagination-wrapper">
			<button class="btn" on:click={onPreviousButtonClicked} class:disabled={!hasPreviousPage}
							disabled={!hasPreviousPage}>Previous
			</button>
			<button class="btn" on:click={onNextButtonClicked} class:disabled={!hasNextPage} disabled={!hasNextPage}>Next
			</button>
		</div>
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

    .pagination-wrapper {
      display: flex;
      width: 100%;
      flex-direction: row;
      gap: 1rem;
      justify-content: center;

      .btn {
        border: none;
        background-color: transparent;
        text-decoration: underline;
        cursor: pointer;

        &.disabled {
          color: $grey;
          cursor: unset;
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
      overflow: clip;
      margin: 0 auto;

      h1 {
        padding: 0 0 1rem 0;
        line-height: 2rem;
      }
    }
  }
</style>