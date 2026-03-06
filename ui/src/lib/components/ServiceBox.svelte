<script lang="ts">
    import IconifyIcon from '@iconify/svelte'
    import Markdown from '@magidoc/plugin-svelte-marked'

    type InputProps = {
        title: string
        icon?: string
        content: string
        active: boolean
    }

    let { title, icon, content, active }: InputProps = $props()
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="group relative h-72 w-72 overflow-clip rounded-xl" class:active>
    <div
        class="relative h-full w-full rounded-xl border border-[var(--primary-color)] p-4 transition-[width,height]"
    >
        <div
            class="flex h-full w-full flex-col items-center justify-center gap-2 text-2xl font-bold text-[var(--primary-color)] group-hover:opacity-20 max-sm:group-[.active]:opacity-20"
        >
            {#if icon}
                <IconifyIcon {icon} class="h-12 w-12" />
            {/if}
            <p>{title}</p>
        </div>
    </div>

    <div
        class="absolute inset-0 bg-white opacity-0 group-hover:block group-hover:opacity-80 max-sm:group-[.active]:opacity-80 max-sm:[&.active]:block"
    ></div>
    <div
        class="prose absolute top-0 hidden h-72 w-full overflow-y-auto p-4 text-left group-hover:block max-sm:group-[.active]:block"
    >
        <Markdown source={content} />
    </div>
</div>
