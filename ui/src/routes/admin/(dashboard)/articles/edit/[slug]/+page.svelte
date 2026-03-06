<script lang="ts">
    import { ArticleServices } from '$lib/services/article.service'
    import ArticleEditor from '$lib/components/dashboard/ArticleEditor.svelte'
    import type { PageProps } from './$types'
    import Loading from '$lib/components/common/Loading.svelte'
    import type { CategoryData, Language } from '$lib/types'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let title = $state('')
    let content = $state('')
    let categoryId: string | undefined = $state(undefined)
    let categories: CategoryData[] = $state([])

    async function fetchArticle(id: string, language: Language = 'zh') {
        const resp = await ArticleServices.retrieve(id, language)

        if (resp.error) {
            console.error(resp.message)
            return
        }

        return {
            title: resp.article?.data.title ?? '',
            content: resp.article?.data.content ?? '',
            categoryId: resp.article?.data.category_id,
        }
        // forkJoin({
        //     article: ArticleServices.retrieve(data.id, 'zh'),
        //     categories: CategoryService.list('zh')
        // }).pipe(
        //     startWithTap(() => isLoading = true),
        //     finalize(() => isLoading = false),
        //     tap((resp) => {
        //         console.log(`${JSON.stringify(resp.article)}`)
        //         title = resp?.article?.data.title ?? ''
        //         content = resp?.article?.data.content ?? ''
        //         categoryId = resp?.article?.data.category_id
        //         categories = resp.categories
        //     })
        // ).subscribe({
        //     error: console.error
        // })
    }

    async function fetchCategories(language: Language = 'zh') {
        return []
    }

    $effect(() => {
        ;(async () => {
            isLoading = true
            const language = 'zh'
            try {
                let article = await fetchArticle(data.id, language)
                if (article) {
                    title = article.title
                    content = article.content
                    categoryId = article.categoryId
                }

                const resp = await fetchCategories(language)
                categories = resp
            } finally {
                isLoading = false
            }
        })()
    })
</script>

{#if isLoading}
    <Loading />
{:else}
    <ArticleEditor {categories} {categoryId} {content} id={data.id} {title} />
{/if}
