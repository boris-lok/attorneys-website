<script lang="ts">
    import type { Language } from '$lib/types'
    import { HomeServices } from '$lib/services/home.service'
    import Loading from '$lib/components/shared/Loading.svelte'
    import Markdown from '@magidoc/plugin-svelte-marked'

    type InputProps = {
        language: Language
        onReady?: (id?: string) => void
    }
    let { language, onReady }: InputProps = $props()

    let content = $state('')
    let isLoading = $state(false)
    let errorMessage = $state('')

    async function fetchContent() {
        const resp = await HomeServices.list(language)
        if (resp.error) {
            console.error(resp.message)
            errorMessage = resp.message || 'Failed to fetch content.'
            return { content: '', id: undefined }
        }

        errorMessage = ''

        return {
            id: resp.home?.[0]?.id,
            content: resp.home?.[0]?.data?.data || '',
        }
    }

    $effect(() => {
        ;(async () => {
            isLoading = true
            const c = await fetchContent()
            content = c.content
            isLoading = false

            onReady?.(c.id)
        })()
    })
</script>

{#if isLoading}
    <Loading />
{:else}
    {#if errorMessage}
        <div>{errorMessage}</div>
    {/if}
    <div class="prose mx-auto my-6 w-full px-6 md:my-16 md:min-w-2xl md:px-16 lg:min-w-3xl">
        <Markdown source={content}></Markdown>
    </div>
{/if}
