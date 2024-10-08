<script lang="ts">
	import { onMount } from 'svelte';
	import { Home } from '$lib/services';
	import { startWithTap, text_overflow } from '$lib/utils';
	import { finalize, tap } from 'rxjs';
	import { t } from 'svelte-i18n';
	import SvelteMarkdown from 'svelte-markdown';

	// isLoading is a flag that indicates we are loading a resource from API.
	let isLoading = false;
	// content is the actual content of the Markdown document.
	let content = '';
	// hasData is a flag that indicates whether
	// we have received a valid response from the API / doesn't have any data.
	let hasData = false;
	// The home resource ID
	let id = '';

	onMount(() => {
		Home.list('zh')
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false),
				tap(e => {
					hasData = e !== null;
					id = e?.id ?? '';
					content = e?.data.data ?? '';
					content = 'Tony Hawk\'s Underground is a 2003 skateboarding video game and the fifth entry in the Tony Hawk\'s series after Tony Hawk\'s Pro Skater 4. It was developed by Neversoft and published by Activision in 2003 for the GameCube, PlayStation 2, Xbox, and Game Boy Advance. In 2004, it was published for Microsoft Windows in Australia and New Zealand as a budget release.\n' +
						'\n' +
						'Underground is built upon the skateboarding formula of previous Tony Hawk\'s games: the player explores levels and completes goals while performing tricks. The game features a new focus on customization; the player, instead of selecting a professional skater, creates a custom character. Underground adds the ability for players to dismount their boards and explore on foot. The plot follows the player character and their friend Eric Sparrow as the two become professionals and grow apart.\n' +
						'\n' +
						'The game was developed with a theme of individuality which was manifested in the extensive character customization options, the presence of a narrative, and the product\'s characterization as an adventure game. Real world professional skateboarders contributed their experiences to the plot. Upon release, the game was a major critical and commercial success, with reviewers praising its wide appeal, soundtrack, customization, multiplayer, and storyline. The graphics and the controls for driving vehicles and walking were less well received. Underground\'s PlayStation 2 version had sold 2.11 million copies in the United States by December 2007. A sequel, Tony Hawk\'s Underground 2, followed in 2004. ';
					content = text_overflow(content, 180);
				})
			)
			.subscribe();
	});
</script>

<div class="home-wrapper">
	<h2 class="title">{$t('home')}</h2>
	<div class="home-content-wrapper">
		<div class="function-tools-wrapper">
			{#if hasData}
				<a href="/admin/home/edit/{id}">{$t('edit')}</a>
			{:else}
			{/if}
		</div>
		{#if isLoading}
			<p>{$t('loading')}...</p>
		{:else if content.length > 0}
			<SvelteMarkdown source={content} />
		{/if}
	</div>
</div>

<style lang="scss">
  .home-wrapper {
    .title {
      font-size: 2rem;
      text-align: center;
      margin: 0;
      border-bottom: 1px solid $black;
    }

    .home-content-wrapper {
      display: flex;
      flex-direction: column;
      width: 100%;
      height: 16rem;
    }
  }
</style>