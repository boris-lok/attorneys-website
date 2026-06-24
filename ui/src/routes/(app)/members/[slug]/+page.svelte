<script lang="ts">
    import Markdown from '@magidoc/plugin-svelte-marked'
    import Image from '$lib/components/shared/Image.svelte'
    import IconifyIcon from '@iconify/svelte'
    import { browser } from '$app/environment'
    import type { MemberData } from '$lib/types'

    let { data } = $props()

    const member: MemberData = $derived(data.data)

    function onBackClicked() {
        if (browser) {
            window.history.back()
        }
    }

</script>

<div class="my-8 px-4 md:mx-auto md:my-16 md:max-w-5xl md:px-8">
    <div class="relative flex flex-col items-center justify-between md:flex-row">
        <p class="mb-8 text-3xl font-bold text-(--primary-color)">
            {member.data.name}
        </p>
        {#if member.avatar}
            <div class="h-48 w-48">
                <Image alt={member.data.name} image={member.avatar} />
            </div>
        {:else}
            <IconifyIcon icon="radix-icons:avatar" class="h-48 w-48" />
        {/if}
    </div>
    <div class="prose mt-4 max-w-2xl md:max-w-3xl lg:max-w-4xl">
        <Markdown source={member.data.description ?? ''}></Markdown>
    </div>

    <div class="relative flex flex-row items-center justify-center">
        <button
            class="h-10 w-36 cursor-pointer rounded border transition-[background-color,font-size] duration-500 hover:border-[var(--primary-color)] hover:bg-[var(--primary-color)] hover:text-lg hover:text-white"
            onclick={onBackClicked}
        >返回
        </button>
    </div>
</div>
