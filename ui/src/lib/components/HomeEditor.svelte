<script lang="ts">
    import Textarea from '$lib/components/shared/Textarea.svelte'
    import Markdown from '@magidoc/plugin-svelte-marked'
    import { HomeServices } from '$lib/services/home.service'
    import Loading from '$lib/components/shared/Loading.svelte'

    type EditorProps = {
        id?: string
        data?: string
    }

    let { id, data = '' }: EditorProps = $props()
    let content: string = $state(data)
    let errorMsg = $state('')
    let isLoading = $state(false)

    /**
     * Handles the content change event triggered by a textarea element.
     *
     * @param {Event & { currentTarget: EventTarget & HTMLTextAreaElement }} e - The event triggered when the content of the textarea changes. It includes the current target which is the HTMLTextAreaElement instance.
     * @return {void} This function does not return a value. It updates the content variable with the trimmed value from the textarea.
     */
    function onContentChanged(
        e: Event & { currentTarget: EventTarget & HTMLTextAreaElement }
    ): void {
        content = (e.currentTarget as HTMLTextAreaElement).value.trim()
    }

    /**
     * Checks if the content is valid by verifying that it is not an empty string after trimming.
     *
     * @return {boolean} Returns true if the trimmed content is not an empty string, otherwise false.
     */
    function isValid(): boolean {
        return content.trim() !== ''
    }

    /**
     * Handles the save button click event. Validates the input content
     * and sends it to the save service. Displays an error message if
     * the validation fails or the save process encounters an error.
     *
     * @return {Promise<void>} A promise that resolves after the save process is completed or an error occurs.
     */
    async function onSaveClicked(): Promise<void> {
        errorMsg = ''

        if (!isValid()) {
            errorMsg = 'Content is required and cannot be empty'
            return
        }

        isLoading = true
        const resp = await HomeServices.save({
            ...(id ? { id } : {}),
            data: content,
            language: 'zh',
            seq: 0,
        })
        isLoading = false

        if (resp.error) {
            console.error('Error saving content:', resp.message)
            errorMsg = 'We got an error when saving content.'
            return
        }
    }
</script>

{#if isLoading}
    <Loading />
{:else}
    <div class="mb-2 flex hidden rounded-lg bg-red-50 p-4 [.show]:block" role="alert">
        <p class="w-full text-center text-sm text-red-800">{errorMsg}</p>
    </div>
    <div class="relative flex flex-col gap-y-4 px-4 py-4 md:flex-row md:gap-x-4">
        <div class="flex-1">
            <Textarea label="Home Content" name="home" onInput={onContentChanged} value={data ?? ''}
            ></Textarea>
        </div>
        <div class="flex-1">
            <p class="mb-2 block text-sm font-medium text-gray-900">Preview</p>
            <div class="prose block min-h-96 w-full rounded-lg bg-gray-100 px-4 py-4">
                <Markdown source={content}></Markdown>
            </div>
        </div>
    </div>
    <div class="relative flex flex-row justify-center gap-x-4">
        <button
            class="block cursor-pointer rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none disabled:cursor-auto disabled:bg-gray-500"
            disabled={content === ''}
            onclick={onSaveClicked}
        >
            Save
        </button>
    </div>
{/if}
