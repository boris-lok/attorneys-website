<script lang="ts">
    import Icon from '@iconify/svelte'
    import HomeContent from '$lib/components/HomeContent.svelte'
    import type { Language } from '$lib/types'

    let language: Language = 'zh'
    let id: undefined | string = $state(undefined)
</script>

<div class="relative mx-auto flex max-w-[var(--max-screen-width)] flex-col gap-y-8 px-16 py-16">
    <div class="relative flex flex-row justify-end">
        {#if id && id !== ''}
            <a href="/admin/home/edit/{id}">
                <Icon icon="mingcute:edit-line" width="24" height="24" />
            </a>
        {:else}
            <a href="/admin/home/edit">
                <Icon icon="gridicons:create" width="24" height="24" />
            </a>
        {/if}
    </div>

    <div>
        <HomeContent {language} onReady={(e) => (id = e)} />
    </div>
</div>
