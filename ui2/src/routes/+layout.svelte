<script lang="ts">
    import '../app.css'
    import NavigateBar from '$lib/components/sidebar/NavigateBar.svelte'

    let { children } = $props()
</script>

<NavigateBar></NavigateBar>

<main>
    {@render children()}
</main>
