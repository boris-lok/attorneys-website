<script lang="ts">
	import type { Language } from '$lib/models/Language';
	import { onMount } from 'svelte';
	import { Members } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize, mergeMap, tap } from 'rxjs';
	import type { SimpleMember } from '$lib/models/Member';
	import { t } from 'svelte-i18n';
	import SpinningLoading from '$lib/components/SpinningLoading.svelte';

	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;
	// The language that we want to fetch the resources from API
	let language: Language = 'zh';
	// The data that we want to display
	let data: SimpleMember[] = [];

	/**
	 * Function to handle the deletion of a member
	 * @param memberId The id of the member to be deleted
	 */
	function onDeleteButtonClicked(memberId: string) {
		Members.delete(memberId)
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				mergeMap(_ => {
					return Members.list(language)
						.pipe(
							tap(e => {
								data = e;
							})
						);
				})
			)
			.subscribe();
	}

	onMount(() => {
		Members.list(language)
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

	<h2 class="title">{$t('members')}</h2>

	<div class="function-tools-wrapper">
		<a class="btn green" href="/admin/members/create">
			<span class="material-icon">add_circle</span>
			<span>{$t('create')}</span>
		</a>
	</div>

	<div class="loading-wrapper">
		<SpinningLoading isLoading={isLoading} />
	</div>

	{#if data.length > 0}
		<div class="list-section">
			{#each data as member}
				<div class="content-section">
					<p class="content-title">{member.name}</p>
					<a class="btn blue" href="/admin/members/edit/{member.id}">
						<span class="material-icon">edit_document</span>
						<span>{$t('edit')}</span>
					</a>
					<button class="btn red" on:click={() => onDeleteButtonClicked(member.id)}>
						<span class="material-icon">delete</span>
						<span>{$t('edit')}</span>
					</button>
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
      max-height: 12rem;;
      overflow: auto;

      .content-section {
        position: relative;
        padding: 0.5rem 1rem;
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin: 0.5rem 0.5rem;
        border-radius: 4px;
        box-shadow: 0 0 4px 0 $deep-grey;
        gap: 0.5rem;

        .content-title {
          width: calc(100% - 40px);
          font-size: 1rem;
          font-weight: bold;
        }

        .btn {
          text-decoration: none;
          gap: 0.25rem;
          display: flex;
          flex-direction: row;
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
        flex-direction: row;
        overflow-y: scroll;

        .content-section {
          min-width: 350px;
        }
      }
    }
  }
</style>