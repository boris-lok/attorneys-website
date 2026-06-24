<script lang="ts">
    import { goto } from '$app/navigation'
    import { page as pageStore } from '$app/state'
    import ArticleCard from '$lib/components/feature/ArticleCard.svelte'
    import CategorySelector from '$lib/components/feature/CategorySelector.svelte'
    import PaginationComponent from '$lib/components/shared/PaginationComponent.svelte'

    let { data } = $props()

    function updateParams(changes: Record<string, string | null>) {
        const params = new URLSearchParams(pageStore.url.searchParams)
        for (const [key, value] of Object.entries(changes)) {
            if (value === null) params.delete(key)
            else params.set(key, value)
        }
        goto(`?${params}`)
    }

    function onCategoryChanged(categoryId: string | null) {
        updateParams({ category: categoryId, page: '0' })
    }

    function onPageChanged(newPage: number) {
        updateParams({ page: String(newPage) })
    }
</script>

<div class="mb-8 px-4 lg:px-16">
    <div class="mb-8">
        <h1
            class="mb-8 px-4 pt-16 text-center text-4xl font-bold text-(--primary-color) md:px-8 lg:px-16"
        >
            文章
        </h1>
    </div>

    <div class="flex flex-col-reverse gap-4 lg:flex-row lg:gap-8">
        <div class="flex-4">
            <div class="grid grid-cols-1 gap-6 pt-8 md:grid-cols-2 lg:grid-cols-3">
                {#each data.articles as article (article.id)}
                    <ArticleCard
                        id={article.id}
                        title={article.title}
                        createdAt={article.createdAt}
                    />
                {/each}
            </div>
        </div>

        <div class="flex-1">
            <CategorySelector
                categories={[]}
                onChanged={onCategoryChanged}
                selectedCategoryId={data.categoryId}
            />
        </div>
    </div>

    <div class="mt-8 lg:mt-16">
        <PaginationComponent totalPages={data.totalPages} {onPageChanged} currentPage={data.page} />
    </div>
</div>
