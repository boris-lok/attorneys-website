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
                    console.log(resp)
                    members = resp
                })
            )
            .subscribe({ error: console.error })
    })
</script>

{#if isLoading}
    <p>Loading...</p>
{:else}
    <div>
        <div class="relative my-4 flex flex-row justify-end px-2">
            <a href="/admin/members/edit">
                <Icon icon="gridicons:create" width="24" height="24" />
            </a>
        </div>
        <div class="relative flex md:flex-row flex-col gap-8">
            {#each members as member}
                <div class="flex flex-row w-fit px-4 py-4 items-center gap-2 rounded shadow">
                    {#if member.avatar}
                        <Image alt={member.name} image={member.avatar} size={48} />
                    {:else}
                        <IconifyIcon icon="radix-icons:avatar" class="w-12 h-12" />
                    {/if}
                    <p class="text-lg">{member.name}</p>
                    <div class="pl-12 py-2">
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
