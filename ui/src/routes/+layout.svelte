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
    min-height: 67vh;
    margin: 15rem 5% 2.5rem 5%;
  }

  @media (min-width: 768px) {
    main {
      margin: 20rem 10% 5rem 10%;
    }
  }
</style>
