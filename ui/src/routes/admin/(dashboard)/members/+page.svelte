<script lang="ts">
    import { MemberServices } from '$lib/services/member.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import type { Language, SimpleMember } from '$lib/types'
    import Image from '$lib/components/common/Image.svelte'
    import IconifyIcon from '@iconify/svelte'
    import Icon from '@iconify/svelte'
    import Loading from '$lib/components/common/Loading.svelte'

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
                })
            )
            .subscribe({ error: console.error })
    })
</script>

{#if isLoading}
    <Loading />
{:else}
    <div>
        <div class="relative my-4 flex flex-row justify-end px-2">
            <a href="/admin/members/edit">
                <Icon height="24" icon="gridicons:create" width="24" />
            </a>
        </div>
        <div
            class="relative flex flex-col gap-8 px-4 md:flex-row mt-4 md:justify-center"
        >
            {#each members as member}
                <div
                    class="flex h-36 w-full flex-row items-center gap-6 rounded px-4 py-4 shadow-md md:w-84 lg:w-96"
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
                    <div class="py-2 pl-12">
                        <a href="/admin/members/edit/{member.id}">
                            <Icon
                                icon="mingcute:edit-line"
                                width="24"
                                height="24"
                            />
                        </a>
                    </div>
                </div>
            {/each}
        </div>
    </div>

{/if}