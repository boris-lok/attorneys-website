<script lang="ts">
    import ArticleEditor from '$lib/components/dashboard/ArticleEditor.svelte'
    import { CategoryService } from '$lib/services/category.service'
    import type { CategoryData } from '$lib/types'
    import Loading from '$lib/components/common/Loading.svelte'

    let isLoading = $state(false)
    let categories: CategoryData[] = $state([])

    async function fetchCategories() {
        const resp = await CategoryService.list('zh')
        if (resp.error) {
            console.error(resp.message)
            return
        }

        categories = resp.categories ?? []
    }

    $effect(() => {
        (async () => {
            isLoading = true
            await fetchCategories()
            isLoading = false
        })()
    })
</script>

{#if isLoading}
    <Loading />
{:else}
    <ArticleEditor {categories} />

{/if}