<script lang="ts">
	import Input from '$lib/components/Input.svelte';
	import { t } from 'svelte-i18n';
	import TextArea from '$lib/components/TextArea.svelte';
	import UploadImage from '$lib/components/UploadImage.svelte';
	import { browser } from '$app/environment';
	import SvelteMarkdown from 'svelte-markdown';
	import { Members } from '$lib/services';
	import type { Language } from '$lib/models/Language';
	import { startWithTap } from '$lib/utils';
	import { finalize, mergeMap, of } from 'rxjs';
	import type { AvatarData } from '$lib/models/Member';
	import SpinningLoading from '$lib/components/SpinningLoading.svelte';

	// If id is not empty, we update the member resource by id.
	// Otherwise, we create a new member
	export let id = '';
	// The name of the staff member
	export let name = '';
	// The description of the staff member
	export let description = '';
	// The avatar URL of the staff member
	export let avatarData: AvatarData | null;
	// The updated avatar of the staff member
	let avatar: File | null = null;
	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;
	// The language of the resource
	let language: Language = 'zh';

	// Handle image changed
	function onImageChanged(e: CustomEvent) {
		avatar = e.detail.file;
	}

	// Handle name changed
	function onNameChanged(e: Event) {
		const target = e.target as HTMLInputElement;
		name = target.value.trim();
	}

	// Handle description changed
	function onDescriptionChanged(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		description = target.value.trim();
	}

	// Check the name and description is not empty
	function validate() {
		return name.trim() !== '' && description !== '';
	}

	function onSubmitButtonClicked() {
		if (isLoading) {
			return;
		}

		if (!validate()) {
			return;
		}

		let json = {
			...id !== '' ? { id: id } : {},
			name: name,
			description: description,
			language: language
		};

		Members.save(json)
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				mergeMap(e => {
					if (e !== null && avatar instanceof File && avatar) {
						return Members.uploadAvatar(e, avatar);
					}
					return of(e);
				})
			)
			.subscribe({
				error: console.error
			});

	}

	// An event handler handles `back` button click
	function onBackButtonClicked() {
		if (browser) {
			window.history.back();
		}
	}
</script>

<div class="wrapper">
	<div class="full-page-loading-wrapper" class:active={isLoading}>
		<SpinningLoading isLoading={isLoading} />
	</div>

	<UploadImage avatarData={avatarData} on:change={onImageChanged} />
	<div class="form-wrapper">
		<div class="edit-section">
			<Input label={$t('member.name')} name="name" on:input={onNameChanged} value={name} />
			<TextArea data={description} label={$t('member.description')} on:input={onDescriptionChanged} />
		</div>

		<div class="btn-container">
			<button class="btn submit" disabled={isLoading} on:click={onSubmitButtonClicked}
							type="button">{$t('save')}</button>
			<button class="btn back" disabled={isLoading} on:click={onBackButtonClicked} type="button">{$t('back')}</button>
		</div>

		<div class="preview-section add-margin-to-listview">
			<SvelteMarkdown source={description} />
		</div>
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