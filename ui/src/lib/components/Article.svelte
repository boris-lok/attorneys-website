<script lang="ts">
    import { ArticleServices } from '$lib/services/article.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import Markdown from '@magidoc/plugin-svelte-marked'

    type InputProps = {
        id: string;
    }

    // The id of article
    let { id }: InputProps = $props()
    let isLoading = $state(false)

    let data = $state({
        title: '',
        content: ''
    })

    function fetchData() {
        ArticleServices.retrieve(id, 'zh')
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp) => {
                    data = {
                        title: resp?.data.title ?? '',
                        content: resp?.data.content ?? ''
                    }
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
    <div class="relative mt-8 md:mt-16 md:max-w-5xl mx-auto">
        <p class=" text-3xl font-bold text-[var(--primary-color)] text-center my-8">
            {data.title}
        </p>
        <div class="prose min-w-full">
            <Markdown source={data.content}></Markdown>
        </div>
    </div>
{/if}