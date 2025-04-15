<script lang="ts">
    import IconifyIcon from '@iconify/svelte';
    import Markdown from '@magidoc/plugin-svelte-marked';

    type InputProps = {
		title: string,
		icon?: string,
		content: string,
		active: boolean,
	}

	let { title, icon, content, active }: InputProps = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="group relative h-72 w-72 overflow-clip rounded-xl" class:active>
	<div
		class="relative h-full w-full rounded-xl border border-[var(--primary-color)] p-4 transition-[width,height]"
	>
		<div
			class="flex flex-col h-full w-full gap-2 items-center justify-center text-2xl font-bold text-[var(--primary-color)] max-sm:group-[.active]:opacity-20 group-hover:opacity-20"
		>
			{#if icon}
				<IconifyIcon icon={icon} class="h-12 w-12" />
			{/if}
			<p>{title}</p>
		</div>
	</div>

	<div
		class="absolute inset-0 opacity-0 bg-white group-hover:block max-sm:[&.active]:block max-sm:group-[.active]:opacity-80 group-hover:opacity-80"></div>
	<div
		class="prose absolute hidden p-4 group-hover:block max-sm:group-[.active]:block overflow-y-auto top-0 h-72 text-left"
	>
		<Markdown source={content} />
	</div>
</div>