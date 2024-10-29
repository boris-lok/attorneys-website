<script lang="ts">
	import { t } from 'svelte-i18n';
	import type { ServiceData } from '$lib/models/Services';
	import { onMount } from 'svelte';
	import { Services } from '$lib/services';
	import type { Language } from '$lib/models/Language';
	import { startWithTap, text_overflow } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import SvelteMarkdown from 'svelte-markdown';
	import SpinningLoading from '$lib/components/SpinningLoading.svelte';

	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;
	// All Services data
	let data: ServiceData[] = [];
	let language: Language = 'zh';

	onMount(() => {
		Services.list(language)
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

	<h2 class="title">{$t('services')}</h2>

	<div class="function-tools-wrapper">
		<a class="btn green" href="/admin/services/create">
			<span class="material-icon">add_circle</span>
			<span>{$t('create')}</span>
		</a>
	</div>

	<div class="loading-wrapper">
		<SpinningLoading isLoading={isLoading} />
	</div>

	{#if data.length > 0}
		<div class="list-section">
			{#each data as service, i}
				<div class="content-section">
					<h3>{service.data.title}</h3>
					<div class="add-margin-to-listview">
						<SvelteMarkdown source={text_overflow(service.data.data, 50)} />
					</div>
					<a class="btn blue" href="/admin/services/edit/{service.id}">
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
      max-height: 300px;
      overflow: scroll;

      .content-section {
        position: relative;
        margin: 0.5rem 0.5rem;
        border-radius: 4px;
        box-shadow: 0 0 4px 0 $deep-grey;
        padding: 0.5rem 1rem;

        h3 {
          width: calc(100% - 40px);
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
        right: 5%;

        .btn {
          span:nth-child(2) {
            display: block;
          }
        }
      }

      .list-section {
        flex-direction: row;
        width: 100%;
        overflow-x: scroll;
        height: fit-content;

        .content-section {
          min-width: 350px;
        }
      }
    }
  }
</style>