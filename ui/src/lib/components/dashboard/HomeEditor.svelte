<script lang="ts">
    import Textarea from '$lib/components/common/Textarea.svelte'
    import Markdown from '@magidoc/plugin-svelte-marked'
    import { HomeServices } from '$lib/services/home.service'
    import { finalize, tap } from 'rxjs'
    import { startWithTap } from '$lib/utils'
    import Loading from '$lib/components/common/Loading.svelte'

    type EditorProps = {
        id?: string
        data?: string
    }

    let { id, data }: EditorProps = $props()

    let content = $state(data ?? '')
    let errorMsg = $state('')
    let isLoading = $state(false)

    // handles content has been changed
    // it will update the preview zone automatically
    function onContentChanged(
        e: Event & { currentTarget: EventTarget & HTMLTextAreaElement }
    ) {
        content = (e.currentTarget as HTMLTextAreaElement).value.trim()
    }

    // checks if content is not empty
    // if it is, returns false. Otherwise, returns true
    function isValid() {
        return content.trim() !== ''
    }

    // handles the save button has been clicked
    function onSaveClicked() {
        errorMsg = ''

        if (!isValid()) {
            errorMsg = 'Content is required and cannot be empty'
            return
        }

        HomeServices.save({
            ...(id === undefined ? {} : { id: id }),
            data: content,
            language: 'zh',
            seq: 0
        })
            .pipe(
                startWithTap(() => isLoading = true),
                finalize(() => isLoading = false),
                tap((resp) => {
                    if (resp.error) {
                        console.error('Error saving content:', resp.message)
                        errorMsg = 'We got an error when saving content.'
                    }
                })
            )
            .subscribe()
    }
</script>

<Loading show={isLoading} />
<div
    class="mb-2 flex rounded-lg bg-red-50 p-4 text-sm text-red-800 hidden [.show]:block"
    role="alert"
>
    <p class="w-full text-center">{errorMsg}</p>
</div>
<div class="relative flex flex-col gap-y-4 px-4 py-4 md:flex-row md:gap-x-4">
    <div class="flex-1">
        <Textarea
            label="Home Content"
            name="home"
            onInput={onContentChanged}
            value={data ?? ''}
        ></Textarea>
    </div>
    <div class="flex-1">
        <p class="mb-2 block text-sm font-medium text-gray-900">Preview</p>
        <div
            class="prose block min-h-96 w-full rounded-lg bg-gray-100 px-4 py-4"
        >
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
