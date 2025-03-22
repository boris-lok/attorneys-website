<script lang="ts">
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import { ArticleServices } from '$lib/services/article.service'
    import ArticleEditor from '$lib/components/dashboard/ArticleEditor.svelte'
    import type { PageProps } from './$types'
    import Markdown from '@magidoc/plugin-svelte-marked'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let title = $state('')
    let content = $state('')

    function fetchData() {
        ArticleServices.retrieve(data.id, 'zh')
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp) => {
                    content = resp?.data.content ?? ''
                    title = resp?.data.title ?? ''
                })
            )
            .subscribe({
                error: console.error
            })
    }

    $effect(() => fetchData())
</script>

{#if isLoading}
    <p>Loading...</p>
{:else}
    <div class="relative">
        <p class="mb-8 text-3xl font-bold text-[var(--primary-color)]">{title}</p>
        <div class="prose">
            <Markdown source={content}></Markdown>
        </div>
    </div>
{/if}
