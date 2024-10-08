<script lang="ts">
	import { onMount } from 'svelte';
	import { Home } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import type { HomeData } from '$lib/models/Home';
	import { t } from 'svelte-i18n';

	let isLoading = false;
	let data: HomeData | null = null;

	onMount(() => {
		Home.list('zh')
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => {
					data = e;
				})
			)
			.subscribe();
	});
</script>

<div class="home-wrapper">
	<h2 class="title">{$t('home')}</h2>
</div>

<style lang="scss">
  .home-wrapper {
    .title {
      font-size: 2rem;
      text-align: center;
      margin: 0;
      padding: 0;
			border-bottom: 1px solid $black;
    }
  }
</style>