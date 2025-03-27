<script lang="ts">
    import { MemberServices } from '$lib/services/member.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import type { Language, SimpleMember } from '$lib/types'
    import Image from '$lib/components/common/Image.svelte'
    import IconifyIcon from '@iconify/svelte'

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
    <div class="relative flex flex-col md:flex-row md:items-center">
        <div class="relative flex flex-col gap-4">
            <p
                class="px-4 text-4xl font-bold text-[var(--primary-color)] md:px-8 lg:px-16"
            >
                本所成員
            </p>
            <div
                class="relative flex flex-col gap-8 px-4 md:flex-row md:px-8 lg:px-16"
            >
                {#each members as member}
                    <a href="/members/{member.id}">
                        <div
                            class="flex h-48 w-full flex-col items-center justify-center gap-2 rounded px-4 py-4 shadow md:w-72"
                        >
                            {#if member.avatar}
                                <div class="h-24 w-24">
                                    <Image
                                        alt={member.name}
                                        image={member.avatar}
                                    />
                                </div>
                            {:else}
                                <IconifyIcon
                                    icon="radix-icons:avatar"
                                    class="h-24 w-24"
                                />
                            {/if}
                            <p class="text-2xl">{member.name}</p>
                        </div>
                    </a>
                {/each}
            </div>
        </div>

        <div
            class="hidden h-[calc(100vh-64px-144px)] min-h-[48rem] flex-1 justify-end overflow-clip md:flex"
        >
            <img
                class="w-[80%]"
                src="https://images.pexels.com/photos/7681417/pexels-photo-7681417.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=2"
                alt="members-bg"
            />
        </div>
    </div>
{/if}
