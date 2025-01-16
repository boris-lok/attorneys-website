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
                        }),
                    )
                    .subscribe({
                        next: () => goto('/admin/login'),
                    })
            },
        },
    ]
</script>

<NavigateBar {items}></NavigateBar>

<main>
    {@render children()}
</main>
