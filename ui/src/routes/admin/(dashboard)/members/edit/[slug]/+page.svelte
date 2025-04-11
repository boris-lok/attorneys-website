<script lang="ts">
    import type { PageProps } from './$types'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import { MemberServices } from '$lib/services/member.service'
    import MemberEditor from '$lib/components/dashboard/MemberEditor.svelte'
    import type { ImageData, MemberData } from '$lib/types'
    import Loading from '$lib/components/common/Loading.svelte'

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

<Loading show={isLoading} />
<MemberEditor avatarData={avatar} {description} id={data.id} {name} {seq} />
