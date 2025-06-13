<script lang="ts">
    import type { PageProps } from './$types'
    import HomeEditor from '$lib/components/dashboard/HomeEditor.svelte'
    import { HomeServices } from '$lib/services/home.service'
    import Loading from '$lib/components/common/Loading.svelte'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let content = $state('')

    async function fetchData() {
        const resp = await HomeServices.retrieve(data.id, 'zh')
        if (resp.error) {
            console.error(resp.message)
            return
        }

        content = resp.home?.data.data ?? ''
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
    <HomeEditor data={content} id={data.id} />
{/if}
