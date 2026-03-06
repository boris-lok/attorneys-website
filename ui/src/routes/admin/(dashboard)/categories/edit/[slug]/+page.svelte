<script lang="ts">
    import type { PageProps } from './$types'
    import CategoryEditor from '$lib/components/dashboard/CategoryEditor.svelte'
    import { CategoryService } from '$lib/services/category.service'
    import Loading from '$lib/components/common/Loading.svelte'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let icon: string | undefined = $state(undefined)
    let name = $state('')

    async function fetchData() {
        const resp = await CategoryService.retrieve(data.id, 'zh')
        if (resp.error) {
            console.error(resp.message)
            return
        }

        icon = resp.category?.data.icon
        name = resp.category?.data.name ?? ''
    }

    $effect(() => {
        ;(async () => {
            isLoading = true
            await fetchData()
            isLoading = false
        })()
    })
</script>

{#if isLoading}
    <Loading />
{:else}
    <CategoryEditor {icon} {name} />
{/if}
