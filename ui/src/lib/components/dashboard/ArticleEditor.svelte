<script lang="ts">
    import Textarea from '$lib/components/common/Textarea.svelte'
    import Markdown from '@magidoc/plugin-svelte-marked'
    import { finalize, tap } from 'rxjs'
    import Input from '$lib/components/common/Input.svelte'
    import { ArticleServices } from '$lib/services/article.service'
    import Loading from '$lib/components/common/Loading.svelte'
    import { startWithTap } from '$lib/utils'
    import type { CategoryData } from '$lib/types'

    type EditorProps = {
        id?: string
        title?: string
        content?: string
        categoryId?: string
        categories: CategoryData[]
    }

    let { id, content, title, categoryId, categories }: EditorProps = $props()
    let newData: { title: string, content: string, category_id?: string } = $state({ title: '', content: '' })
    $effect(() => {
        // init the state by props
        newData = {
            title: title ?? '',
            content: content ?? '',
            category_id: categoryId
        }
    })
    let errorMsg = $state('')
    let isLoading = $state(false)

    // handles content has been changed
    // it will update the preview zone automatically
    function onContentChanged(
        e: Event & { currentTarget: EventTarget & HTMLTextAreaElement }
    ) {
        newData = {
            ...newData,
            content: (e.currentTarget as HTMLTextAreaElement).value.trim()
        }
    }

    // handles title has been changed
    function onTitleChanged(
        e: Event & { currentTarget: EventTarget & HTMLInputElement }
    ) {
        newData = {
            ...newData,
            title: (e.currentTarget as HTMLInputElement).value.trim()
        }
    }

    // handles category has been changed
    function onCategoryChanged(
        e: Event & { currentTarget: EventTarget & HTMLSelectElement }
    ) {
        newData = {
            ...newData,
            category_id: (e.currentTarget as HTMLSelectElement).value.trim()
        }
    }

    // checks if content is not empty
    // if it is, returns false. Otherwise, returns true
    function isValid() {
        return newData.title.trim() !== '' && newData.content !== ''
    }

    // handles the save button has been clicked
    function onSaveClicked() {
        errorMsg = ''

        if (!isValid()) {
            errorMsg = 'Content is required and cannot be empty'
            return
        }

        ArticleServices.save({
            ...(id === undefined ? {} : { id: id }),
            ...newData,
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
    class:show={errorMsg !== ''}
    role="alert"
>
    <p class="w-full text-center">{errorMsg}</p>
</div>
<div class="relative flex flex-col gap-y-4 px-4 py-4 md:flex-row md:gap-x-4">
    <div class="flex-1">
        <div class="relative flex-row gap-x-4">
            <Input
                hasError={errorMsg !== ''}
                label="Article Title"
                name="title"
                onInput={onTitleChanged}
                type="text"
                value={title ?? ''}
            />

            <div class="mb-4">
                <label class="block mb-2 text-sm font-bold text-gray-700" for="countries">Category</label>
                <select class="bg-gray-50 border border-gray-300 text-gray-700 text-sm rounded-lg block w-full p-2.5"
                        id="countries"
                        onchange={onCategoryChanged}
                >
                    <option selected>Choose a category</option>
                    {#each categories as category (category.id) }
                        <option value={category.id} selected={categoryId === category.id}>{category.data.name}</option>
                    {/each}
                </select>
            </div>

            <Textarea
                label="Article Content"
                name="service"
                onInput={onContentChanged}
                value={content ?? ''}
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
                <Markdown source={newData.content}></Markdown>
            </div>
        </div>
    </div>
</div>
<div class="relative flex flex-row justify-center gap-x-4">
    <button
        class="block cursor-pointer rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none disabled:cursor-auto disabled:bg-gray-500"
        disabled={newData.title === '' || newData.content === ''}
        onclick={onSaveClicked}
    >
        Save
    </button>
</div>
