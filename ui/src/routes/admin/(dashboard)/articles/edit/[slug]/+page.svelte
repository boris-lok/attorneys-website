<script lang="ts">
    import { startWithTap } from '$lib/utils'
    import { finalize, forkJoin, tap } from 'rxjs'
    import { ArticleServices } from '$lib/services/article.service'
    import ArticleEditor from '$lib/components/dashboard/ArticleEditor.svelte'
    import type { PageProps } from './$types'
    import Loading from '$lib/components/common/Loading.svelte'
    import { CategoryService } from '$lib/services/category.service'
    import type { CategoryData } from '$lib/types'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let title = $state('')
    let content = $state('')
    let categoryId: string | undefined = $state(undefined)
    let categories: CategoryData[] = $state([])

    function fetchData() {
        forkJoin({
            article: ArticleServices.retrieve(data.id, 'zh'),
            categories: CategoryService.list('zh')
        }).pipe(
            startWithTap(() => isLoading = true),
            finalize(() => isLoading = false),
            tap((resp) => {
                console.log(`${JSON.stringify(resp.article)}`)
                title = resp?.article?.data.title ?? ''
                content = resp?.article?.data.content ?? ''
                categoryId = resp?.article?.data.category_id
                categories = resp.categories
            })
        ).subscribe({
            error: console.error
        })
    }

    $effect(() => fetchData())
</script>

<Loading show={isLoading} />
<ArticleEditor {categories} {categoryId} {content} id={data.id} {title} />
