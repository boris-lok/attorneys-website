<script lang="ts">
    import type { PageProps } from './$types';
    import Article from '$lib/components/Article.svelte';
    import { browser } from '$app/environment';

    let { data }: PageProps = $props();

	function onBackClicked() {
		if (browser) {
			window.history.back();
		}
	}
</script>

<Article id={data.id} />

<div class="relative flex flex-row justify-center items-center my-8">
	<button
		class="rounded border w-36 h-10 hover:border-[var(--primary-color)] cursor-pointer hover:bg-[var(--primary-color)] hover:text-white transition-[background-color,font-size] duration-500 hover:text-lg"
		onclick={onBackClicked}>返回
	</button>
</div>