<script lang="ts">

	import type { Language } from '$lib/models/Language';
	import { browser } from '$app/environment';
	import Input from '$lib/components/Input.svelte';
	import { t } from 'svelte-i18n';
	import TextArea from '$lib/components/TextArea.svelte';
	import SvelteMarkdown from 'svelte-markdown';
	import { Services } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize } from 'rxjs';
	import SpinningLoading from '$lib/components/SpinningLoading.svelte';

	// If id is not empty, we update the contact resource by id.
	// Otherwise, we create a new service
	export let id = '';
	// The title of the service
	export let title = '';
	// The description of the service
	export let data = '';

	let language: Language = 'zh';
	let isLoading = false;


	// An event handler handles the title changed
	function onTitleChanged(e: Event) {
		title = (e.target as HTMLInputElement).value;
	}

	// An event handler handles the data changed
	function onDataChanged(e: Event) {
		data = (e.target as HTMLInputElement).value;
	}

	// An event handler handles `back` button click
	function onBackButtonClicked() {
		if (browser) {
			window.history.back();
		}
	}

	// validate the service data
	function validate() {
		return title.trim() !== '' && data.trim() !== '';
	}

	// An event handler handles `save` button click
	function onSubmitButtonClicked() {
		if (isLoading) {
			return;
		}

		if (!validate()) {
			return;
		}

		let json = {
			...id !== '' ? { id: id } : {},
			title: title,
			data: data,
			language: language
		};

		Services.save(json).pipe(
			startWithTap(() => isLoading = true),
			finalize(() => isLoading = false)
		)
			.subscribe();
	}
</script>

<div class="form-wrapper">

	<div class="full-page-loading-wrapper" class:active={isLoading}>
		<SpinningLoading isLoading={isLoading} />
	</div>

	<div class="edit-section">
		<Input label={$t('service.title')} name="name" on:input={onTitleChanged} value={title} />
		<TextArea data={data} label={$t('service.data')} on:input={onDataChanged} />
	</div>

	<div class="btn-container">
		<button class="btn submit" disabled={isLoading} on:click={onSubmitButtonClicked} type="button">{$t('save')}</button>
		<button class="btn back" disabled={isLoading} on:click={onBackButtonClicked} type="button">{$t('back')}</button>
	</div>

	<div class="preview-section add-margin-to-listview">
		<SvelteMarkdown source={data} />
	</div>
</div>

<style lang="scss">
  .form-wrapper {
    display: grid;
    grid-template:
		'edit-section'
		'btn-container'
		'preview-section';
    row-gap: 1rem;
    padding: 1.25rem 0.75rem;
  }

  .edit-section {
    grid-area: edit-section;
  }

  .preview-section {
    grid-area: preview-section;
  }

  .btn-container {
    grid-area: btn-container;
    text-align: center;

    .btn {
      width: 7.5rem;
      height: 2.5rem;
      cursor: pointer;
      margin: 0 0.25rem;

      &.submit {
        border: 1px solid $deep-blue;
      }

      &.back {
        border: 1px solid $deep-red;
      }
    }
  }

  @media (min-width: 768px) {
    .form-wrapper {
      grid-template:
		  'edit-section preview-section'
			'btn-container btn-container';
      width: 100%;

      .edit-section {
        width: calc(50vw - 2rem);
        margin: 0 1rem;
      }

      .preview-section {
        width: calc(50vw - 2rem);
        margin: 0 1rem;
        padding: 0;
      }
    }
  }
</style>