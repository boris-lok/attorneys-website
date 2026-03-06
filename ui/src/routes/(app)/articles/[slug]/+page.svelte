<script lang="ts">
    import type { PageProps } from './$types'
    import Article from '$lib/components/Article.svelte'
    import { browser } from '$app/environment'

    let { data }: PageProps = $props()

    function onBackClicked() {
        if (browser) {
            window.history.back()
        }
    }
</script>

<Article id={data.id} />

<div class="relative my-8 flex flex-row items-center justify-center">
    <button
        class="h-10 w-36 cursor-pointer rounded border transition-[background-color,font-size] duration-500 hover:border-[var(--primary-color)] hover:bg-[var(--primary-color)] hover:text-lg hover:text-white"
        onclick={onBackClicked}
        >返回
    </button>
</div>
