<script lang="ts">
	import TextArea from '$lib/components/TextArea.svelte';
	import { t } from 'svelte-i18n';
	import SvelteMarkdown from 'svelte-markdown';
	import type { Language } from '$lib/models/Language';
	import { Home } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize } from 'rxjs';

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

	function onSubmitClicked() {
		if (isLoading) {
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
</script>

<div class="create-edit-home-wrapper">
	<div class="edit-section">
		<TextArea data={content} label={$t('home')} on:input={onContentChanged} />
	</div>
	<div class="btn-container">
		<button disabled={isLoading} on:click={onSubmitClicked} type="button">{$t('save')}</button>
	</div>
	<div class="preview-section">
		<SvelteMarkdown source={content} />
	</div>
</div>

<style lang="scss">
  .create-edit-home-wrapper {
    .preview-section {
      padding: 1rem;
      margin-top: 1rem;
    }
  }
</style>