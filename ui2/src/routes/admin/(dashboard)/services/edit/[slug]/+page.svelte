<script lang="ts">
    import type { PageProps } from './$types'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import { ServiceServices } from '$lib/services/service.service'
    import ServiceEditor from '$lib/components/dashboard/ServiceEditor.svelte'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let title = $state('')
    let content = $state('')

    function fetchData() {
        ServiceServices.retrieve(data.id, 'zh')
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp) => {
                    content = resp?.data.data ?? ''
                    title = resp?.data.title ?? ''
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
    <ServiceEditor id={data.id} {title} data={content} />
{/if}
