<script lang="ts">
    import { ArticleServices } from '$lib/services/article.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import Markdown from '@magidoc/plugin-svelte-marked'
    import IconifyIcon from '@iconify/svelte'

    type InputProps = {
        id: string;
        onBackClicked?: () => void;
    }

    // The id of article
    let { id, onBackClicked }: InputProps = $props()
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
        <div class="relative flex flex-row justify-between">

            <p class=" text-3xl font-bold text-[var(--primary-color)] text-center my-8">
                {data.title}
            </p>
            {#if onBackClicked}
                <button class="cursor-pointer" onclick={onBackClicked}>
                    <IconifyIcon icon="line-md:close-circle"
                                 class="w-6 h-6 m-2" />
                </button>
            {/if}
        </div>
        <div class="prose min-w-full">
            <Markdown source={data.content}></Markdown>
        </div>
    </div>
{/if}