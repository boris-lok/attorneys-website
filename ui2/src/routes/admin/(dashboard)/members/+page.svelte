<script lang="ts">
    import { MemberServices } from '$lib/services/member.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import type { Language, SimpleMember } from '$lib/types'

    let members: SimpleMember[] = $state([])
    let isLoading = $state(false)
    let lang: Language = 'zh'

    $effect(() => {
        MemberServices.list(lang)
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp) => {
                    console.log(resp)
                    members = resp
                }),
            )
            .subscribe({ error: console.error })
    })
</script>

{#if isLoading}
    <p>Loading...</p>
{:else}
    <div class="relative">
        {#each members as member}
            <p class="text-lg">{member.name}</p>
        {/each}
    </div>
{/if}
