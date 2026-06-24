<script lang="ts">
    import type { PageProps } from './$types'
    import { browser } from '$app/environment'
    import Markdown from '@magidoc/plugin-svelte-marked'

    let { data }: PageProps = $props()

    function onBackClicked() {
        if (browser) {
            window.history.back()
        }
    }
</script>

<div class="relative mx-auto mt-8 md:mt-16 md:max-w-5xl">
    <div class="relative flex flex-row justify-between">
        <p class=" my-8 text-center text-3xl font-bold text-(--primary-color)">
            {data.article.data.title}
        </p>
    </div>
    <div class="prose min-w-full">
        <Markdown source={data.article.data.content}></Markdown>
    </div>
</div>

<div class="relative my-8 flex flex-row items-center justify-center">
    <button
        class="h-10 w-36 cursor-pointer rounded border transition-[background-color,font-size] duration-500 hover:border-(--primary-color) hover:bg-(--primary-color) hover:text-lg hover:text-white"
        onclick={onBackClicked}
        >返回
    </button>
</div>
