<script lang="ts">
	import { Fa } from 'svelte-fa';
	import { faAddressBook, faBars, faBook, faHouse, faIdCard, faList, faXmark } from '@fortawesome/free-solid-svg-icons';
	import NavigateItem from '$lib/components/sidebar/NavigateItem.svelte';

	let items = [
		{
			icon: faHouse,
			name: 'Home'
		},
		{
			icon: faList,
			name: 'Services'
		},
		{
			icon: faBook,
			name: 'Articles'
		},
		{
			icon: faIdCard,
			name: 'Members'
		},
		{
			icon: faAddressBook,
			name: 'Contact us'
		}
	];

	// The status of dropdown menu.
	let show = $state(false);

	// handle the menu button clicked.
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
			<div class="sm:hidden w-6 h-6 relative flex items-center justify-center">
				<button onclick={toggleMenu}>
					<Fa class="text-xl" icon={show ? faXmark : faBars} />
				</button>
			</div>

			<!-- Top Bar Navigate Item -->
			<div class="relative flex flex-row sm:gap-4 md:gap-8 xl:gap-12 max-sm:hidden">
				{#each items as item (item.name)}
					<NavigateItem label={item.name} icon={item.icon} topBar={true} />
				{/each}
			</div>

		</div>
	</div>
	<!-- Dropdown Navigate Item -->
	<div class="absolute h-0 transition-[height,opacity] backdrop-blur-sm top-16 w-screen overflow-y-scroll duration-300 opacity-0 md:hidden"
			 class:show={show}>
		<div class="grid grid-cols-2 h-[32rem] w-full justify-items-center items-center pt-6">
			{#each items as item (item.name)}
				<NavigateItem label={item.name} icon={item.icon} topBar={false} />
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
