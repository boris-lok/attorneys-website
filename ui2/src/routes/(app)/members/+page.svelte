<script lang="ts">
    import { MemberServices } from '$lib/services/member.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import type { Language, SimpleMember } from '$lib/types'
    import Image from '$lib/components/common/Image.svelte'
    import IconifyIcon from '@iconify/svelte'
    import Icon from '@iconify/svelte'

    let members: SimpleMember[] = $state([])
    let isLoading = $state(false)
    let lang: Language = 'zh'

    $effect(() => {
        MemberServices.list(lang)
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp) => {
                    members = resp
                })
            )
            .subscribe({ error: console.error })
    })
</script>

{#if isLoading}
    <p>Loading...</p>
{:else}

    <div class="relative flex md:flex-row flex-col gap-8">
        {#each members as member}
            <a href="/members/{member.id}">
                <div class="flex flex-row w-full md:w-fit px-4 py-4 items-center gap-2 rounded shadow">
                    {#if member.avatar}
                        <div class="w-12 h-12">
                            <Image alt={member.name} image={member.avatar} />
                        </div>
                    {:else}
                        <IconifyIcon icon="radix-icons:avatar" class="w-12 h-12" />
                    {/if}
                    <p class="text-lg">{member.name}</p>
                </div>
            </a>
        {/each}
    </div>
{/if}
