<script lang="ts">
    import '../../styles/global.scss';
    import NavigateBar from "$lib/components/NavigateBar.svelte";
    import Footer from "$lib/components/Footer.svelte";
</script>

<NavigateBar/>
<main>
    <slot/>
</main>
<Footer/>