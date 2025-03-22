<script lang="ts">
    import NavigateBar from '$lib/components/sidebar/NavigateBar.svelte'
    import type { NavigationItem } from '$lib/types'
    import { UserService } from '$lib/services/user.service'
    import { user } from '$lib/stores/user.store'
    import { finalize } from 'rxjs'
    import { goto } from '$app/navigation'

    let { children } = $props()

    let items: NavigationItem[] = [
        {
            icon: 'iconoir:home',
            name: 'Home',
            url: '/admin/home'
        },
        {
            icon: 'lsicon:list-outline',
            name: 'Services',
            url: '/admin/services'
        },
        {
            icon: 'lineicons:books-2',
            name: 'Articles',
            url: '/admin/articles'
        },
        {
            icon: 'tdesign:member',
            name: 'Members',
            url: '/admin/members'
        },
        {
            icon: 'hugeicons:contact-02',
            name: 'Contact us',
            url: '/admin/contact_us'
        },
        {
            icon: 'tabler:logout',
            name: 'logout',
            onClick: () => {
                // Call an API to remove the token, navigate to the login page
                // whatever success or failure. because we have removed the token
                // from the cookie. We won't use the previous token again.
                UserService.logout()
                    .pipe(
                        finalize(() => {
                            // Remove the token from the cookie
                            user.remove()
                        })
                    )
                    .subscribe({
                        next: () => goto('/admin/login')
                    })
            }
        }
    ]
</script>

<NavigateBar {items}></NavigateBar>

<main>
    <div class="relative max-w-[var(--max-screen-width)] mx-auto my-8">
        {@render children()}
    </div>
</main>
