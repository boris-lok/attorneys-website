<script lang="ts">
    import type { Language, SimpleMember } from '$lib/types'
    import { MemberServices } from '$lib/services/member.service'
    import Loading from '$lib/components/common/Loading.svelte'
    import IconifyIcon from '@iconify/svelte'
    import Icon from '@iconify/svelte'
    import Image from '$lib/components/common/Image.svelte'

    type InputProps = {
        isAdmin: boolean
    }

    let { isAdmin = false }: InputProps = $props()

    let members: SimpleMember[] = $state([])
    let isLoading = $state(false)
    let language: Language = 'zh'

    async function fetchMembers() {
        const resp = await MemberServices.list(language)
        if (resp.error) {
            console.log(resp.message)
            return []
        }

        return resp.members
    }

    $effect(() => {
        ;(async () => {
            isLoading = true
            members = await fetchMembers()
            isLoading = false
        })()
    })
</script>

{#if isLoading}
    <Loading />
{:else}
    {#each members as member (member.id)}
        <div
            class="flex h-36 w-full flex-row justify-between rounded px-4 py-4 shadow-md md:w-84 lg:w-96"
        >
            <a
                href="/members/{member.id}"
                class="flex flex-row items-center gap-4"
            >
                {#if member.avatar}
                    <div class="h-24 w-24">
                        <Image alt={member.name} image={member.avatar} />
                    </div>
                {:else}
                    <IconifyIcon icon="tabler:user-circle" class="h-24 w-24" />
                {/if}
                <p class="text-2xl">{member.name}</p>
            </a>
            {#if isAdmin}
                <a
                    href="/admin/members/edit/{member.id}"
                    class="inline-block h-6 w-6"
                >
                    <Icon icon="mingcute:edit-line" width="24" height="24" />
                </a>
            {/if}
        </div>
    {/each}
{/if}
