<script lang="ts">
    import type { PageProps } from './$types'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import { ServiceServices } from '$lib/services/service.service'
    import ServiceEditor from '$lib/components/dashboard/ServiceEditor.svelte'
    import Loading from '$lib/components/common/Loading.svelte'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let title = $state('')
    let content = $state('')
    let icon = $state<string | undefined>(undefined)

    function fetchData() {
        ServiceServices.retrieve(data.id, 'zh')
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp) => {
                    content = resp?.data.data ?? ''
                    title = resp?.data.title ?? ''
                    icon = resp?.data.icon
                })
            )
            .subscribe({
                error: console.error
            })
    }

    $effect(() => fetchData())
</script>

{#if isLoading}
    <Loading />
{:else}
    <ServiceEditor data={content} id={data.id} {title} {icon} />
{/if}
