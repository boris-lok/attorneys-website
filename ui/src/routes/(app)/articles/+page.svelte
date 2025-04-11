<script lang="ts">
    import { startWithTap } from '$lib/utils'
    import { BehaviorSubject, distinctUntilChanged, finalize, switchMap, tap } from 'rxjs'
    import type { Language, SimpleArticle } from '$lib/types'
    import { ArticleServices } from '$lib/services/article.service'
    import Article from '$lib/components/Article.svelte'
    import Loading from '$lib/components/common/Loading.svelte'

    let articles: SimpleArticle[] = $state([])
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

    // handles previous button clicked
    function onPreviousButtonClicked() {
        if (hasPreviousPage) {
            page = page - 1
            page$.next(page)
        }
    }

    // handles next button clicked
    function onNextButtonClicked() {
        if (hasNextPage) {
            page = page + 1
            page$.next(page)
        }
    }

    // fetch data for a specific language and page number
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

    // handles article clicked event
    function onArticleClicked(id: string) {
        articleID = id
    }

    // handles back button clicked event
    function onBackClicked() {
        articleID = ''
    }

    $effect(() => {
        // create an event listener to listen the page has been changed
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

<Loading show={isLoading} />
<div class="relative">

    <!-- Articles -->
    <div class="flex flex-col md:flex-row group justify-center"
         class:active={articleID !== ''}>

        <div
            class="relative px-8 w-full max-w-[64rem] group-[.active]:max-w-[32rem] transition-[max-width,padding] duration-500 ease-in-out"
        >

            <!-- Title -->
            <div class="my-8 md:my-16">
                <p
                    class="px-4 text-4xl font-bold text-[var(--primary-color)] md:px-8 lg:px-16 w-full text-center opacity-100 max-sm:group-[.active]:hidden md:group-[.active]:invisible"
                >
                    文章
                </p>
            </div>

            <div class="flex flex-row md:flex-col gap-6 overflow-auto py-4 w-full">
                {#each articles as article}
                    <button
                        class="relative flex flex-row gap-4 justify-between items-center rounded shadow h-12 w-full min-w-72 cursor-pointer [.active]:text-[var(--primary-color)] [.active]:border [.active]:border-[var(--primary-color)]"
                        class:active={articleID === article.id} onclick={() => onArticleClicked(article.id)}>
                            <span class="w-[calc(100%-120px)]">
                                <p class="text-xl text-left text-ellipsis overflow-hidden whitespace-nowrap pl-4 md:pl-8">{article.title}</p>
                            </span>
                        <span>
                                <p class="text-sm text-gray-500 border-l pl-4 pr-4 md:pr-8">{article.createdAtString}</p>
                            </span>
                    </button>
                {/each}
            </div>
        </div>


        {#if articleID !== ''}
            <div class="group-[.active]:flex-4 relative px-4">
                <Article id={articleID} onBackClicked={onBackClicked} />
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
