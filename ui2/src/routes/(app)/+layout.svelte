<script lang="ts">
    import NavigateBar from '$lib/components/sidebar/NavigateBar.svelte'
    import Background from '$lib/components/Background.svelte'
    import type { NavigationItem } from '$lib/types'

    let { children } = $props()

    let items: NavigationItem[] = [
        {
            icon: 'iconoir:home',
            name: 'Home',
            url: '/home',
        },
        {
            icon: 'lsicon:list-outline',
            name: 'Services',
            url: '/services',
        },
        {
            icon: 'lineicons:books-2',
            name: 'Articles',
            url: '/articles',
        },
        {
            icon: 'tdesign:member',
            name: 'Members',
            url: '/members',
        },
        {
            icon: 'hugeicons:contact-02',
            name: 'Contact us',
            url: '/contact_us',
        },
    ]
</script>

<NavigateBar {items} />
<Background />

<main>
    <div class="relative">
        {@render children()}
    </div>
</main>
