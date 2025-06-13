<script lang="ts">
    import { ArticleServices } from '$lib/services/article.service'
    import Markdown from '@magidoc/plugin-svelte-marked'
    import IconifyIcon from '@iconify/svelte'
    import Loading from '$lib/components/common/Loading.svelte'

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

    async function fetchData() {
        const resp = await ArticleServices.retrieve(id, 'zh')

        if (resp.error) {
            console.error(resp.message)
            return
        }

        return {
            title: resp.article?.data.title ?? '',
            content: resp.article?.data.content ?? ''
        }
    }

    $effect(() => {
        (async () => {
            isLoading = true

            try {
                const resp = await fetchData()
                if (resp) {
                    data = resp
                }
            } finally {
                isLoading = false
            }
        })()
    })
</script>

{#if isLoading}
    <Loading />
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
