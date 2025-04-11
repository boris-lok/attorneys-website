<script lang="ts">
    import Textarea from '$lib/components/common/Textarea.svelte'
    import { finalize, tap } from 'rxjs'
    import Input from '$lib/components/common/Input.svelte'
    import { ServiceServices } from '$lib/services/service.service'
    import ServiceBox from '$lib/components/ServiceBox.svelte'
    import Loading from '$lib/components/common/Loading.svelte'
    import { startWithTap } from '$lib/utils'

    type EditorProps = {
        id?: string
        title?: string
        data?: string
        icon?: string
    }

    let { id, data, title, icon }: EditorProps = $props()

    let newData: { icon: string, title: string, data: string } = $state({
        icon: '',
        title: '',
        data: ''
    })
    $effect(() => {
        // init the state by props
        newData = {
            icon: icon ?? '',
            title: title ?? '',
            data: data ?? ''
        }
    })
    let errorMsg = $state('')
    let show = $state(false)
    let isLoading = $state(false)

    // handles content has been changed
    // it will update the preview zone automatically
    function onContentChanged(
        e: Event & { currentTarget: EventTarget & HTMLTextAreaElement }
    ) {
        newData = {
            ...newData,
            data: (e.currentTarget as HTMLTextAreaElement).value.trim()
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

    // handles icon has been changed
    function onIconChanged(e: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        newData = {
            ...newData,
            icon: (e.currentTarget as HTMLInputElement).value.trim()
        }
    }

    // handles the preview box has been clicked
    function onServiceBoxClicked() {
        show = !show
    }

    // checks if content is not empty
    // if it is, returns false. Otherwise, returns true
    function isValid() {
        return newData.title.trim() !== '' && newData.data !== '' && newData.icon.trim() !== ''
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
        <div class="relative flex-row gap-x-4">
            <div class="flex flex-col gap-x-1">
                <Input
                    hasError={errorMsg !== ''}
                    label="Service Icon"
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
        <button onclick={onServiceBoxClicked}>
            <ServiceBox active={show} content={newData.data} icon={newData.icon} title={newData.title} />
        </button>
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
