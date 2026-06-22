<script lang="ts">
    import { page } from '$app/state'
    import Error403 from '$lib/components/errors/Error403.svelte'
</script>

{#if page.state === 403}
    <Error403 message={page.error?.message} />
{:else}
    <h1>{page.state}</h1>
    <p>{page.error?.message ?? ''}</p>
{/if}