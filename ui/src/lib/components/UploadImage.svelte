<script lang="ts">
    import {t} from 'svelte-i18n';
    import iterate from 'iterare';
    import {createEventDispatcher} from "svelte";

    let isDragging = false;
    let image: HTMLElement;
    const dispatch = createEventDispatcher();

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
        reader.onload = function (event: ProgressEvent) {
            if (typeof reader.result === "string") {
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
        if (files.length > 0) {
            const file = iterate(files)
                .find(e => e.type.startsWith("image/"));
            if (file) {
                generatePreview(file);
                dispatch("change", {
                    file: file
                });
            }
        }
    }
</script>

<div class="dropzone-wrapper" on:drop={handleDrop} on:dragover={handleDragOver} on:dragleave={handleDragLeave}
     class:is-dragging={isDragging}>
    <div class="dropzone-desc">
        <p>{$t('upload_image_hint')}</p>
    </div>
    <input type="file" name="image" class="dropzone" on:change={onInputChanged}/>
</div>

<div>
    <img src="" bind:this={image} alt="Preview" width="128" height="auto">
</div>

<style lang="scss">
  .dropzone-wrapper {
    border: 2px dashed $grey;
    color: $black;
    position: relative;
    height: 144px;

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
</style>