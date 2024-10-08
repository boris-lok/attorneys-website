<script>
	import NavigateBar from '$lib/components/NavigateBar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import '../../styles/global.scss';
</script>

<NavigateBar />
<main>
	<slot />
</main>
<Footer />

<style lang="scss">
  main {
    min-height: 67vh;
    margin: 15rem 0 2.5rem 0;
  }

  @media (min-width: 768px) {
    main {
      margin: 20rem 0 5rem 0;
    }
  }
</style>
