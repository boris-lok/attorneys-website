<script lang="ts">
    import IconifyIcon from '@iconify/svelte'

    type InputProps = {
        show: boolean
    }

    let { show }: InputProps = $props()
</script>

<svelte:window on:wheel|nonpassive|preventDefault />
<div class="w-full h-screen hidden top-0 left-0 bg-gray-100/50 [.show]:flex fixed items-center justify-center" class:show>
    <IconifyIcon icon="eos-icons:bubble-loading" class="w-12 h-12" />
</div>