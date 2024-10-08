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
		Home.list('zh')
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => {
					content = e?.data.data ?? '';
				})
			)
			.subscribe();
	});
</script>

{#if isLoading}
	<Loading />
{:else}
	<div class="home-wrapper">
		<SvelteMarkdown source={content} />
	</div>
{/if}

<style lang="scss">
  .home-wrapper {
    display: flex;
    flex-direction: column;
    padding: 1rem 5%;
  }

  @media (min-width: 768px) {
    .home-wrapper {
      padding: 3rem;
      border-radius: 1rem;
      background-color: white;
      max-width: 768px;
      margin: 0 auto;
    }
  }
</style>

