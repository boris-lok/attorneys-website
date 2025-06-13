<script lang="ts">
    import type { PageProps } from './$types'
    import { MemberServices } from '$lib/services/member.service'
    import MemberEditor from '$lib/components/dashboard/MemberEditor.svelte'
    import type { ImageData } from '$lib/types'
    import Loading from '$lib/components/common/Loading.svelte'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let name = $state('')
    let description = $state('')
    let avatar: ImageData | undefined = $state(undefined)
    let seq = $state(0)

    async function fetchData() {
        const resp = await MemberServices.retrieve(data.id, 'zh')
        if (resp.error) {
            console.error(resp.message)
            return
        }

        name = resp.member?.data.name ?? ''
        description = resp.member?.data.description ?? ''
        avatar = resp.member?.avatar
        seq = resp.member?.seq ?? 0
    }

    $effect(() => {
        (async () => {
            isLoading = true
            await fetchData()
            isLoading = false
        })()
    })
</script>

{#if isLoading}
    <Loading />
{:else}
    <MemberEditor avatarData={avatar} {description} id={data.id} {name} {seq} />
{/if}
