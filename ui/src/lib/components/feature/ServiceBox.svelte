<script lang="ts">
    import IconifyIcon from '@iconify/svelte'
    import Markdown from '@magidoc/plugin-svelte-marked'

    type InputProps = {
        title: string
        icon?: string
        content: string
    }

    let { title, icon, content }: InputProps = $props()
</script>

<div class="group relative h-72 w-72 overflow-clip rounded-xl">
    <div class="relative h-full w-full rounded-xl border border-(--primary-color) p-4">
        <div
            class="flex h-full w-full flex-col items-center justify-center gap-2 text-2xl font-bold text-(--primary-color) group-hover:opacity-20"
        >
            {#if icon}
                <IconifyIcon {icon} class="h-12 w-12" />
            {/if}
            <p>{title}</p>
        </div>
    </div>

    <div
        class="absolute inset-0 bg-white opacity-0 transition-opacity duration-200 group-hover:opacity-80"
    ></div>
    <div
        class="prose pointer-events-none absolute inset-0 overflow-y-auto p-4 text-left opacity-0 transition-opacity duration-200 group-hover:pointer-events-auto group-hover:opacity-100"
    >
        <Markdown source={content} />
    </div>
</div>
