<script lang="ts">
    import '../app.css'
    import Toast from '$lib/components/shared/Toast.svelte'
    import { navigating } from '$app/state'
    import ConfirmModal from '$lib/components/shared/ConfirmModal.svelte'

    let { children } = $props()
</script>

<div class="relative">
    {#if navigating.to}
        <div class="fixed top-0 left-0 z-50 h-1 w-full overflow-hidden bg-transparent">
            <div class="h-full w-full animate-pulse bg-blue-500"></div>
        </div>
    {/if}

    {@render children()}
</div>

<Toast />

<ConfirmModal />
