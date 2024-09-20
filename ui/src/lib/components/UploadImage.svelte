<script lang="ts">
	import { t } from 'svelte-i18n';
	import iterate from 'iterare';
	import { createEventDispatcher, onMount } from 'svelte';
	import { BehaviorSubject } from 'rxjs';

	let isDragging = false;
	let image: HTMLElement;
	const dispatch = createEventDispatcher();
	let hasImage = false;
	let file = new BehaviorSubject<File | undefined>(undefined);

	onMount(() => {
		const disposer = file.subscribe({
			next: (f) => {
				hasImage = !!f;
				console.log(`change`);
				dispatch('change', {
					file: f
				});
			}
		});

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

	function onDeleteClicked() {
		file.next(undefined);
	}
</script>

<div class="image-wrapper">
	{#if !hasImage}
		<div class="dropzone-wrapper" on:drop={handleDrop} on:dragover={handleDragOver} on:dragleave={handleDragLeave}
				 class:is-dragging={isDragging}>
			<div class="dropzone-desc">
				<p>{$t('upload_image_hint')}</p>
			</div>
			<input type="file" name="image" class="dropzone" on:change={onInputChanged} />

		</div>
	{/if}
	{#if hasImage}
		<div class="preview-zone-wrapper">
			<img src="" bind:this={image} alt="Preview" width="128" height="128">
			<i class="material-icon" on:click={onDeleteClicked}>delete</i>
		</div>
	{/if}
</div>


<style lang="scss">
  .image-wrapper {
    width: 100%;
    height: 144px;
    overflow: clip;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;

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
      height: 128px;
      position: relative;

      i {
        position: absolute;
        top: calc(50% - 24px + 3px);
        right: -64px;
        padding: 6px;
        border-radius: 50%;
        cursor: pointer;
        color: $deep-red;
        box-shadow: 0 0 4px 1px $grey;
      }
    }
  }
</style>