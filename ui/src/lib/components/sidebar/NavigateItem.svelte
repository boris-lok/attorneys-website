<script lang="ts">
    import IconifyIcon from '@iconify/svelte'

    let {
        icon,
        label,
        topBar
    }: {
        icon: string
        label: string
        topBar: boolean
    } = $props()
</script>

{#if topBar}
    <div
        class="group relative flex cursor-pointer flex-row items-center gap-[0.25rem]"
    >
        <IconifyIcon {icon} class="h-6 w-6" />
        <p
            class="hidden whitespace-nowrap text-base absolute lg:relative lg:block group-hover:block max-lg:group-hover:top-5">{label}</p>
    </div>
{:else}
    <div
        class="group relative flex h-32 w-32 cursor-pointer flex-col items-center justify-around gap-2 rounded border-2 border-black p-4 hover:border-4"
    >
        <IconifyIcon {icon} class="h-12 w-12" />
        <p class=" group-hover:font-bold">{label}</p>
    </div>
{/if}
