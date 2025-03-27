<script lang="ts">
    import type { PageProps } from './$types'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import { MemberServices } from '$lib/services/member.service'
    import MemberEditor from '$lib/components/dashboard/MemberEditor.svelte'
    import type { ImageData, MemberData } from '$lib/types'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let name = $state('')
    let description = $state('')
    let avatar: ImageData | undefined = $state(undefined)
    let seq = $state(0)

    function fetchData() {
        MemberServices.retrieve(data.id, 'zh')
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp: MemberData | null) => {
                    if (resp) {
                        name = resp.data.name ?? ''
                        description = resp.data.description ?? ''
                        avatar = resp.avatar
                        seq = resp.seq ?? 0
                    }
                })
            )
            .subscribe({
                error: console.error
            })
    }

    $effect(() => fetchData())
</script>

{#if isLoading}
    <p>Loading...</p>
{:else}
    <MemberEditor id={data.id} {name} {description} avatarData={avatar} {seq} />
{/if}
