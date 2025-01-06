<script lang="ts">
	import IconifyIcon from '@iconify/svelte';

	let items = [
		{
			icon: 'line-md:home',
			name: 'Home'
		},
		{
			icon: 'stash:espiral-book-light',
			name: 'Services'
		},
		{
			icon: 'tabler:books',
			name: 'Articles'
		},
		{
			icon: 'tdesign:member-filled',
			name: 'Members'
		},
		{
			icon: 'hugeicons:contact-02',
			name: 'Contact us'
		}
	];

	let show = $state(false);

	function toggleMenu() {
		show = !show;
	}
</script>

<nav class="relative">
	<div class="relative flex flex-row gap-12 h-16 items-center px-8 bg-gray-400 overflow-hidden">
		<div class="relative flex flex-row items-center h-12 justify-between w-[100%]">
			<!-- Logo -->
			<div>
				<p class="text-lg">Logo</p>
			</div>

			<!-- Menu Icon -->
			<div class="sm:hidden w-6 h-6">
				<button onclick={toggleMenu}>
					<IconifyIcon icon="ri:menu-2-fill" width="24" height="24" />
				</button>
			</div>

			<!--			&lt;!&ndash; Navigate Item &ndash;&gt;-->
			<!--			<div class="">-->
			<!--				{#each items as item (item.name)}-->
			<!--					<NavigateItem label={item.name} icon={item.icon} />-->
			<!--				{/each}-->
			<!--			</div>-->
		</div>
	</div>
	<div class:show={show}
			 class="absolute h-0 transition-[height,opacity] backdrop-blur-sm top-16 w-screen overflow-y-scroll duration-300 opacity-0">
		<div class="grid grid-cols-2 h-[32rem] w-full justify-items-center items-center pt-6">
			{#each items as item (item.name)}
				<div
					class="relative flex flex-col gap-2 w-32 h-32 items-center p-4 rounded border-black hover:border-orange-500 group border-2 hover:border-4">
					<IconifyIcon icon={item.icon} width="48" height="48" class="group-hover:menu-icon-hover" />
					<p class="group-hover:text-orange-500 group-hover:font-bold">{item.name}</p>
				</div>
			{/each}
		</div>
	</div>
</nav>

<style lang="postcss">
    nav {
        .show {
            height: calc(100vh - 4rem);
            opacity: 1;
        }
    }
</style>
