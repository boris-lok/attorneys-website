<script>
    import '../styles/global.scss';
    import NavigateBar from "$lib/components/NavigateBar.svelte";
    import Footer from "$lib/components/Footer.svelte";
</script>

<NavigateBar/>
<main>
    <slot/>
</main>
<Footer/>

<style lang="scss">
  main {
    min-height: 67vh
  }
</style>
