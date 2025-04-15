<script lang="ts">
	import NavigateBar from '$lib/components/sidebar/NavigateBar.svelte';
	import Background from '$lib/components/Background.svelte';
	import type { NavigationItem } from '$lib/types';
	import Footer from '$lib/components/Footer.svelte';

	let { children } = $props();

	let items: NavigationItem[] = [
		{
			icon: 'lsicon:list-outline',
			name: '服務項目',
			url: '/services'
		},
		{
			icon: 'lineicons:books-2',
			name: '文章',
			url: '/articles'
		},
		{
			icon: 'tdesign:member',
			name: '專業團隊',
			url: '/members'
		},
		{
			icon: 'hugeicons:contact-02',
			name: '聯絡我們',
			url: '/contact_us'
		}
	];
</script>

<NavigateBar {items} rootUrl="/" />
<Background />

<main>
	<div class="relative min-h-[calc(100vh-64px-48px)]">
		{@render children()}
	</div>
</main>

<Footer />
