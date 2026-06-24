<script lang="ts">
    import { page } from '$app/state'
    import Error403 from '$lib/components/errors/Error403.svelte'
</script>

{#if page.status === 403}
    <Error403 message={page.error?.message} />
{:else}
    <h2>{page.status}</h2>
    <p>{page.error?.message ?? ''}</p>
{/if}
