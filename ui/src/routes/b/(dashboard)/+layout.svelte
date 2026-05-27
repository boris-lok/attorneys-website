<script lang="ts">
    import NavigateBar from '$lib/components/sidebar/NavigateBar.svelte'
    import type { NavigationItem } from '$lib/types'
    import { UserService } from '$lib/services/user.service'
    import { user } from '$lib/stores/user.store'
    import { goto } from '$app/navigation'

    let { children } = $props()

    let items: NavigationItem[] = [
        {
            icon: 'tabler:square-rounded-letter-w',
            name: '工作時時',
            url: '/b/case',
        },
        {
            icon: 'tabler:logout',
            name: '登出',
            onClick: async () => {
                const resp = await UserService.logout()
                if (resp.error) {
                    console.error(resp.message)
                }
                user.remove()
                await goto('/admin/login')
            },
        },
    ]
</script>

<NavigateBar {items} rootUrl="/b/dashboard"></NavigateBar>

<main>
    <div class="relative min-h-[calc(100vh-64px-48px)]">
        {@render children()}
    </div>
</main>
