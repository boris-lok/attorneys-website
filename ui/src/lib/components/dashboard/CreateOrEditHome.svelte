<script lang="ts">
	import TextArea from '$lib/components/TextArea.svelte';
	import { t } from 'svelte-i18n';
	import SvelteMarkdown from 'svelte-markdown';
	import type { Language } from '$lib/models/Language';
	import { Home } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize } from 'rxjs';
	import { browser } from '$app/environment';

	export let id = '';
	export let content = '';

	let language: Language = 'zh';
	let isLoading = false;

	// An event handler handles the textarea changed event
	//
	// Update the content -> show the new content in preview section
	function onContentChanged(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		content = target.value;
	}

	function validate() {
		return content !== '';
	}

	// An event handler handles `create/update` home resource
	function onSubmitButtonClicked() {
		if (isLoading) {
			return;
		}

		if (!validate()) {
			return;
		}

		let json = {
			...id !== '' ? { id: id } : {},
			data: content,
			language: language
		};

		Home.save(json)
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false)
			)
			.subscribe();
	}

	// An event handler handles `back` button click
	function onBackButtonClicked() {
		if (browser) {
			window.history.back();
		}
	}
</script>

<div class="create-edit-home-wrapper">
	<div class="edit-section">
		<TextArea data={content} label={$t('home')} on:input={onContentChanged} />
	</div>
	<div class="btn-container">
		<button class="btn submit" disabled={isLoading} on:click={onSubmitButtonClicked} type="button">{$t('save')}</button>
		<button class="btn back" disabled={isLoading} on:click={onBackButtonClicked} type="button">{$t('back')}</button>
	</div>
	<div class="preview-section add-margin-to-listview">
		<SvelteMarkdown source={content} />
	</div>
</div>

<style lang="scss">
  .create-edit-home-wrapper {
    display: grid;
    grid-template:
		'edit-section'
		'btn-container'
		'preview-section';
    row-gap: 1rem;
    padding: 1.25rem 0.75rem;

    .edit-section {
      grid-area: edit-section;
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

    .preview-section {
      grid-area: preview-section;
    }
  }

  @media (min-width: 768px) {
    .create-edit-home-wrapper {
      grid-template:
		  'edit-section preview-section'
			'btn-container btn-container';
      width: 100%;

      .edit-section {
        width: calc(50vw - 2rem);
        margin: 0 1rem;
      }

      .preview-section {
        margin: 0 1rem;
      }
    }
  }
</style>