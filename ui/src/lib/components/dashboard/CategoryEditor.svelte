<script lang="ts">
    import { finalize, tap } from 'rxjs'
    import { CategoryService } from '$lib/services/category.service'
    import Input from '$lib/components/common/Input.svelte'
    import IconifyIcon from '@iconify/svelte'
    import { startWithTap } from '$lib/utils'
    import Loading from '$lib/components/common/Loading.svelte'

    type EditorProps = {
        id?: string
        icon?: string
        name?: string

    }

    let { id, icon, name }: EditorProps = $props()

    let data = $state({
        icon: icon ?? '',
        name: name ?? ''
    })
    let errorMsg = $state('')
    let isLoading = $state(false)

    // handles name has been changed
    function onNameChanged(
        e: Event & { currentTarget: EventTarget & HTMLInputElement }
    ) {
        data = {
            ...data,
            name: (e.currentTarget as HTMLInputElement).value.trim()
        }
    }

    // handles icon has been changed
    function onIconChanged(
        e: Event & { currentTarget: EventTarget & HTMLInputElement }
    ) {
        data = {
            ...data,
            icon: (e.currentTarget as HTMLInputElement).value.trim()
        }
    }

    // checks if content is not empty
    // if it is, returns false. Otherwise, returns true
    function isValid() {
        return data.name.trim() !== ''
    }

    // handles the save button has been clicked
    function onSaveClicked() {
        errorMsg = ''

        if (!isValid()) {
            errorMsg = 'Content is required and cannot be empty'
            return
        }

        CategoryService.save({
            ...(id === undefined ? {} : { id: id }),
            language: 'zh',
            ...data,
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
    class:show={errorMsg !== ''}
>
    <p class="w-full text-center">{errorMsg}</p>
</div>
<div class="relative flex flex-col gap-y-4 px-4 py-4 md:flex-row md:gap-x-4">
    <div class="flex-1">
        <div class="flex flex-col gap-x-1">
            <Input
                hasError={errorMsg !== ''}
                label="Category Icon"
                name="icon"
                onInput={onIconChanged}
                type="text"
                value={icon ?? ''}
            />
            <p class="mt-[-8px] text-gray-500 text-xs mb-2 px-2">Please find the icon from <a
                class="text-blue-600 underline hover:text-blue-800 visited:text-pink-600"
                href="https://icon-sets.iconify.design/"
                target="_blank"
            >here</a></p>
        </div>
        <Input
            hasError={errorMsg !== ''}
            label="Category name"
            name="name"
            onInput={onNameChanged}
            type="text"
            value={name ?? ''}
        />
    </div>
    <div class="flex-1">
        <p class="mb-2 block text-sm font-medium text-gray-900">Preview</p>
        <div
            class="flex flex-row gap-2 w-full rounded-lg bg-gray-100 px-4 py-4 items-center"
        >
            {#if data.icon}
                <IconifyIcon icon={data.icon} class="h-8 w-8" />
            {/if}
            <p class="inline-block h-fit">{data.name}</p>
        </div>
    </div>
</div>
<div class="relative flex flex-row justify-center gap-x-4">
    <button
        class="block cursor-pointer rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none disabled:cursor-auto disabled:bg-gray-500"
        disabled={data.name.trim() === ''}
        onclick={onSaveClicked}
    >
        Save
    </button>
</div>
