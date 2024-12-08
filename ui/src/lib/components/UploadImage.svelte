<script lang="ts">
	import { t } from 'svelte-i18n';
	import iterate from 'iterare';
	import { createEventDispatcher, onMount } from 'svelte';
	import { BehaviorSubject } from 'rxjs';
	import type { AvatarData } from '$lib/models/Member';
	import Avatar from '$lib/components/Avatar.svelte';

	// The avatarUrl that displays the avatar
	export let avatarData: AvatarData | null = null;
	// The flag is used to indicate that user is dragging
	let isDragging = false;
	// The image is used to display
	let image: HTMLElement;
	// The event dispatcher to let parent component handle the event
	const dispatch = createEventDispatcher();
	// The flag is used to indicate that user has selected an image
	let hasImage = false;
	// The file selected by the user
	let file = new BehaviorSubject<File | undefined>(undefined);

	onMount(() => {
		// listen to file has been changed
		const disposer = file.subscribe({
			next: (f) => {
				hasImage = !!f;
				dispatch('change', {
					file: f
				});
			}
		});

		// disposer is responsible for the component has been removed
		return () => {
			disposer.unsubscribe();
		};
	});

	// Handle file drop
	function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragging = false;

		// Extract files from event
		const files = e.dataTransfer?.files ?? ([] as File[]);

		handleInputFiles(files);
	}

	// Prevent default behavior for dragover and dragenter
	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		isDragging = true;
	}

	// Handle file drag leave event
	function handleDragLeave(e: DragEvent) {
		e.preventDefault();
		isDragging = false;
	}

	function generatePreview(file: File) {
		const reader = new FileReader();
		reader.onload = function(event: ProgressEvent) {
			if (typeof reader.result === 'string') {
				image.setAttribute('src', reader.result);
			}
		};
		reader.readAsDataURL(file);
	}

	function onInputChanged(e: Event) {
		const files = (e.target as HTMLInputElement)?.files ?? ([] as File[]);
		avatarData = null;
		handleInputFiles(files);
	}

	function handleInputFiles(files: File[] | FileList) {
		let newFile = undefined;
		if (files.length > 0) {
			newFile = iterate(files)
				.find(e => e.type.startsWith('image/'));
			if (newFile) {
				generatePreview(newFile);
			}
		}
		file.next(newFile);
	}

	// delete the image that user has selected
	function onDeleteClicked() {
		file.next(undefined);
	}

	// delete the avatar data
	function onAvatarDeleteButtonClicked() {
		avatarData = null;
	}
</script>

<div class="image-wrapper">
	{#if !hasImage && avatarData === null}
		<div class="dropzone-wrapper" on:drop={handleDrop} on:dragover={handleDragOver} on:dragleave={handleDragLeave}
				 class:is-dragging={isDragging}>
			<div class="dropzone-desc">
				<p>{$t('upload_image_hint')}</p>
			</div>
			<input type="file" name="image" class="dropzone" on:change={onInputChanged} />

		</div>
	{/if}
	{#if avatarData}
		<div class="avatar-section">
			<Avatar avatar={avatarData} />
			<i class="material-icon" on:click={onAvatarDeleteButtonClicked}>delete</i>
		</div>
	{/if}
	{#if hasImage}
		<div class="preview-zone-wrapper">
			<img src="" bind:this={image} alt="avatarPreview" width="256" height="256">
			<i class="material-icon" on:click={onDeleteClicked}>delete</i>
		</div>
	{/if}
</div>


<style lang="scss">
  .image-wrapper {
    width: 100%;
    overflow: clip;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;

    .avatar-section {
      display: flex;
      flex-direction: row;
      gap: 2rem;
      align-items: center;
    }

    i {
      border-radius: 50%;
      cursor: pointer;
      color: $deep-red;
      box-shadow: 0 0 4px 1px $grey;
      padding: 6px;
    }

    .dropzone-wrapper {
      border: 2px dashed $grey;
      color: $black;
      position: relative;
      width: 100%;
      box-sizing: border-box;
      height: 144px;
      display: flex;
      align-items: center;
      justify-content: center;
      max-width: 512px;

      &:hover, &.is-dragging {
        border-color: $light-orange;
      }

      .dropzone-desc {
        position: absolute;
        display: flex;
        width: 100%;
        height: 100%;
        justify-content: center;
        align-items: center;
        font-size: 18px;
      }

      .dropzone {
        position: absolute;
        outline: none;
        width: 100%;
        height: 100%;
        cursor: pointer;
        opacity: 0;
        box-sizing: border-box;
      }
    }

    .preview-zone-wrapper {
      position: relative;
      display: flex;
      flex-direction: row;
      align-items: center;
      gap: 2rem;

      img {
        border-radius: 50%;
        overflow: clip;
      }

      i {
        width: 36px;
        height: 36px;
      }
    }
  }
</style>