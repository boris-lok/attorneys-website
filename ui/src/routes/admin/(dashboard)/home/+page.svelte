<script lang="ts">
    import Markdown from '@magidoc/plugin-svelte-marked'
    import { HomeServices } from '$lib/services/home.service'
    import Icon from '@iconify/svelte'
    import Loading from '$lib/components/common/Loading.svelte'

    let id = $state('')
    // The content of home page
    let content = $state('')
    // The loading statue of retrieving the content from API.
    let isLoading = $state(false)

    // fetch the content from API.
    async function fetchData() {
        const resp = await HomeServices.list('zh')
        if (resp.error) {
            console.error(resp.message)
            return
        }

        content = (resp.home ?? []).length === 0 ? '' : resp.home![0].data.data
    }

    $effect(() => {
        (async () => {
            isLoading = true
            await fetchData()
            isLoading = false
        })()
    })
</script>

{#if isLoading}
    <Loading />
{:else}
    <div
        class="relative mx-auto flex max-w-[var(--max-screen-width)] flex-col gap-y-8 px-16 py-16"
    >
        <div class="relative flex flex-row justify-end">
            {#if id !== ''}
                <a href="/admin/home/edit/{id}">
                    <Icon icon="mingcute:edit-line" width="24" height="24" />
                </a>
            {:else}
                <a href="/admin/home/edit">
                    <Icon icon="gridicons:create" width="24" height="24" />
                </a>
            {/if}
        </div>
        <div
            class="prose w-full px-6 pt-16 md:min-w-2xl lg:min-w-3xl md:px-16 md:pt-0 mx-auto md:mt-16"
        >
            <Markdown source={content}></Markdown>
        </div>
    </div>
{/if}