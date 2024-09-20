<script lang="ts">
	import { onMount } from 'svelte';
	import { Home } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import SvelteMarkdown from 'svelte-markdown';
	import Loading from '$lib/components/Loading.svelte';

	let isLoading = false;
	let content = '';

	onMount(async () => {
		Home.list()
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => {
					content = e;
				})
			)
			.subscribe();
	});
</script>

{#if isLoading}
	<Loading />
{:else}
	<section class="home-wrapper">
		<SvelteMarkdown source={content} />
	</section>
{/if}

<style lang="scss">
  .home-wrapper {
    display: flex;
    flex-direction: column;
  }

  @media (min-width: 768px) {
    .home-wrapper {
      padding: 3rem;
      border-radius: 1rem;
      background-color: white;
    }
  }
</style>

