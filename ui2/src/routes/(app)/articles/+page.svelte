<script lang="ts">
    import { startWithTap } from '$lib/utils'
    import { BehaviorSubject, distinctUntilChanged, finalize, switchMap, tap } from 'rxjs'
    import type { ArticleData, Language } from '$lib/types'
    import { ArticleServices } from '$lib/services/article.service'
    import Article from '$lib/components/Article.svelte'
    import IconifyIcon from '@iconify/svelte'

    let articles: ArticleData[] = $state([])
    let isLoading = $state(false)
    let pageSize = 10
    let page = $state(0)
    // Observable for the current page number. We use BehaviorSubject to ensure that the page number is updated correctly when we navigate to a new page.
    const page$ = new BehaviorSubject(0)
    // The flag indicates that we have previous page
    let hasPreviousPage = $state(false)
    // The flag indicates that we have next page
    let hasNextPage = $state(false)
    // The ID of the article that is currently selected
    let articleID: string = $state('')

    function onPreviousButtonClicked() {
        page = page - 1
        page$.next(page)
    }

    function onNextButtonClicked() {
        page = page + 1
        page$.next(page)
    }

    function fetchData(lang: Language, page: number, pageSize: number) {
        return ArticleServices.list(lang, page, pageSize).pipe(
            startWithTap(() => (isLoading = true)),
            finalize(() => (isLoading = false)),
            tap((resp) => {
                console.log(resp)
                articles = resp.articles
                hasPreviousPage = page > 0
                hasNextPage = resp.total - (page + 1) * pageSize > 0
                console.log(hasNextPage, hasPreviousPage)
            })
        )
    }

    function onArticleClicked(id: string) {
        articleID = id
    }

    function onBackClicked() {
        articleID = ''
    }

    $effect(() => {
        const disposer = page$
            .pipe(
                distinctUntilChanged(),
                switchMap((page) => {
                    return fetchData('zh', page, pageSize)
                })
            )
            .subscribe({ error: console.error })

        return () => {
            disposer.unsubscribe()
        }
    })
</script>

{#if isLoading}
    <p>Loading...</p>
{:else}
    <div class="relative">
        <!-- Title -->
        {#if articleID === ''}
            <div class="my-8 md:my-16">
                <p
                    class="px-4 text-4xl font-bold text-[var(--primary-color)] md:px-8 lg:px-16 w-full text-center"
                >
                    文章
                </p>
            </div>
        {/if}

        <!-- Articles -->
        <div class="flex flex-col md:flex-row gap-8 group"
             class:active={articleID !== ''}>
            <div
                class="relative flex flex-row md:flex-col gap-4 md:max-w-[48rem] md:mx-auto group-[.active]:flex-1 group-[.active]:pt-24 overflow-auto py-8 px-4 w-full"
            >
                {#each articles as article}
                    <button
                        class="relative rounded shadow h-12 w-full cursor-pointer [.active]:text-[var(--primary-color)]"
                        class:active={articleID === article.id} onclick={() => onArticleClicked(article.id)}>
                        <p class="text-xl text-ellipsis w-full overflow-hidden whitespace-nowrap px-8">{article.data.title}</p>
                    </button>
                {/each}
            </div>


            {#if articleID !== ''}
                <div class="group-[.active]:flex-4 relative px-4">
                    <button class="cursor-pointer" onclick={onBackClicked}>
                        <IconifyIcon icon="lets-icons:refund-back-light"
                                     class="md:absolute h-8 w-8 top-8 md:top-16 md:left-8" />
                    </button>
                    <Article id={articleID} />
                </div>
            {/if}
        </div>


        <!-- The button group for controlling the page -->
        {#if articles.length > 0 && articleID === ''}
            <div
                class="relative my-8 flex flex-row items-center justify-center gap-4"
            >
                <button
                    class="cursor-pointer border-none bg-transparent underline [&.disabled]:cursor-default [&.disabled]:text-gray-500"
                    onclick={onPreviousButtonClicked}
                    class:disabled={!hasPreviousPage}
                    disabled={!hasPreviousPage}
                >Previous
                </button>
                <button
                    class="cursor-pointer border-none bg-transparent underline [&.disabled]:cursor-default [&.disabled]:text-gray-500"
                    onclick={onNextButtonClicked}
                    class:disabled={!hasNextPage}
                    disabled={!hasNextPage}
                >Next
                </button>
            </div>
        {/if}
    </div>
{/if}
