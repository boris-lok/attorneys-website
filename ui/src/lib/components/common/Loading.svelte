<script lang="ts">
    import IconifyIcon from '@iconify/svelte'

    type InputProps = {
        show: boolean
    }

    let { show }: InputProps = $props()


    $effect(() => {
        document.body.style.overflow = show ? 'hidden' : ''
    })
</script>

<div class="w-full h-screen hidden top-0 left-0 bg-gray-100/50 [.show]:flex fixed items-center justify-center z-50"
     class:show>
    <IconifyIcon class="w-12 h-12" icon="eos-icons:bubble-loading" />
</div>