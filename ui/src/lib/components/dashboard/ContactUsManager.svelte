<script lang="ts">
	import { onMount } from 'svelte';
	import type { ContactData } from '$lib/models/ContactUs';
	import { Contacts } from '$lib/services';
	import type { Language } from '$lib/models/Language';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import { t } from 'svelte-i18n';

	let language: Language = 'zh';

	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;
	// data is dataset that represents the contact information
	let data: ContactData | null = null;
	// hasData is a flag that indicates whether
	// we have received a valid response from the API / doesn't have any data.
	let hasData = false;

	onMount(() => {
		Contacts.list(language)
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => {
					hasData = !!e;
					data = e;
				})
			)
			.subscribe({ error: console.error });
	});
</script>

<div class="wrapper">
	<h2 class="title">{$t('contact_us')}</h2>
	<div class="function-tools-wrapper">
		{#if hasData && data}
			<a href="/admin/contact/edit/{data.id}" class="btn blue">
				<span class="material-icon">edit_document</span>
				<span>{$t('edit')}</span>
			</a>
		{:else}
			<a href="/admin/contact/create" class="btn green">
				<span class="material-icon">add_circle</span>
				<span>{$t('create')}</span>
			</a>
		{/if}
	</div>
	{#if isLoading}
		<p>{$t('loading')}...</p>
	{:else if data}
		<div class="contact-info">
			<div class="block">
				<p><strong>{$t('address')}:</strong>&nbsp;{data.data.address}</p>
			</div>
			<div class="block">
				<p><strong>{$t('phone')}:</strong>&nbsp;{data.data.phone}</p>
			</div>
			<div class="block">
				<p><strong>{$t('email')}:</strong>&nbsp;{data.data.email}</p>
			</div>
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

        &.blue {
          color: $deep-blue;
        }

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
    }
  }
</style>