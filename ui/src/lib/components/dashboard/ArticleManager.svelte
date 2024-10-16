<script lang="ts">
	import { t } from 'svelte-i18n';
	import { onMount } from 'svelte';
	import { Articles } from '$lib/services';
	import type { Language } from '$lib/models/Language';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import type { ArticleData } from '$lib/models/Articles';
	import SpinningLoading from '$lib/components/SpinningLoading.svelte';

	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;
	// All Services data
	let data: ArticleData[] = [];
	let language: Language = 'zh';

	onMount(() => {
		Articles.list(language)
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
					<a class="btn blue" href="/admin/articles/edit/{article.id}">
						<span class="material-icon">edit_document</span>
						<span>{$t('edit')}</span>
					</a>
				</div>
			{/each}
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

    .list-section {
      display: flex;
      flex-direction: column;
      gap: 1rem;

      .content-section {
        position: relative;
        height: 200px;

        .content-title {
          width: calc(100% - 40px);
          font-size: 1rem;
          font-weight: bold;
        }

        .btn {
          position: absolute;
          top: 1.25rem;
          right: 1rem;
          text-decoration: none;
          gap: 0.25rem;
          display: flex;
          flex-direction: row;

          &.blue {
            color: $deep-blue;
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
        right: 1.25rem;

        .btn {
          span:nth-child(2) {
            display: block;
          }
        }
      }

      .list-section {
        flex-direction: column;
        height: 10rem;
        overflow-y: scroll;

        .content-section {
        }
      }
    }
  }
</style>