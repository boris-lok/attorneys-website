<script lang="ts">
    import { CategoryService } from '$lib/services/category.service'
    import Input from '$lib/components/shared/Input.svelte'
    import IconifyIcon from '@iconify/svelte'
    import Loading from '$lib/components/shared/Loading.svelte'

    type EditorProps = {
        id?: string
        icon?: string
        name?: string
    }

    let { id, icon, name }: EditorProps = $props()
    let data: { icon: string; name: string } = $state({ icon: '', name: '' })
    $effect(() => {
        // init the state by props
        data = {
            icon: icon ?? '',
            name: name ?? '',
        }
    })

    let errorMsg = $state('')
    let isLoading = $state(false)

    // handles name has been changed
    function onNameChanged(e: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        data = {
            ...data,
            name: (e.currentTarget as HTMLInputElement).value.trim(),
        }
    }

    // handles icon has been changed
    function onIconChanged(e: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        data = {
            ...data,
            icon: (e.currentTarget as HTMLInputElement).value.trim(),
        }
    }

    // checks if content is not empty
    // if it is, returns false. Otherwise, returns true
    function isValid() {
        return data.name.trim() !== ''
    }

    // handles the save button has been clicked
    async function onSaveClicked() {
        errorMsg = ''

        if (!isValid()) {
            errorMsg = 'Content is required and cannot be empty'
            return
        }

        isLoading = true

        const resp = await CategoryService.save({
            ...(id === undefined ? {} : { id: id }),
            language: 'en',
            ...data,
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
    <div
        class="mb-2 flex hidden rounded-lg bg-red-50 p-4 text-sm text-red-800 [.show]:block"
        class:show={errorMsg !== ''}
        role="alert"
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
                <p class="mt-[-8px] mb-2 px-2 text-xs text-gray-500">
                    Please find the icon from <a
                        class="text-blue-600 underline visited:text-pink-600 hover:text-blue-800"
                        href="https://icon-sets.iconify.design/"
                        target="_blank">here</a
                    >
                </p>
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
            <div class="flex w-full flex-row items-center gap-2 rounded-lg bg-gray-100 px-4 py-4">
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
{/if}
