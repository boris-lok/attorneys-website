<script lang="ts">
    import IconifyIcon from '@iconify/svelte'
</script>

<svelte:window on:wheel|nonpassive|preventDefault />
<div
    class="fixed top-0 left-0 z-50 flex h-screen w-full items-center justify-center bg-gray-100/50"
>
    <IconifyIcon class="h-12 w-12" icon="eos-icons:bubble-loading" />
</div>
