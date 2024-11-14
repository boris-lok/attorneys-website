<script lang="ts">

	import type { Language } from '$lib/models/Language';
	import { browser } from '$app/environment';
	import Input from '$lib/components/Input.svelte';
	import { t } from 'svelte-i18n';
	import TextArea from '$lib/components/TextArea.svelte';
	import SvelteMarkdown from 'svelte-markdown';
	import { Articles } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize } from 'rxjs';
	import SpinningLoading from '$lib/components/SpinningLoading.svelte';
	import NumberInput from '$lib/components/NumberInput.svelte';

	// If id is not empty, we update the resource by id.
	// Otherwise, we create a new article
	export let id = '';
	// The title of the article
	export let title = '';
	// The description of the article
	export let content = '';
	// The sequence of the article
	export let seq = 0;

	let language: Language = 'zh';
	let isLoading = false;


	// An event handler handles the title changed
	function onTitleChanged(e: Event) {
		title = (e.target as HTMLInputElement).value;
	}

	// An event handler handles the data changed
	function onContentChanged(e: Event) {
		content = (e.target as HTMLInputElement).value;
	}

	// An event handler handles the sequence changed
	function onSeqChanged(e: Event) {
		seq = parseInt((e.target as HTMLInputElement).value);
	}

	// An event handler handles `back` button click
	function onBackButtonClicked() {
		if (browser) {
			window.history.back();
		}
	}

	// validate the service data
	function validate() {
		const validSeq = !isNaN(seq) && seq <= 32767 && seq >= -32768;
		return title.trim() !== '' && content.trim() !== '' && validSeq;
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
			content: content,
			seq: seq,
			language: language
		};

		Articles.save(json).pipe(
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
		<Input label={$t('article.title')} name="name" on:input={onTitleChanged} value={title} />
		<TextArea data={content} label={$t('article.data')} on:input={onContentChanged} />
		<NumberInput label={$t('seq')} name="seq" on:input={onSeqChanged} placeholder={$t("seq.warning")} value={seq} />
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
    display: flex;
    flex-direction: column;
    row-gap: 1rem;
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