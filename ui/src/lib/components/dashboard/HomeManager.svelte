<script lang="ts">
	import { onMount } from 'svelte';
	import { Home } from '$lib/services';
	import { startWithTap, text_overflow } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import { t } from 'svelte-i18n';
	import SvelteMarkdown from 'svelte-markdown';

	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;
	// content is the actual content of the Markdown document.
	let content = '';
	// hasData is a flag that indicates whether
	// we have received a valid response from the API / doesn't have any data.
	let hasData = false;
	// The home resource ID
	let id = '';

	onMount(() => {
		Home.list('zh')
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => {
					hasData = e !== null;
					id = e?.id ?? '';
					content = e?.data.data ?? '';
					content = text_overflow(content, 100);
				})
			)
			.subscribe();
	});
</script>

<div class="home-wrapper">
	<h2 class="title">{$t('home')}</h2>
	<div class="home-content-wrapper">
		<div class="function-tools-wrapper">
			{#if hasData}
				<a href="/admin/home/edit/{id}" class="btn">
					<span class="material-icon">edit_document</span>
					<span>{$t('edit')}</span>
				</a>
			{:else}
				<a href="/admin/home/create" class="btn">
					<span class="material-icon">add_circle</span>
					<span>{$t('create')}</span>
				</a>
			{/if}
		</div>
		{#if isLoading}
			<p>{$t('loading')}...</p>
		{:else if content.length > 0}
			<SvelteMarkdown source={content} />
		{:else}
			<p class="no-data">{$t('no_data_available')}</p>
		{/if}
	</div>
</div>

<style lang="scss">
  .home-wrapper {
    .title {
      font-size: 2rem;
      text-align: center;
      margin: 0;
      border-bottom: 1px solid $black;
    }

    .home-content-wrapper {
      display: flex;
      flex-direction: column;
      width: 100%;
    }

    .function-tools-wrapper {
      display: flex;
      flex-direction: row;
      justify-content: flex-end;
      padding: 0.5rem 0.5rem;

      .btn {
        text-decoration: none;
        gap: 0.25rem;
        display: flex;
        flex-direction: row;
        color: $deep-orange;

        span:nth-child(2) {
          display: none;
        }
      }
    }

    .no-data {
      text-align: center;
      font-size: 1.25rem;
    }
  }

  @media (min-width: 768px) {
    .home-wrapper {
      .function-tools-wrapper {
        padding: 0.5rem 1.25rem;

        .btn {
          span:nth-child(2) {
            display: block;
          }
        }
      }
    }
  }
</style>