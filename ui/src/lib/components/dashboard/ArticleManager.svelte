<script lang="ts">
	import { t } from 'svelte-i18n';
	import { onDestroy, onMount } from 'svelte';
	import { Articles } from '$lib/services';
	import type { Language } from '$lib/models/Language';
	import { startWithTap } from '$lib/utils';
	import { BehaviorSubject, finalize, mergeMap, Subscription, tap } from 'rxjs';
	import type { ArticleData } from '$lib/models/Articles';
	import SpinningLoading from '$lib/components/SpinningLoading.svelte';

	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;
	// All Services data
	let data: ArticleData[] = [];
	// The language that we want to load
	let language: Language = 'zh';

	// Pagination
	let page = 0;
	// Observable for the current page number. We use BehaviorSubject to ensure that the page number is updated correctly when we navigate to a new page.
	const page$ = new BehaviorSubject(0);
	// The size of the articles per page. We define it as 5.
	let pageSize = 5;
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

	// handle delete button clicked
	function onDeleteButtonClicked(id: string) {
		Articles.delete(id)
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				mergeMap(_ => {
					// reload the page
					return Articles.list(language, page, pageSize)
						.pipe(
							tap(e => {
								data = e.articles;
								hasPreviousPage = page > 0;
								hasNextPage = (e.total - ((page + 1) * pageSize)) > 0;
							})
						);
				})
			)
			.subscribe();
	}

	onMount(() => {
		disposer = page$
			.subscribe(p => {
				Articles.list(language, p, pageSize)
					.pipe(
						startWithTap(() => isLoading = true),
						finalize(() => isLoading = false),
						tap(e => {
							data = e.articles;
							hasPreviousPage = page > 0;
							hasNextPage = (e.total - ((page + 1) * pageSize)) > 0;
						})
					)
					.subscribe();
			});
	});

	onDestroy(() => disposer?.unsubscribe());
</script>

<div class="wrapper">

	<h2 class="title">{$t('articles')}</h2>

	<div class="function-tools-wrapper">
		<a class="btn green" href="/admin/articles/create">
			<span class="material-icon">add_circle</span>
			<span>{$t('create')}</span>
		</a>
	</div>

	<div class="loading-wrapper">
		<SpinningLoading isLoading={isLoading} />
	</div>

	{#if data.length > 0}
		<div class="list-section">
			{#each data as article, i}
				<div class="content-section">
					<p class="content-title">{article.data.title}</p>
					<div class="tools-group">
						<a class="btn blue" href="/admin/articles/edit/{article.id}">
							<span class="material-icon">edit_document</span>
							<span>{$t('edit')}</span>
						</a>
						<button class="btn red" on:click={() => onDeleteButtonClicked(article.id)}>
							<span class="material-icon">delete</span>
							<span>{$t('edit')}</span>
						</button>
					</div>
				</div>
			{/each}
		</div>
		<div class="pagination-wrapper">
			<button class="btn" on:click={onPreviousButtonClicked} class:disabled={!hasPreviousPage}
							disabled={!hasPreviousPage}>Previous
			</button>
			<button class="btn" on:click={onNextButtonClicked} class:disabled={!hasNextPage} disabled={!hasNextPage}>Next
			</button>
		</div>
	{:else}
		<p class="no-data">{$t('no_data_available')}</p>
	{/if}
</div>

<style lang="scss">
  .wrapper {
    width: 100%;
    position: relative;
    padding: 1rem 5%;

    .title {
      font-size: 2rem;
      text-align: center;
      margin: 0;
      border-bottom: 1px solid $black;
    }

    .loading-wrapper {
      display: flex;
      justify-content: center;
    }

    .function-tools-wrapper {
      display: flex;
      flex-direction: row;
      justify-content: flex-end;
      padding: 0.5rem 0.5rem;
      position: absolute;
      right: 0;
      top: 3rem;

      .btn {
        text-decoration: none;
        gap: 0.25rem;
        display: flex;
        flex-direction: row;

        &.green {
          color: $deep-green;
        }

        span:nth-child(2) {
          display: none;
        }
      }
    }

    .pagination-wrapper {
      display: flex;
      width: 100%;
      flex-direction: row;
      gap: 1rem;
      justify-content: center;
			padding: 1rem 0;

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

    .list-section {
      display: flex;
      flex-direction: column;
      gap: 1rem;

      .content-section {
        position: relative;
        padding: 0 1rem;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;

        .content-title {
          width: 100%;
          max-width: 60vw;
          overflow: hidden;
          text-overflow: ellipsis;
          font-size: 1rem;
          font-weight: bold;
        }

        .tools-group {
          display: flex;
          flex-direction: row;
          gap: 0.5rem;
        }

        .btn {
          text-decoration: none;
          gap: 0.25rem;
          display: flex;
          flex-direction: row;
          outline: none;
          border: 0;
          background-color: transparent;
          cursor: pointer;

          &.blue {
            color: $deep-blue;
          }

          &.red {
            color: $deep-red;
          }

          span:nth-child(2) {
            display: none;
          }
        }
      }
    }

    .no-data {
      text-align: center;
      font-size: 1.25rem;
    }
  }

  @media (min-width: 768px) {
    .wrapper {
      .function-tools-wrapper {
        padding: 0.5rem 1.25rem;
        right: 5%;

        .btn {
          span:nth-child(2) {
            display: block;
          }
        }
      }

      .list-section {
        flex-direction: column;
        height: 20rem;
        overflow-y: scroll;

        .content-section {
        }
      }
    }
  }
</style>