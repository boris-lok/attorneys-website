<script lang="ts">
    import Loading from '$lib/components/common/Loading.svelte'
    import ArticleCard from '$lib/components/ArticleCard.svelte'
    import type { CategoryData, Language, SimpleArticle } from '$lib/types'
    import { ArticleServices } from '$lib/services/article.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import { CategoryService } from '$lib/services/category.service'
    import CategorySelector from '$lib/components/CategorySelector.svelte'
    import PaginationComponent from '$lib/components/PaginationComponent.svelte'

    let isLoading = $state(false)
    let articles: SimpleArticle[] = $state([])
    let categories: CategoryData[] = $state([])
    let lang: Language = $state('zh')
    let page = $state(0)
    let selectedCategoryId = $state<string | null>(null)
    let pageSize = 10
    let totalPages = $state(0)

    function fetchArticlesObservable(lang: Language, categoryId: string | null, page: number, pageSize: number) {
        return ArticleServices.list(lang, categoryId, page, pageSize)
            .pipe(
                startWithTap(() => isLoading = true),
                finalize(() => isLoading = false),
                tap(resp => {
                    articles = resp.articles
                    totalPages = Math.floor(resp.total / pageSize) + (resp.total % pageSize > 0 ? 1 : 0)
                })
            )
    }

    function fetchCategoriesObservable(lang: Language) {
        return CategoryService
            .list(lang)
            .pipe(
                tap(resp => {
                    categories = resp
                })
            )
    }

    function onCategoryChanged(categoryId: string | null) {
        selectedCategoryId = categoryId
    }

    function onPageChanged(page: number) {
        page = page
    }

    $effect(() => {
        fetchCategoriesObservable(lang).subscribe({ error: console.error })
    })

    $effect(() => {
        fetchArticlesObservable(lang, selectedCategoryId, page, pageSize).subscribe({ error: console.error })
    })
</script>

{#if isLoading}
    <Loading />

{:else}
    <div class="px-4 lg:px-16 mb-8">
        <div class="mb-8">
            <h1 class="mb-8 px-4 pt-16 text-center text-4xl font-bold text-[var(--primary-color)] md:px-8 lg:px-16">
                文章</h1>
        </div>

        <div class="flex flex-col-reverse lg:flex-row gap-4 lg:gap-8">
            <div class="flex-4">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 pt-8">
                    {#each articles as article (article.id)}
                        <ArticleCard id={article.id} title={article.title} createdAt={article.createdAtString} />
                    {/each}
                </div>
            </div>

            <div class="flex-1">
                <CategorySelector categories={categories} onChanged={onCategoryChanged}
                                  selectedCategoryId={selectedCategoryId} />
            </div>
        </div>

        <div class="lg:mt-16 mt-8">
            <PaginationComponent totalPages={totalPages} onPageChanged={onPageChanged} currentPage={page} />
        </div>

    </div>

{/if}