<script lang="ts">
    import { startWithTap } from '$lib/utils'
    import { BehaviorSubject, distinctUntilChanged, finalize, switchMap, tap } from 'rxjs'
    import type { ArticleData, Language, SimpleArticle } from '$lib/types'
    import Icon from '@iconify/svelte'
    import { ArticleServices } from '$lib/services/article.service'

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
        <div class="relative my-4 flex flex-row justify-end px-2">
            <a href="/admin/articles/edit">
                <Icon icon="gridicons:create" width="24" height="24" />
            </a>
        </div>
        <div class="relative flex flex-col gap-4">
            {#each articles as article}
                <div class="flex w-full flex-row overflow-clip rounded shadow">
                    <div class="flex-auto">
                        <p
                            class="px-8 py-2 text-lg font-bold text-[var(--primary-color)]"
                        >
                            {article.title}
                        </p>
                    </div>
                    <div class="px-2 py-2">
                        <a href="/admin/articles/edit/{article.id}">
                            <Icon
                                icon="mingcute:edit-line"
                                width="24"
                                height="24"
                            />
                        </a>
                    </div>
                </div>
            {/each}
        </div>
        <div
            class="relative my-4 flex flex-row items-center justify-center gap-4"
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
    </div>
{/if}
