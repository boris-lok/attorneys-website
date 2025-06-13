<script lang="ts">
    import NavigateBar from '$lib/components/sidebar/NavigateBar.svelte'
    import type { NavigationItem } from '$lib/types'
    import { UserService } from '$lib/services/user.service'
    import { user } from '$lib/stores/user.store'
    import { goto } from '$app/navigation'

    let { children } = $props()

    let items: NavigationItem[] = [
        {
            icon: 'iconoir:home',
            name: '首頁',
            url: '/admin/home'
        },
        {
            icon: 'lsicon:list-outline',
            name: '服務',
            url: '/admin/services'
        },
        {
            icon: 'ph:books-light',
            name: '文章分類',
            url: '/admin/categories'
        },
        {
            icon: 'lineicons:books-2',
            name: '文章',
            url: '/admin/articles'
        },
        {
            icon: 'tdesign:member',
            name: '專業團隊',
            url: '/admin/members'
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
            }
        }
    ]
</script>

<NavigateBar {items} rootUrl="/admin/dashboard"></NavigateBar>

<main>
    <div class="relative min-h-[calc(100vh-64px-48px)]">
        {@render children()}
    </div>
</main>
