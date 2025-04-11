<script lang="ts">
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import { ArticleServices } from '$lib/services/article.service'
    import ArticleEditor from '$lib/components/dashboard/ArticleEditor.svelte'
    import type { PageProps } from './$types'
    import Loading from '$lib/components/common/Loading.svelte'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let title = $state('')
    let content = $state('')

    function fetchData() {
        ArticleServices.retrieve(data.id, 'zh')
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp) => {
                    content = resp?.data.content ?? ''
                    title = resp?.data.title ?? ''
                })
            )
            .subscribe({
                error: console.error
            })
    }

    $effect(() => fetchData())
</script>

<Loading show={isLoading} />
<ArticleEditor id={data.id} {title} {content} />
