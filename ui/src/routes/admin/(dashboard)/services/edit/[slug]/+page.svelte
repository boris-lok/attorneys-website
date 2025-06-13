<script lang="ts">
    import type { PageProps } from './$types'
    import { ServiceServices } from '$lib/services/service.service'
    import ServiceEditor from '$lib/components/dashboard/ServiceEditor.svelte'
    import Loading from '$lib/components/common/Loading.svelte'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let title = $state('')
    let content = $state('')
    let icon = $state<string | undefined>(undefined)

    async function fetchData() {
        const resp = await ServiceServices.retrieve(data.id, 'zh')
        if (resp.error) {
            console.error(resp.message)
            return
        }

        content = resp.service?.data.data ?? ''
        title = resp.service?.data.title ?? ''
        icon = resp.service?.data.icon
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
    <ServiceEditor data={content} id={data.id} {title} {icon} />
{/if}
