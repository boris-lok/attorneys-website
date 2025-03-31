<script lang="ts">
    import { BehaviorSubject } from 'rxjs'
    import type { ImageData } from '$lib/types'
    import { iterate } from 'iterare'
    import Image from '$lib/components/common/Image.svelte'
    import IconifyIcon from '@iconify/svelte'

    type InputProps = {
        imageData?: ImageData | null
        onChange?: (file: File | undefined) => void
    }

    // The avatarUrl that displays the avatar
    let { imageData, onChange }: InputProps = $props()
    // The snapshot of image data
    let data = $state($state.snapshot(imageData))
    // The flag is used to indicate that user is dragging
    let isDragging = $state(false)
    // The image is used to display
    let imageSrc = $state('')
    // // The event dispatcher to let parent component handle the event
    // const dispatch = createEventDispatcher()
    // The flag is used to indicate that user has selected an image
    let hasImage = $state(false)
    // The file selected by the user
    let file = new BehaviorSubject<File | undefined>(undefined)

    $effect(() => {
        const disposer = file.subscribe({
            next: (f) => {
                console.log('file has been changed')
                console.log(f)
                hasImage = !!f
                onChange?.(f)
            }
        })

        return () => {
            disposer.unsubscribe()
        }
    })

    // Handle file drop
    function handleDrop(e: DragEvent) {
        e.preventDefault()
        isDragging = false

        // Extract files from event
        const files = e.dataTransfer?.files ?? ([] as File[])

        const newFile = handleInputFiles(files)
        if (newFile) {
            generatePreview(newFile)
        }
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
        reader.onload = function() {
            if (typeof reader.result === 'string') {
                imageSrc = reader.result
            }
        }
        reader.readAsDataURL(file)
    }

    function onInputChanged(e: Event) {
        const files = (e.target as HTMLInputElement)?.files ?? ([] as File[])
        const newFile = handleInputFiles(files)
        if (newFile) {
            generatePreview(newFile)
        }
    }

    function handleInputFiles(files: File[] | FileList) {
        let newFile = undefined
        if (files.length > 0) {
            newFile = iterate(files).find((e) => e.type.startsWith('image/'))
        }
        file.next(newFile)
        return newFile
    }

    // delete the image that user has selected
    function onDeleteClicked() {
        file.next(undefined)
    }
</script>

<div class="relative flex w-full items-center justify-center overflow-clip">
    {#if !hasImage && !data}
        <!-- svelte-ignore a11y_no_static_element_interactions because we use div as a drop zone-->
        <div
            class="relative box-border flex h-36 w-full max-w-2xl items-center justify-center border-2 border-dashed border-gray-300 text-black hover:border-amber-500 [&.is-dragging]:border-amber-500"
            ondrop={handleDrop}
            ondragover={handleDragOver}
            ondragleave={handleDragLeave}
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
                onchange={onInputChanged}
            />
        </div>
    {/if}
    {#if data}
        <div class="flex flex-row items-center gap-8">
            <Image image={data} alt="preview" />
            <button onclick={onDeleteClicked} class="cursor-pointer">
                <IconifyIcon
                    icon="material-symbols-light:delete-outline"
                    class="h-6 w-6 text-red-800 hover:text-red-500"
                />
            </button>
        </div>
    {/if}
    {#if hasImage}
        <div class="relative flex flex-row items-center gap-4">
            <img
                src={imageSrc}
                alt="avatarPreview"
                class="h-36 w-36 overflow-clip rounded-[50%]"
            />
            <button onclick={onDeleteClicked} class="cursor-pointer">
                <IconifyIcon
                    icon="material-symbols-light:delete-outline"
                    class="h-6 w-6 text-red-800 hover:text-red-500"
                />
            </button>
        </div>
    {/if}
</div>
