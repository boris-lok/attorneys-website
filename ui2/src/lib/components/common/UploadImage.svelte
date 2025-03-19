<script lang="ts">
    import { createEventDispatcher, onMount } from 'svelte'
    import { BehaviorSubject } from 'rxjs'

    // The avatarUrl that displays the avatar
    export let avatarData: null = null
    // The flag is used to indicate that user is dragging
    let isDragging = false
    // The image is used to display
    let image: HTMLElement
    // // The event dispatcher to let parent component handle the event
    // const dispatch = createEventDispatcher()
    // The flag is used to indicate that user has selected an image
    let hasImage = false
    // The file selected by the user
    let file = new BehaviorSubject<File | undefined>(undefined)

    onMount(() => {
        // // listen to file has been changed
        // const disposer = file.subscribe({
        //     next: (f) => {
        //         hasImage = !!f
        //         dispatch('change', {
        //             file: f,
        //         })
        //     },
        // })
        //
        // // disposer is responsible for the component has been removed
        // return () => {
        //     disposer.unsubscribe()
        // }
    })

    // Handle file drop
    function handleDrop(e: DragEvent) {
        e.preventDefault()
        isDragging = false

        // Extract files from event
        const files = e.dataTransfer?.files ?? ([] as File[])

        handleInputFiles(files)
    }

    // Prevent default behavior for dragover and dragenter
    function handleDragOver(e: DragEvent) {
        e.preventDefault()
        isDragging = true
    }

    // Handle file drag leave event
    function handleDragLeave(e: DragEvent) {
        e.preventDefault()
        isDragging = false
    }

    function generatePreview(file: File) {
        const reader = new FileReader()
        reader.onload = function (event: ProgressEvent) {
            if (typeof reader.result === 'string') {
                image.setAttribute('src', reader.result)
            }
        }
        reader.readAsDataURL(file)
    }

    function onInputChanged(e: Event) {
        const files = (e.target as HTMLInputElement)?.files ?? ([] as File[])
        // avatarData = null
        handleInputFiles(files)
    }

    function handleInputFiles(files: File[] | FileList) {
        let newFile = undefined
        if (files.length > 0) {
            // newFile = iterate(files).find((e) => e.type.startsWith('image/'))
            // if (newFile) {
            //     generatePreview(newFile)
            // }
        }
        file.next(newFile)
    }

    // delete the image that user has selected
    function onDeleteClicked() {
        file.next(undefined)
    }

    // delete the avatar data
    function onAvatarDeleteButtonClicked() {
        // avatarData = null
    }
</script>

<div class="relative flex w-full items-center justify-center overflow-clip">
    {#if !hasImage && avatarData === null}
        <div
            class="relative box-border flex h-36 w-full max-w-2xl items-center justify-center border-2 border-dashed border-gray-300 text-black hover:border-amber-500 [&.is-dragging]:border-amber-500"
            on:drop={handleDrop}
            on:dragover={handleDragOver}
            on:dragleave={handleDragLeave}
            class:is-dragging={isDragging}
        >
            <div
                class="absolute flex h-full w-full items-center justify-center text-xl"
            >
                <p>Please click or drag an image here.</p>
            </div>
            <input
                type="file"
                name="image"
                class="absolute box-border h-full w-full cursor-pointer opacity-0 outline-none"
                on:change={onInputChanged}
            />
        </div>
    {/if}
    {#if avatarData}
        <div class="flex flex-row items-center gap-8">
            <Avatar avatar={avatarData} />
            <!--            <i class="material-icon" on:click={onAvatarDeleteButtonClicked}>delete</i>-->
        </div>
    {/if}
    {#if hasImage}
        <div class="relative flex flex-row items-center gap-8">
            <img
                src=""
                bind:this={image}
                alt="avatarPreview"
                width="256"
                height="256"
                class="overflow-clip rounded-[50%]"
            />
            <!--            <i class="material-icon" on:click={onDeleteClicked}>delete</i>-->
        </div>
    {/if}
</div>
