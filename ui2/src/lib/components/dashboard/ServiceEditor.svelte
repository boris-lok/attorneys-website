<script lang="ts">
    import Textarea from '$lib/components/common/Textarea.svelte'
    import Markdown from '@magidoc/plugin-svelte-marked'
    import { tap } from 'rxjs'
    import Input from '$lib/components/common/Input.svelte'
    import { ServiceServices } from '$lib/services/service.service'

    type EditorProps = {
        id?: string
        title?: string
        data?: string
    }

    let { id, data, title }: EditorProps = $props()

    let newData = $state({
        title: title ?? '',
        data: data ?? '',
    })
    let errorMsg = $state('')

    // handles content has been changed
    // it will update the preview zone automatically
    function onContentChanged(
        e: Event & { currentTarget: EventTarget & HTMLTextAreaElement },
    ) {
        newData = {
            ...newData,
            data: (e.currentTarget as HTMLTextAreaElement).value.trim(),
        }
    }

    // handles title has been changed
    function onTitleChanged(
        e: Event & { currentTarget: EventTarget & HTMLInputElement },
    ) {
        newData = {
            ...newData,
            title: (e.currentTarget as HTMLInputElement).value.trim(),
        }
    }

    // checks if content is not empty
    // if it is, returns false. Otherwise, returns true
    function isValid() {
        return newData.title.trim() !== '' && newData.data !== ''
    }

    // handles the save button has been clicked
    function onSaveClicked() {
        errorMsg = ''

        if (!isValid()) {
            errorMsg = 'Content is required and cannot be empty'
            return
        }

        ServiceServices.save({
            ...(id === undefined ? {} : { id: id }),
            ...newData,
            language: 'zh',
            seq: 0,
        })
            .pipe(
                tap((resp) => {
                    if (resp.error) {
                        console.error('Error saving content:', resp.message)
                        errorMsg = 'We got an error when saving content.'
                    }
                }),
            )
            .subscribe()
    }
</script>

{#if errorMsg !== ''}
    <div
        class="mb-2 flex rounded-lg bg-red-50 p-4 text-sm text-red-800"
        role="alert"
    >
        <p class="w-full text-center">{errorMsg}</p>
    </div>
{/if}
<div class="relative flex flex-col gap-y-4 px-4 py-4 md:flex-row md:gap-x-4">
    <div class="flex-1">
        <div class="relative flex-row gap-x-4">
            <Input
                hasError={errorMsg !== ''}
                label="Service Title"
                name="title"
                onInput={onTitleChanged}
                type="text"
                value={title ?? ''}
            />
            <Textarea
                label="Service Content"
                name="service"
                onInput={onContentChanged}
                value={data ?? ''}
            ></Textarea>
        </div>
    </div>
    <div class="flex-1">
        <p class="mb-2 block text-sm font-medium text-gray-900">Preview</p>
        <div
            class="prose block min-h-96 w-full rounded-lg bg-gray-100 px-4 py-4"
        >
            <p class="py-2 text-lg font-bold text-[var(--primary-color)]">
                {newData.title}
            </p>
            <div class="prose">
                <Markdown source={newData.data}></Markdown>
            </div>
        </div>
    </div>
</div>
<div class="relative flex flex-row justify-center gap-x-4">
    <button
        class="block cursor-pointer rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none disabled:cursor-auto disabled:bg-gray-500"
        disabled={newData.title === '' || newData.data === ''}
        onclick={onSaveClicked}
    >
        Save
    </button>
</div>
